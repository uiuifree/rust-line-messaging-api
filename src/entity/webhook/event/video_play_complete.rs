use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize, Clone)]

pub struct LineWebhookEventVideoPlayComplete {
    #[serde(rename = "trackingId")]
    tracking_id: String,
}
