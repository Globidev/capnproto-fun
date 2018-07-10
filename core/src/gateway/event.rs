extern crate serde;
extern crate serde_json;
extern crate serde_value;

use self::serde::{de, Serialize};
use self::serde_value::{Value as ASTValue, DeserializerError};
use self::serde_json::{
    from_str as decode,
    to_string as encode,
};

use std::fmt;

pub mod op {
    #[derive(Debug, Default, Serialize)]
    pub struct Identify {
        pub token: String,
        pub properties: IdentifyProperties,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub compress: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub large_threshold: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub shard: Option<(u32, u32)>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub presence: Option<StatusUpdate>
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct Hello {
        pub heartbeat_interval: u64,

        #[serde(rename = "_trace")]
        trace: Vec<String>
    }

    #[derive(Debug, Default, Serialize)]
    pub struct IdentifyProperties {
        #[serde(rename = "$os")]
        pub os: String,

        #[serde(rename = "$browser")]
        pub browser: String,

        #[serde(rename = "$device")]
        pub device: String
    }

    #[derive(Debug, Default, Serialize)]
    pub struct StatusUpdate {
        pub since: Option<u64>,
        pub game: Option<Activity>,
        pub status: String,
        pub afk: bool
    }

    #[derive(Debug, Default, Serialize)]
    pub struct Activity {

    }
}

mod event {

