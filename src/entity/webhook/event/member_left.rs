use crate::webhook::source::LineWebhookSource;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize, Clone)]

pub struct LineWebhookEventMemberLeft {
    #[serde(rename = "members")]
    members: Vec<LineWebhookSource>,
}
