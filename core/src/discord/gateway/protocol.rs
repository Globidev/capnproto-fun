pub use self::input::{Message as MessageIn, *};
pub use self::output::{Message as MessageOut, *};

pub type OpCode = u16;

pub mod op_codes {
    use super::OpCode;

    pub const DISPATCH: OpCode             = 0;
    pub const HEARTBEAT: OpCode            = 1;
    pub const IDENTIFY: OpCode             = 2;
    pub const STATUS_UPDATE: OpCode        = 3;
    pub const VOICE_STATUS_UPDATE: OpCode  = 4;
    pub const VOICE_SERVER_PING: OpCode    = 5;
    pub const RESUME: OpCode               = 6;
    pub const RECONNECT: OpCode            = 7;
    pub const REQUEST_GUILD_MEMBER: OpCode = 8;
    pub const INVALID_SESSION: OpCode      = 9;
    pub const HELLO: OpCode                = 10;
    pub const HEARTBEAT_ACK: OpCode        = 11;
}

mod input {
    use discord::types::SequenceNumber;
    use discord::gateway::event::Event;

    #[derive(Debug)]
    pub enum Message {
        Dispatch(SequenceNumber, Event),
        HeartBeat(SequenceNumber),
        Reconnect,
        InvalidSession,
        Hello(Hello),
        HeartbeatAck,
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct Hello {
        pub heartbeat_interval: u64,

        #[serde(rename = "_trace")]
        trace: Vec<String>
    }
}

mod output {
    extern crate serde;

    use self::serde::ser::{Serialize, Serializer};
    use discord::types::*;

    #[derive(Debug)]
    pub enum Message {
        Heartbeat(Option<SequenceNumber>),
        Identify(Identify),
        UpdateStatus(StatusUpdate),
        UpdateVoiceState(VoiceStateUpdate),
        VoiceServerPing,
        Resume(Resume),
        RequestGuildMembers(GuildMembersRequest),
    }

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

    #[derive(Debug, Serialize)]
    pub struct Resume {
        pub token: String,
        pub session_id: String,
        pub seq: SequenceNumber
    }

    #[derive(Debug, Serialize)]
    pub struct GuildMembersRequest {
        pub guild_id: Snowflake,
        pub query: String,
        pub limit: u32,
    }

    #[derive(Debug, Serialize)]
    pub struct VoiceStateUpdate {
        pub guild_id: Snowflake,
        pub channel_id: Option<Snowflake>,
        pub self_mute: bool,
        pub self_deaf: bool,
    }

    #[derive(Debug, Serialize)]
    pub struct StatusUpdate {
        pub since: Option<UnixTimestamp>,
        pub game: Option<Activity>,
        pub status: String, // Maybe reify to a status type enum: https://discordapp.com/developers/docs/topics/gateway#update-status-status-types
        pub afk: bool
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

    #[derive(Debug, Serialize)]
    pub struct Activity {
        name: String,
        _type: ActivityType,

        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        timestamps: Option<ActivityTimestamps>,

        #[serde(skip_serializing_if = "Option::is_none")]
        application_id: Option<Snowflake>,

        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        party: Option<ActivityParty>,

        #[serde(skip_serializing_if = "Option::is_none")]
        assets: Option<ActivityAssets>,
    }

    #[derive(Debug)]
    pub enum ActivityType {
        Game,
        Streaming,
        Listening,
    }

    impl Serialize for ActivityType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use self::ActivityType::*;

            let value = match self {
                Game      => 0,
                Streaming => 1,
                Listening => 2,
            };

            serializer.serialize_u16(value)
        }
    }

    #[derive(Debug, Serialize)]
    struct ActivityTimestamps {
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<UnixTimestamp>,

        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<UnixTimestamp>,
    }

    #[derive(Debug, Serialize)]
    struct ActivityParty {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<(u32, u32)>,
    }

    #[derive(Debug, Serialize)]
    struct ActivityAssets {
        #[serde(skip_serializing_if = "Option::is_none")]
        large_image: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        large_text: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        small_image: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        small_text: Option<String>,
    }
}
