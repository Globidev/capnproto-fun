#[derive(Debug)]
pub enum Event {
    Hello(Hello),
    Ready(Ready),
    Resumed(Resumed),
    InvalidSession(InvalidSession),
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleDelete(GuildRoleDelete),
    MessageCreate(MessageCreate),
    MessageUpdate(MessageUpdate),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    PresenceUpdate(PresenceUpdate),
    TypingStart(TypingStart),
    UserUpdate(UserUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    WebhooksUpdate(WebhooksUpdate),
}

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
pub struct DMChannel {

}

#[derive(Debug, Default, Deserialize)]
pub struct UnavailableGuild {

}