    #[derive(Debug, Default, Deserialize)]
    pub struct Hello {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct Ready {
        // gateway_version: u32,
        // user: User,
        // private_channels: Vec<DMChannel>,
        // guilds:Vec<UnavailableGuild>,
        // session_id: String,
        // trace: Vec<String>,
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct Resumed {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct InvalidSession {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct ChannelCreate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct ChannelUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct ChannelDelete {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct ChannelPinsUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildCreate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildDelete {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildBanAdd {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildBanRemove {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildEmojisUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildIntegrationsUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildMemberAdd {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildMemberRemove {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildMemberUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildMembersChunk {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildRoleCreate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildRoleUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct GuildRoleDelete {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageCreate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageDelete {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageDeleteBulk {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageReactionAdd {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageReactionRemove {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct MessageReactionRemoveAll {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct PresenceUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct TypingStart {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct UserUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct VoiceStateUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct VoiceServerUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct WebhooksUpdate {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct User {
        id: Snowflake,
        username: String,
        discriminator: String,
        avatar: Option<String>,
        bot: Option<bool>,
        mfa_enabled: Option<bool>,
        verified: Option<bool>,
        email: Option<String>,
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct DMChannel {

    }

    #[derive(Debug, Default, Deserialize)]
    pub struct UnavailableGuild {

    }

    pub type Snowflake = u64;
}

#[derive(Debug)]
pub enum Event {
    Hello(event::Hello),
    Ready(event::Ready),
    Resumed(event::Resumed),
    InvalidSession(event::InvalidSession),
    ChannelCreate(event::ChannelCreate),
    ChannelUpdate(event::ChannelUpdate),
    ChannelDelete(event::ChannelDelete),
    ChannelPinsUpdate(event::ChannelPinsUpdate),
    GuildCreate(event::GuildCreate),
    GuildUpdate(event::GuildUpdate),
    GuildDelete(event::GuildDelete),
    GuildBanAdd(event::GuildBanAdd),
    GuildBanRemove(event::GuildBanRemove),
    GuildEmojisUpdate(event::GuildEmojisUpdate),
    GuildIntegrationsUpdate(event::GuildIntegrationsUpdate),
    GuildMemberAdd(event::GuildMemberAdd),
    GuildMemberRemove(event::GuildMemberRemove),
    GuildMemberUpdate(event::GuildMemberUpdate),
    GuildMembersChunk(event::GuildMembersChunk),
    GuildRoleCreate(event::GuildRoleCreate),
    GuildRoleUpdate(event::GuildRoleUpdate),
    GuildRoleDelete(event::GuildRoleDelete),
    MessageCreate(event::MessageCreate),
    MessageUpdate(event::MessageUpdate),
    MessageDelete(event::MessageDelete),
    MessageDeleteBulk(event::MessageDeleteBulk),
    MessageReactionAdd(event::MessageReactionAdd),
    MessageReactionRemove(event::MessageReactionRemove),
    MessageReactionRemoveAll(event::MessageReactionRemoveAll),
    PresenceUpdate(event::PresenceUpdate),
    TypingStart(event::TypingStart),
    UserUpdate(event::UserUpdate),
    VoiceStateUpdate(event::VoiceStateUpdate),
    VoiceServerUpdate(event::VoiceServerUpdate),
    WebhooksUpdate(event::WebhooksUpdate),
}

#[derive(Debug)]
pub enum MessageIn {
    Dispatch(u32, Event),
    HeartBeat(u32),
    Reconnect,
    InvalidSession,
    Hello(op::Hello),
    HeartbeatAck,
}

#[derive(Debug)]
pub enum MessageOut {
    Identify(op::Identify),
    Heartbeat(Option<u32>),
}

pub fn from_raw_payload(raw_data: &str) -> Result<MessageIn, PayloadError> {
    decode(raw_data)
        .map_err(PayloadError::JsonError)
}

pub fn to_raw_payload(event: MessageOut) -> Result<String, PayloadError> {
    use self::MessageOut::*;

    match event {
        Heartbeat(heartbeat) => encode_event(1, heartbeat),
        Identify(identify)   => encode_event(2, identify),
    }
}

fn encode_event(op_code: u32, data: impl Serialize)
    -> Result<String, PayloadError>
{
    encode(&PayloadOut { op_code, data })
        .map_err(PayloadError::JsonError)
}

#[derive(Debug, Serialize)]
pub struct PayloadOut<T: Serialize> {
    #[serde(rename = "op")]
    op_code: u32,

    #[serde(rename = "d")]
    data: T,
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
    op_code: Option<u32>,
    data: Option<ASTValue>,
    sequence_number: Option<u32>,
    event_name: Option<String>
}

impl PayloadBuilder {
    fn new() -> Self { Self::default() }

    fn with_op_code(self, op_code: u32) -> Self {
        Self { op_code: Some(op_code), ..self }
    }

    fn with_data(self, data: ASTValue) -> Self {
        Self { data: Some(data), ..self }
    }

    fn with_sequence_number(self, sequence_number: Option<u32>) -> Self {
        Self { sequence_number, ..self }
    }

    fn with_event_name(self, event_name: Option<String>) -> Self {
        Self { event_name, ..self }
    }

    fn finish(self) -> Result<MessageIn, PayloadError> {
        use self::MessageIn::*;
        use self::PayloadError::*;
        use self::PayloadField::*;

        let op_code = self.op_code.ok_or_else(|| MissingField(OpCode))?;
        let data = self.data.ok_or_else(|| MissingField(Data))?;

        match op_code {
            0 => {
                let sequence_number = self.sequence_number
                    .ok_or_else(|| MissingField(SequenceNumber))?;
                let event_name = self.event_name
                    .ok_or_else(|| MissingField(EventName))?;
                let event = deserialize_event(&event_name, data)?;

                Ok(Dispatch(sequence_number, event))
            },
            1 => {
                let sequence_number = self.sequence_number
                    .ok_or_else(|| MissingField(SequenceNumber))?;

                Ok(HeartBeat(sequence_number))
            },
            7 => Ok(Reconnect),
            9 => Ok(InvalidSession),
            10 => Ok(Hello(data.deserialize_into().map_err(DataFormatError)?)),
            11 => Ok(HeartbeatAck),

            unknown => Err(UnknownOp { op_code: unknown, data })
        }
    }
}

fn deserialize_event(event_name: &str, data: ASTValue)
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

#[derive(Debug)]
pub enum PayloadError {
    JsonError(self::serde_json::Error),
    DataFormatError(DeserializerError),
    MissingField(PayloadField),
    UnknownOp { op_code: u32, data: ASTValue },
    UnknownEvent { name: String, data: ASTValue },
}

impl fmt::Display for PayloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::{Event, MessageIn::*, from_raw_payload};

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

        match from_raw_payload(data) {
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

        match from_raw_payload(data) {
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

        match from_raw_payload(data) {
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

        match from_raw_payload(data) {
            Ok(Dispatch(327, Event::PresenceUpdate(_))) => assert!(true),
            _                                           => assert!(false)
        }
    }
}
