use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize, Clone)]

pub struct LineWebhookEventBeacon {
    #[serde(rename = "type")]
    beacon_event_type: String,
    hwid: String,
}
