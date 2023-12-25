use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct LineMessageSticker {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "packageId")]
    package_id: String,
    #[serde(rename = "stickerId")]
    sticker_id: String,
    #[serde(rename = "quoteToken")]
    quote_token: Option<String>,
}

impl LineMessageSticker {
    pub fn new<T: ToString, S: ToString>(package_id: T, sticker_id: S) -> LineMessageSticker {
        LineMessageSticker {
            message_type: "sticker".to_string(),
            package_id: package_id.to_string(),
            sticker_id: sticker_id.to_string(),
            quote_token: None,
        }
    }
}

impl LineMessageObject for LineMessageSticker {
    fn build(&self) -> serde_json::Value {
        json!(self)
    }
}
