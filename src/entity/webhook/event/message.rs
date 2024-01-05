use crate::util::LineUtil;
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
    pub fn text(&self) -> &str {
        match self {
            LineWebhookEventMessage::Text(v) => v.text.as_str(),
            LineWebhookEventMessage::Video(_) => "",
            LineWebhookEventMessage::Image(_) => "",
            LineWebhookEventMessage::Audio(_) => "",
            LineWebhookEventMessage::File(_) => "",
            LineWebhookEventMessage::Location(_) => "",
            LineWebhookEventMessage::Sticker(_) => "",
            LineWebhookEventMessage::None => "",
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageText {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "emojis")]
    pub emojis: Option<Vec<LineWebhookEventMessageTextEmoji>>,
    #[serde(rename = "mention")]
    pub mention: Option<LineWebhookEventMessageTextMention>,
    #[serde(rename = "quotedMessageId")]
    pub quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextEmoji {
    pub index: u32,
    pub length: u32,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
    #[serde(rename = "quotedMessageId")]
    pub quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextMention {
    pub mentionees: Vec<LineWebhookEventMessageTextMentionMentionees>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageTextMentionMentionees {
    pub index: u32,
    pub length: u32,
    #[serde(rename = "type")]
    pub mention_type: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageImage {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    #[serde(rename = "contentProvider")]
    pub content_provider: LineWebhookEventMessageContentProvider,
    #[serde(rename = "imageSet")]
    pub image_set: Option<LineWebhookEventMessageImageImageSet>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageImageImageSet {
    pub id: Option<String>,
    pub index: Option<u32>,
    pub total: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageVideo {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    #[serde(rename = "duration")]
    pub duration: Option<u32>,
    #[serde(rename = "contentProvider")]
    pub content_provider: LineWebhookEventMessageContentProvider,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageAudio {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "duration")]
    pub duration: Option<u32>,
    #[serde(rename = "contentProvider")]
    pub content_provider: LineWebhookEventMessageContentProvider,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageFile {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageLocation {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub title: Option<String>,
    pub address: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageSticker {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
    #[serde(rename = "stickerResourceType")]
    pub sticker_resource_type: String,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
    #[serde(rename = "quotedMessageId")]
    pub quoted_message_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventMessageContentProvider {
    #[serde(rename = "type")]
    pub content_provider_type: String,
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: Option<String>,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: Option<String>,
}
