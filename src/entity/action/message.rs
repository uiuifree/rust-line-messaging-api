use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionMessage {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
    text: String,
}

impl LineMessageActionMessage {
    pub fn new(label: String, text: String) -> LineMessageActionMessage {
        LineMessageActionMessage {
            message_type: "message".to_string(),
            label,
            text,
        }
    }
}
