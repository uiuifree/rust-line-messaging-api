use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tagged_enum! {
    /// Content of a message in a message event.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#message-event>
    pub enum MessageContent {
        "text" => Text(TextMessageContent),
        "image" => Image(ImageMessageContent),
        "video" => Video(VideoMessageContent),
        "audio" => Audio(AudioMessageContent),
        "file" => File(FileMessageContent),
        "location" => Location(LocationMessageContent),
        "sticker" => Sticker(StickerMessageContent),
    }
}

impl MessageContent {
    /// Returns the message ID, or `None` for an unknown message type.
    pub fn id(&self) -> Option<&str> {
        match self {
            MessageContent::Text(m) => Some(&m.id),
            MessageContent::Image(m) => Some(&m.id),
            MessageContent::Video(m) => Some(&m.id),
            MessageContent::Audio(m) => Some(&m.id),
            MessageContent::File(m) => Some(&m.id),
            MessageContent::Location(m) => Some(&m.id),
            MessageContent::Sticker(m) => Some(&m.id),
            MessageContent::Unknown(_) => None,
        }
    }
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-text>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "text", rename_all = "camelCase")]
pub struct TextMessageContent {
    /// Message ID.
    pub id: String,
    /// Message text.
    pub text: String,
    /// LINE emojis contained in `text`.
    pub emojis: Option<Vec<Emoji>>,
    /// Mentions contained in `text`.
    pub mention: Option<Mention>,
    /// Quote token to quote this message.
    pub quote_token: String,
    /// Message ID of the quoted message, when this message quotes a past message.
    pub quoted_message_id: Option<String>,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// LINE emoji in a text message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#wh-text>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    /// Index of the first character of the emoji in `text`.
    pub index: u32,
    /// Length of the LINE emoji string.
    pub length: u32,
    /// Product ID of the LINE emoji set.
    pub product_id: String,
    /// ID of the LINE emoji inside the set.
    pub emoji_id: String,
}

/// Mentions in a text message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#wh-text>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mention {
    /// Mentioned targets (max 20).
    pub mentionees: Vec<Mentionee>,
}

tagged_enum! {
    /// Target of a mention.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#wh-text>
    pub enum Mentionee {
        /// A user is mentioned.
        "user" => User(UserMentionee),
        /// The entire group is mentioned.
        "all" => All(AllMentionee),
    }
}

/// Mention of a user.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "user", rename_all = "camelCase")]
pub struct UserMentionee {
    /// Index of the first character of the mention in `text`.
    pub index: u32,
    /// Length of the mention text.
    pub length: u32,
    /// User ID of the mentioned user, when the user consented to share their profile.
    pub user_id: Option<String>,
    /// Whether the mentioned user is the bot that receives the webhook.
    pub is_self: Option<bool>,
}

/// Mention of the entire group (`@All`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "all")]
pub struct AllMentionee {
    /// Index of the first character of the mention in `text`.
    pub index: u32,
    /// Length of the mention text.
    pub length: u32,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-image>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "image", rename_all = "camelCase")]
pub struct ImageMessageContent {
    /// Message ID.
    pub id: String,
    /// Provider of the image file.
    pub content_provider: ContentProvider,
    /// Set of images sent simultaneously.
    pub image_set: Option<ImageSet>,
    /// Quote token to quote this message.
    pub quote_token: String,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// Images sent simultaneously.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSet {
    /// Image set ID.
    pub id: String,
    /// 1-based index of the image in the set.
    pub index: Option<u32>,
    /// Total number of images sent simultaneously.
    pub total: Option<u32>,
}

/// Provider of a media file.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentProvider {
    /// Provider of the file.
    pub r#type: ContentProviderType,
    /// URL of the file. Only included when `type` is `external`.
    pub original_content_url: Option<String>,
    /// URL of the preview image. Only included when `type` is `external`.
    pub preview_image_url: Option<String>,
}

/// Provider type of a media file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentProviderType {
    /// Sent by the user; get it with the get content API.
    Line,
    /// Hosted outside LINE; see `original_content_url`.
    External,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-video>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "video", rename_all = "camelCase")]
pub struct VideoMessageContent {
    /// Message ID.
    pub id: String,
    /// Length of the video in milliseconds.
    pub duration: Option<u64>,
    /// Provider of the video file.
    pub content_provider: ContentProvider,
    /// Quote token to quote this message.
    pub quote_token: String,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-audio>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "audio", rename_all = "camelCase")]
pub struct AudioMessageContent {
    /// Message ID.
    pub id: String,
    /// Provider of the audio file.
    pub content_provider: ContentProvider,
    /// Length of the audio in milliseconds.
    pub duration: Option<u64>,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-file>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "file", rename_all = "camelCase")]
pub struct FileMessageContent {
    /// Message ID.
    pub id: String,
    /// File name.
    pub file_name: String,
    /// File size in bytes.
    pub file_size: u64,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-location>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "location", rename_all = "camelCase")]
pub struct LocationMessageContent {
    /// Message ID.
    pub id: String,
    /// Title.
    pub title: Option<String>,
    /// Address.
    pub address: Option<String>,
    /// Latitude.
    pub latitude: f64,
    /// Longitude.
    pub longitude: f64,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#wh-sticker>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "sticker", rename_all = "camelCase")]
pub struct StickerMessageContent {
    /// Message ID.
    pub id: String,
    /// Package ID.
    pub package_id: String,
    /// Sticker ID.
    pub sticker_id: String,
    /// Kind of the sticker.
    pub sticker_resource_type: StickerResourceType,
    /// Up to 15 keywords describing the sticker (experimental).
    pub keywords: Option<Vec<String>>,
    /// Text entered by the user. Only included for message stickers.
    pub text: Option<String>,
    /// Quote token to quote this message.
    pub quote_token: String,
    /// Message ID of the quoted message, when this message quotes a past message.
    pub quoted_message_id: Option<String>,
    /// Token used to mark the message as read.
    pub mark_as_read_token: Option<String>,
}

/// Kind of a sticker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StickerResourceType {
    /// `STATIC`: still image sticker.
    Static,
    /// `ANIMATION`: animated sticker.
    Animation,
    /// `SOUND`: sticker with sound.
    Sound,
    /// `ANIMATION_SOUND`: animated sticker with sound.
    AnimationSound,
    /// `POPUP`: pop-up or effect sticker.
    Popup,
    /// `POPUP_SOUND`: pop-up or effect sticker with sound.
    PopupSound,
    /// `CUSTOM`: custom sticker.
    Custom,
    /// `MESSAGE`: message sticker.
    Message,
    /// `NAME_TEXT`: custom sticker (name text).
    NameText,
    /// `PER_STICKER_TEXT`: per-sticker text sticker.
    PerStickerText,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}
