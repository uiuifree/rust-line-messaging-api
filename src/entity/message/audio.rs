use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageAudio {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "originalContentUrl")]
    original_content_url: String,
    duration: u32,
}

impl LineMessageAudio {
    pub fn new(original_content_url: String, duration: u32) -> Self {
        Self {
            message_type: "audio".to_string(),
            original_content_url,
            duration,
        }
    }
}

impl LineMessageObject for LineMessageAudio {
    fn build(&self) -> Value {
        json!(self)
    }
}
