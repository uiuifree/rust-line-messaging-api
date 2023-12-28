use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventUnSend {
    #[serde(rename = "messageId")]
    message_id: String,
}
