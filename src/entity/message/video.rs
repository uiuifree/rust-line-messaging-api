use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageVideo {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "originalContentUrl")]
    original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    preview_image_url: String,
    #[serde(rename = "trackingId")]
    tracking_id: Option<String>,
}

impl LineMessageVideo {
    pub fn new<T: ToString>(original_content_url: T, preview_image_url: T) -> Self {
        Self {
            message_type: "video".to_string(),
            original_content_url: original_content_url.to_string(),
            preview_image_url: preview_image_url.to_string(),
            tracking_id: None,
        }
    }
    pub fn set_tracking_id(&mut self, tracking_id: String) {
        self.tracking_id = Some(tracking_id);
    }
}

impl LineMessageObject for LineMessageVideo {
    fn build(&self) -> Value {
        json!(self)
    }
}
