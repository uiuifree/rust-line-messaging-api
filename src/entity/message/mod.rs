use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

mod audio;
mod flex;
mod image;
mod image_map;
mod location;
mod sticker;
mod template;
mod text;
mod video;

pub use audio::*;
pub use flex::*;
pub use image::*;
pub use image_map::*;
pub use location::*;
pub use sticker::*;
pub use template::*;
pub use text::*;
pub use video::*;

pub trait LineMessageObject {
    fn build(&self) -> Value;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LineMessagesBuilder {
    pub messages: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_aggregation_units: Option<Vec<String>>,
}

impl LineMessagesBuilder {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            notification_disabled: None,
            custom_aggregation_units: None,
        }
    }
    pub fn set_notification_disabled(&mut self, notification_disabled: bool) {
        self.notification_disabled = Some(notification_disabled);
    }
    pub fn set_custom_aggregation_units(&mut self, custom_aggregation_units: Vec<String>) {
        self.custom_aggregation_units = Some(custom_aggregation_units);
    }
    pub fn append(&mut self, message: impl LineMessageObject) {
        self.messages.push(message.build())
    }

    pub fn to_reply_request(&self, reply_token: String) -> LineApiMessageReplyRequest {
        LineApiMessageReplyRequest {
            reply_token,
            messages: self.messages.clone(),
            notification_disabled: self.notification_disabled,
        }
    }
    pub fn to_push_request<T: ToString>(&self, to: T) -> LineApiMessagePushRequest {
        LineApiMessagePushRequest {
            to: to.to_string(),
            messages: self.messages.clone(),
            notification_disabled: self.notification_disabled,
            custom_aggregation_units: self.custom_aggregation_units.clone(),
        }
    }
    pub fn to_multi_cast_request(&self, to: Vec<String>) -> LineApiMessageMulticastRequest {
        LineApiMessageMulticastRequest {
            to,
            messages: self.messages.clone(),
            notification_disabled: self.notification_disabled,
            custom_aggregation_units: self.custom_aggregation_units.clone(),
        }
    }
    pub fn to_broad_cast_request(&self) -> LineApiMessageBroadcastRequest {
        LineApiMessageBroadcastRequest {
            messages: self.messages.clone(),
            notification_disabled: self.notification_disabled,
        }
    }
    pub fn to_actions(&self) -> Vec<Value> {
        self.messages.clone()
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageReplyRequest {
    #[serde(rename = "notificationDisabled")]
    pub reply_token: String,
    // #[serde(flatten)]
    pub messages: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessagePushRequest {
    pub to: String,
    pub messages: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customAggregationUnits")]
    pub custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageMulticastRequest {
    pub to: Vec<String>,
    pub messages: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customAggregationUnits")]
    pub custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageBroadcastRequest {
    pub messages: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
}
