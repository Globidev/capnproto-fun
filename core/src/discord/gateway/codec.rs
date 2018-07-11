extern crate serde;
extern crate serde_value;

use self::serde::{de, Serialize, Serializer, ser::SerializeMap};
use self::serde_value::{Value as ASTValue, DeserializerError};

use super::protocol::{MessageIn, MessageOut, OpCode, op_codes};
use super::event::Event;

use discord::types::*;

use std::fmt;

pub trait Codec {
    type Encoded;
    type Error;

    fn encode(MessageOut) -> Result<Self::Encoded, Self::Error>;
    fn decode(Self::Encoded) -> Result<MessageIn, Self::Error>;
}

pub type JSONCodec = json::Codec;

mod json {
    extern crate serde_json;

    use super::{MessageIn, MessageOut};

    pub struct Codec;

    impl super::Codec for Codec {
        type Encoded = String; // Using Strings for now because of tungstenite
        type Error = self::serde_json::Error;

        fn encode(message: MessageOut) -> Result<Self::Encoded, Self::Error> {
            serde_json::to_string(&message)
        }

        fn decode(data: Self::Encoded) -> Result<MessageIn, Self::Error> {
            serde_json::from_str(&data)
        }
    }
}

impl Serialize for MessageOut {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        use self::MessageOut::*;
        use self::op_codes::*;

        match self {
            Heartbeat(heartbeat)         => encode_event(s, heartbeat, HEARTBEAT),
            Identify(identify)           => encode_event(s, identify,  IDENTIFY),
            UpdateStatus(status)         => encode_event(s, status,    STATUS_UPDATE),
            UpdateVoiceState(state)      => encode_event(s, state,     VOICE_STATUS_UPDATE),
            VoiceServerPing              => encode_event(s, (),        VOICE_SERVER_PING),
            Resume(resume)               => encode_event(s, resume,    RESUME),
            RequestGuildMembers(request) => encode_event(s, request,   REQUEST_GUILD_MEMBER),
        }
    }
}

impl<'de> de::Deserialize<'de> for MessageIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>
    {
        deserializer.deserialize_map(PayloadVisitor)
    }
}

struct PayloadVisitor;

#[derive(Debug, Deserialize)]
pub enum PayloadField {
    #[serde(rename = "op")]
    OpCode,

    #[serde(rename = "d")]
    Data,

    #[serde(rename = "s")]
    SequenceNumber,

    #[serde(rename = "t")]
    EventName
}

impl<'de> de::Visitor<'de> for PayloadVisitor {
    type Value = MessageIn;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Well formed payload")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        use self::PayloadField::*;

        let mut builder = PayloadBuilder::new();

        while let Some(key) = map.next_key()? {
            builder = match key {
                OpCode         => builder.with_op_code(map.next_value()?),
                Data           => builder.with_data(map.next_value()?),
                SequenceNumber => builder.with_sequence_number(map.next_value()?),
                EventName      => builder.with_event_name(map.next_value()?)
            }
        }

        builder.finish()
            .map_err(de::Error::custom)
    }
}

#[derive(Default)]
struct PayloadBuilder {
    op_code: Option<OpCode>,
    data: Option<ASTValue>,
    seq: Option<SequenceNumber>,
    event_name: Option<String>
}

impl PayloadBuilder {
    fn new() -> Self { Self::default() }

    fn with_op_code(self, op_code: OpCode) -> Self {
        Self { op_code: Some(op_code), ..self }
    }

    fn with_data(self, data: ASTValue) -> Self {
        Self { data: Some(data), ..self }
    }

    fn with_sequence_number(self, seq: Option<SequenceNumber>) -> Self {
        Self { seq, ..self }
    }

    fn with_event_name(self, event_name: Option<String>) -> Self {
        Self { event_name, ..self }
    }

