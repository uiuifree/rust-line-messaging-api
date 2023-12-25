use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionLocation {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
}
