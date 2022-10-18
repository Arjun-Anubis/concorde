pub type Snowflake = String;
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub enum IntegerOrString {
    I(i32),
    S(String)
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Ready {
    v: i32,
    user: User,
    guilds: Vec<UnavailableGuild>,
    session_id: String,
    resume_gateway_url: String,
    shard: Option<Vec<i32>>,
    application: Application
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    author: User,
    content: String,
    timestamp: String,
    edited_timestamp: String,
    tts: bool,
    mention_eveyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    mention_channels: Option<ChannelMention>,
    attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id : Option<Snowflake>,
    // type: i32,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<Snowflake>,
    message_reference: Option<MessageReference>,
    flags: Option<i32>,
    // referenced_message: Option<&Message>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    components: Option<Vec<MessageComponent>>,
    sticker_items: Option<Vec<MessageStickerItem>>,
    stickers: Option<Sticker>,
    position: Option<i32>

}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    // type: i32,
    name: String
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Attachment {
    id: Snowflake,
    filename: String,
    description: Option<String>,
    content_type: Option<String>,
    size: i32,
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
    ephemeral: Option<bool>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Embed {
    title: Option<String>,
    // type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<i32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Vec<EmbedField>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Reaction {
    count: i32,
    me: bool,
    emoji: Emoji
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct MessageActivity {
    // type: i32,
    party_id: Option<String>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct MessageReference {
    message_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    guild_id: Option<Snowflake>,
    fail_if_not_exists: Option<bool>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct MessageInteraction {
    id: Snowflake,
    // type: InteractionType,
    name: String,
    user: User,
    // member: GuildMember,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct MessageComponent {
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct MessageStickerItem {
    id: Snowflake,
    name: String,
    format_type: i32
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Channel {
    // TODO
}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedFooter {}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedImage{}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedThumbnail{}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedVideo{}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedProvider{}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedAuthor{}
#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct EmbedField{}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<String>,
    accent_color: Option<i32>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<i32>,
    premium_type : Option<i32>,
    public_flags: Option<i32>
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UnavailableGuild {
    id: String,
    unavailable: bool
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Guild {
    id: String,
    name: String,
    icon: String,
    icon_hash: String,
    splash: String,
    discovery_splash: String,
    owner: bool,
    owner_id: String,
    permissions : String,
    region : String,
    afk_channel_id: String,
    afk_timeout: i32,
    widget_enabled: bool,
    widget_channel_id: String,
    verification_level: i32,
    default_message_notifications: i32,
    explicit_content_filter: i32,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<String>,
    mfa_level: i32,
    application_id: String,
    system_channel_id: String,
    system_channel_flags: i32,
    rules_channel_id: String,
    max_presences: i32,
    max_members: i32,
    vanity_url_code: String,
    description: String,
    banner: String,
    premium_tier: i32,
    premium_subscription_count: i32,
    preferred_locale: String,
    public_updates_channel_id: String,
    max_video_channel_users: i32,
    approximate_member_count: i32,
    approximate_presence_count: i32,
    welcome_screen: WelcomeScreen,
    nsfw_level: i32,
    stickers: Vec<Sticker>,
    premium_progress_bar_enabled: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Role {
    id: String,
    name: String,
    color: i32,
    hoist: bool,
    icon: Option<bool>,
    unicode_emoji: Option<String>,
    position: i32,
    permissions: String,
    managed: bool,
    mentionable: bool,
    tags: Option<Vec<RoleTags>>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RoleTags {
    bot_id: Option<String>,
    integration_id: Option<String>,
    premium_subscriber: Option<bool>
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<Role>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Sticker {
    id: String,
    pack_id: Option<String>,
    name: String,
    description: String,
    tags: Option<String>,
    asset: Option<String>,
    // type: i32,
    format_type: i32,
    guild_id: Option<String>,
    user: Option<User>,
    sort_value: Option<i32>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Vec<WelcomeScreenChannel>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Application {

}