    fn finish(self) -> Result<MessageIn, PayloadError> {
        use self::MessageIn::*;
        use self::PayloadError::*;
        use self::PayloadField::*;
        use self::op_codes::*;

        let op_code = self.op_code.ok_or_else(|| MissingField(OpCode))?;
        let data = self.data.ok_or_else(|| MissingField(Data))?;

        match op_code {
            DISPATCH => {
                let seq = self.seq
                    .ok_or_else(|| MissingField(SequenceNumber))?;
                let event_name = self.event_name
                    .ok_or_else(|| MissingField(EventName))?;
                let event = decode_event(&event_name, data)?;

                Ok(Dispatch(seq, event))
            },
            HEARTBEAT => {
                let seq = self.seq
                    .ok_or_else(|| MissingField(SequenceNumber))?;

                Ok(HeartBeat(seq))
            },
            RECONNECT => Ok(Reconnect),
            INVALID_SESSION => Ok(InvalidSession),
            HELLO => {
                Ok(Hello(data.deserialize_into().map_err(DataFormatError)?))
            },
            HEARTBEAT_ACK => Ok(HeartbeatAck),

            unknown => Err(UnknownOp { op_code: unknown, data })
        }
    }
}

fn encode_event<T, S>(serializer: S, data: T, op_code: OpCode)
    -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer
{
    match serializer.serialize_map(Some(2))? {
        mut payload => {
            payload.serialize_entry("op", &op_code)?;
            payload.serialize_entry("d", &data)?;
            payload.end()
        }
    }
}

fn decode_event(event_name: &str, data: ASTValue)
    -> Result<Event, PayloadError>
{
    use self::Event::*;
    use self::PayloadError::{DataFormatError, UnknownEvent};

    fn reify<C, T>(data: ASTValue, variant_constructor: C)
        -> Result<Event, PayloadError>
    where T: de::DeserializeOwned,
          C: FnOnce(T) -> Event
    {
        data.deserialize_into()
            .map(variant_constructor)
            .map_err(DataFormatError)
    }

    match event_name {
        "HELLO"                       => reify(data, Hello),
        "READY"                       => reify(data, Ready),
        "RESUMED"                     => reify(data, Resumed),
        "INVALID_SESSION"             => reify(data, InvalidSession),
        "CHANNEL_CREATE"              => reify(data, ChannelCreate),
        "CHANNEL_UPDATE"              => reify(data, ChannelUpdate),
        "CHANNEL_DELETE"              => reify(data, ChannelDelete),
        "CHANNEL_PINS_UPDATE"         => reify(data, ChannelPinsUpdate),
        "GUILD_CREATE"                => reify(data, GuildCreate),
        "GUILD_UPDATE"                => reify(data, GuildUpdate),
        "GUILD_DELETE"                => reify(data, GuildDelete),
        "GUILD_BAN_ADD"               => reify(data, GuildBanAdd),
        "GUILD_BAN_REMOVE"            => reify(data, GuildBanRemove),
        "GUILD_EMOJIS_UPDATE"         => reify(data, GuildEmojisUpdate),
        "GUILD_INTEGRATIONS_UPDATE"   => reify(data, GuildIntegrationsUpdate),
        "GUILD_MEMBER_ADD"            => reify(data, GuildMemberAdd),
        "GUILD_MEMBER_REMOVE"         => reify(data, GuildMemberRemove),
        "GUILD_MEMBER_UPDATE"         => reify(data, GuildMemberUpdate),
        "GUILD_MEMBERS_CHUNK"         => reify(data, GuildMembersChunk),
        "GUILD_ROLE_CREATE"           => reify(data, GuildRoleCreate),
        "GUILD_ROLE_UPDATE"           => reify(data, GuildRoleUpdate),
        "GUILD_ROLE_DELETE"           => reify(data, GuildRoleDelete),
        "MESSAGE_CREATE"              => reify(data, MessageCreate),
        "MESSAGE_UPDATE"              => reify(data, MessageUpdate),
        "MESSAGE_DELETE"              => reify(data, MessageDelete),
        "MESSAGE_DELETE_BULK"         => reify(data, MessageDeleteBulk),
        "MESSAGE_REACTION_ADD"        => reify(data, MessageReactionAdd),
        "MESSAGE_REACTION_REMOVE"     => reify(data, MessageReactionRemove),
        "MESSAGE_REACTION_REMOVE_ALL" => reify(data, MessageReactionRemoveAll),
        "PRESENCE_UPDATE"             => reify(data, PresenceUpdate),
        "TYPING_START"                => reify(data, TypingStart),
        "USER_UPDATE"                 => reify(data, UserUpdate),
        "VOICE_STATE_UPDATE"          => reify(data, VoiceStateUpdate),
        "VOICE_SERVER_UPDATE"         => reify(data, VoiceServerUpdate),
        "WEBHOOKS_UPDATE"             => reify(data, WebhooksUpdate),

        unknown => Err(UnknownEvent { name: String::from(unknown), data })
    }
}

