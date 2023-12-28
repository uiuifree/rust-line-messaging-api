use crate::util::LineUtil;
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum LineWebhookEventMessage {
    Text(LineWebhookEventMessageText),
    Video(LineWebhookEventMessageVideo),
    Image(LineWebhookEventMessageImage),
    Audio(LineWebhookEventMessageAudio),
    File(LineWebhookEventMessageFile),
    Location(LineWebhookEventMessageLocation),
    Sticker(LineWebhookEventMessageSticker),
    #[default]
    None,
}

impl LineWebhookEventMessage {
    pub fn from_value(message: &Value) -> LineWebhookEventMessage {
        let message_type = LineUtil::value_to_string("type", &message).unwrap_or_default();
        match message_type.as_str() {
            "text" => LineWebhookEventMessage::Text(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "video" => LineWebhookEventMessage::Video(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "image" => LineWebhookEventMessage::Image(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "audio" => LineWebhookEventMessage::Audio(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "file" => LineWebhookEventMessage::File(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "location" => LineWebhookEventMessage::Location(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            "sticker" => LineWebhookEventMessage::Sticker(
                serde_json::from_value(message.clone()).unwrap_or_default(),
            ),
            _ => LineWebhookEventMessage::None,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageText {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "quoteToken")]
    quote_token: String,
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "emojis")]
    emojis: Option<Vec<LineWebhookEventMessageTextEmoji>>,
    #[serde(rename = "mention")]
    mention: Option<LineWebhookEventMessageTextMention>,
    #[serde(rename = "quotedMessageId")]
    quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextEmoji {
    index: u32,
    length: u32,
    #[serde(rename = "productId")]
    product_id: String,
    #[serde(rename = "emojiId")]
    emoji_id: String,
    #[serde(rename = "quotedMessageId")]
    quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextMention {
    mentionees: Vec<LineWebhookEventMessageTextMentionMentionees>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextMentionMentionees {
    index: u32,
    length: u32,
    #[serde(rename = "type")]
    mention_type: String,
    #[serde(rename = "userId")]
    user_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageImage {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "quoteToken")]
    quote_token: String,
    #[serde(rename = "contentProvider")]
    content_provider: LineWebhookEventMessageContentProvider,
    #[serde(rename = "imageSet")]
    image_set: Option<LineWebhookEventMessageImageImageSet>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageImageImageSet {
    id: Option<String>,
    index: Option<u32>,
    total: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageVideo {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "quoteToken")]
    quote_token: String,
    #[serde(rename = "duration")]
    duration: Option<u32>,
    #[serde(rename = "contentProvider")]
    content_provider: LineWebhookEventMessageContentProvider,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageAudio {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "duration")]
    duration: Option<u32>,
    #[serde(rename = "contentProvider")]
    content_provider: LineWebhookEventMessageContentProvider,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageFile {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "fileName")]
    file_name: String,
    #[serde(rename = "fileSize")]
    file_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageLocation {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    title: Option<String>,
    address: Option<String>,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageSticker {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "quoteToken")]
    quote_token: String,
    #[serde(rename = "packageId")]
    package_id: String,
    #[serde(rename = "stickerId")]
    sticker_id: String,
    #[serde(rename = "stickerResourceType")]
    sticker_resource_type: String,
    keywords: Option<Vec<String>>,
    text: Option<String>,
    #[serde(rename = "quotedMessageId")]
    quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageContentProvider {
    #[serde(rename = "type")]
    content_provider_type: String,
    #[serde(rename = "originalContentUrl")]
    original_content_url: Option<String>,
    #[serde(rename = "previewImageUrl")]
    preview_image_url: Option<String>,
}
