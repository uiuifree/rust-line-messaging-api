use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageLocation {
    #[serde(rename = "type")]
    message_type: String,
    title: String,
    address: String,
    latitude: f32,
    longitude: f32,
}

impl LineMessageLocation {
    pub fn new(title: String, address: String, latitude: f32, longitude: f32) -> Self {
        Self {
            message_type: "location".to_string(),
            title,
            address,
            latitude,
            longitude,
        }
    }
}

impl LineMessageObject for LineMessageLocation {
    fn build(&self) -> Value {
        json!(self)
    }
}
