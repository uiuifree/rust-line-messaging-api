use crate::webhook::source::LineWebhookSource;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LineWebhookEventMemberJoined {
    #[serde(rename = "members")]
    members: Vec<LineWebhookSource>,
}
