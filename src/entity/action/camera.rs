use crate::action::LineActionObject;
use crate::message::LineMessageImageMapActionObject;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionCamera {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
}

impl LineMessageActionCamera {
    pub fn new<T: ToString>(label: T) -> Self {
        Self {
            message_type: "camera".to_string(),
            label: label.to_string(),
        }
    }
}
impl LineActionObject for LineMessageActionCamera {
    fn build(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
