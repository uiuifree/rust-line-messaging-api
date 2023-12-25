use crate::action::LineActionObject;
use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineActionUri {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
    uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "altUri")]
    alt_uri: Option<LineMessageActionUriAltUri>,
}

impl LineActionUri {
    pub fn new<T: ToString, U: ToString>(label: T, uri: U) -> Self {
        Self {
            message_type: "uri".to_string(),
            label: label.to_string(),
            uri: uri.to_string(),
            alt_uri: None,
        }
    }
    pub fn set_alt_uri(&mut self, alt_uri: LineMessageActionUriAltUri) {
        self.alt_uri = Some(alt_uri);
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionUriAltUri {
    desktop: String,
}

impl LineActionObject for LineActionUri {
    fn build(&self) -> serde_json::Value {
        json!(self)
    }
}