#[derive(Debug)]
pub enum PayloadError {
    DataFormatError(DeserializerError),
    MissingField(PayloadField),
    UnknownOp { op_code: OpCode, data: ASTValue },
    UnknownEvent { name: String, data: ASTValue },
}

impl fmt::Display for PayloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::{Event, MessageIn::*, Codec};

    #[test]
    fn op10() {
        let data = r#"{
            "t": null,
            "s": null,
            "op": 10,
            "d": {
                "heartbeat_interval": 41250,
                "_trace": [
                    "gateway-prd-main-8v6p"
                ]
            }
        }"#;

        match super::JSONCodec::decode(String::from(data)) {
            Ok(Hello(_)) => assert!(true),
            _            => assert!(false)
        }
    }

    #[test]
    fn op11() {
        let data = r#"{
            "op":11,
            "d":null
        }"#;

        match super::JSONCodec::decode(String::from(data)) {
            Ok(HeartbeatAck) => assert!(true),
            _                => assert!(false)
        }
    }

    #[test]
    fn ready_event() {
        let data = r#"{
            "t": "READY",
            "s": 42,
            "op": 0,
            "d": {
                "v": 6,
                "user_settings": {},
                "user": {
                    "verified": true,
                    "username": "rust-bot",
                    "mfa_enabled": false,
                    "id": "464239709745577985",
                    "email": null,
                    "discriminator": "2051",
                    "bot": true,
                    "avatar": null
                },
                "session_id": "faa24d8d54d797b8176af61627f7cd73",
                "relationships": [],
                "private_channels": [],
                "presences": [],
                "guilds": [],
                "_trace": [
                    "gateway-prd-main-wd86",
                    "discord-sessions-prd-1-9"
                ]
            }
        }"#;

        match super::JSONCodec::decode(String::from(data)) {
            Ok(Dispatch(42, Event::Ready(_))) => assert!(true),
            _                                 => assert!(false)
        }
    }

    #[test]
    fn presence_update() {
        let data = r#"{
            "t": "PRESENCE_UPDATE",
            "s": 327,
            "op": 0,
            "d": {
                "user": {
                    "id": "85536304992886784"
                },
                "status": "online",
                "roles": [
                    "248726565431541761"
                ],
                "nick": null,
                "guild_id": "143032611814637568",
                "game": {
                    "type": 0,
                    "state": "In A Squad",
                    "session_id": "50f167dcd6c10868e8a21ea28c03e8b1",
                    "party": {
                        "size": [
                            2,
                            4
                        ],
                        "id": "33B998584CEB040B94C07899EA05DA71"
                    },
                    "name": "Fortnite",
                    "flags": 2,
                    "details": "Battle Royale - In Lobby",
                    "assets": {
                        "small_text": "Tier 100",
                        "small_image": "443127519386927104",
                        "large_image": "443127594037018634"
                    },
                    "application_id": "432980957394370572"
                }
            }
        }"#;

        match super::JSONCodec::decode(String::from(data)) {
            Ok(Dispatch(_, Event::PresenceUpdate(_))) => assert!(true),
            _                                         => assert!(false)
        }
    }
}
