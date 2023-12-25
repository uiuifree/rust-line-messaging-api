use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageText {
    #[serde(rename = "type")]
    message_type: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emojis: Option<Vec<LineMessageEmoji>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quoteToken")]
    quote_token: Option<String>,
}

impl LineMessageText {
    pub fn new<T: ToString>(text: T) -> LineMessageText {
        LineMessageText {
            message_type: "text".to_string(),
            text: text.to_string(),
            emojis: None,
            quote_token: None,
        }
    }
    pub fn set_emojis(&mut self, emojis: Vec<LineMessageEmoji>) {
        self.emojis = Some(emojis);
    }
    pub fn set_quote_token(&mut self, quote_token: String) {
        self.quote_token = Some(quote_token);
    }
}
impl LineMessageObject for LineMessageText {
    fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "productId")]
    product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "emojiId")]
    emoji_id: Option<String>,
}
