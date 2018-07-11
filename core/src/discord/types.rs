extern crate chrono;

pub type SequenceNumber = u64;
pub type UnixTimestamp = u64;

#[derive(Debug, Serialize)]
pub struct Snowflake(u64);

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub struct Channel {
    id: Snowflake,
    _type: ChannelType,
    guild_id: Option<Snowflake>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
    last_pin_timestamp: Option<Timestamp>,
}

pub struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    author: User,
    content: String,
    timestamp: Timestamp,
    edited_timestamp: Option<Timestamp>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<Snowflake>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    _type: MessageType,
    activity: Option<MessageActivity>,
    application: Option<MessageApplication>,
}

pub struct MessageActivity {
    _type: MessageActivityType,
    party_id: Option<String>,
}

pub struct MessageApplication {
    id: Snowflake,
    cover_image: String,
    description: String,
    icon: String,
    name: String,
}

pub struct Reaction {
    count: u32,
    me: bool,
    emoji: Emoji,
}

pub struct Overwrite {
    id: Snowflake,
    _type: String,
    allow: u32,
    deny: u32,
}

pub struct Embed {
    title: String,
    _type: String,
    description: String,
    url: String,
    timestamp: Timestamp,
    color: u32,
    footer: EmbedFooter,
    image: EmbedImage,
    thumbnail: EmbedThumbnail,
    video: EmbedVideo,
    provider: EmbedProvider,
    author: EmbedAuthor,
    fields: Vec<EmbedField>,
}

pub struct EmbedFooter {
    text: String,
    icon_url: String,
    proxy_icon_url: String,
}

pub struct EmbedImage {
    url: String,
    proxy_url: String,
    height: u32,
    width: u32,
}

pub struct EmbedThumbnail {
    url: String,
    proxy_url: String,
    height: u32,
    width: u32,
}

pub struct EmbedVideo {
    url: String,
    height: u32,
    width: u32,
}

pub struct EmbedProvider {
    name: String,
    url: String,
}

pub struct EmbedAuthor {
    name: String,
    url: String,
    icon_url: String,
    proxy_icon_url: String,
}

pub struct Attachment {
    id: Snowflake,
    filename: String,
    size: u32,
    url: String,
    proxy_url: String,
    height: Option<u32>,
    width: Option<u32>,
}

pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

pub enum ChannelType {
    GuildText,
    DM,
    GuildVoice,
    GourpDM,
    GuildCategory,
}

pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    GuildMemberJoin,
}

pub enum MessageActivityType {
    Join,
    Spectate,
    Listen,
    JoinRequest,
}

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

pub struct Connection {
    id: String,
    name: String,
    _type: String,
    revoked: bool,
    integrations: Vec<ServerIntegration>,
}

pub struct ServerIntegration {
    // ...
}

pub struct Role {
    // ...
}

pub struct Emoji {
    id: Option<Snowflake>,
    name: String,
    roles: Option<Vec<Role>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
}
