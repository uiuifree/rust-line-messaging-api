use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageFlex {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "altText")]
    alt_text: String,
    contents: Value,
}

impl LineMessageFlex {
    pub fn new<T: ToString>(alt_text: T, contents: Value) -> Self {
        Self {
            message_type: "flex".to_string(),
            alt_text: alt_text.to_string(),
            contents,
        }
    }
}

impl LineMessageObject for LineMessageFlex {
    fn build(&self) -> Value {
        json!(self)
    }
}
