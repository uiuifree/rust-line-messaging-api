use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageImage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "originalContentUrl")]
    original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    preview_image_url: String,
}

impl LineMessageImage {
    pub fn new<T: ToString, U: ToString>(original_content_url: T, preview_image_url: U) -> Self {
        Self {
            message_type: "image".to_string(),
            original_content_url: original_content_url.to_string(),
            preview_image_url: preview_image_url.to_string(),
        }
    }
}

impl LineMessageObject for LineMessageImage {
    fn build(&self) -> Value {
        json!(self)
    }
}
