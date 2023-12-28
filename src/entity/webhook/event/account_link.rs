use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize, Clone)]

pub struct LineWebhookEventAccountLink {
    result: String,
    nonce: String,
}
