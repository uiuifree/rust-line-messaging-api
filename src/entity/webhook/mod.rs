mod event;
pub mod source;

use crate::webhook::source::{LineWebhookSource, LineWebhookSourceUser};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::LineUtil;
use crate::webhook::message::LineWebhookEventMessage;
pub use event::*;
// pub use source::*;

pub struct LineWebhook {}

impl LineWebhook {
    pub fn parse_str(input: &str) -> LineWebhookResponse {
        let input: Value = match serde_json::from_str(input) {
            Ok(input) => input,
            Err(e) => {
                println!("parse_str: {}", e);
                return LineWebhookResponse {
                    destination: "".to_string(),
                    events: vec![],
                };
            }
        };
        let destination = LineUtil::value_to_string("destination", &input).unwrap_or_default();
        let events = LineUtil::value_to_array("events", &input).unwrap_or_default();

        let events = events
            .into_iter()
            .map(|value| match serde_json::from_value(value.clone()) {
                Ok(event) => {
                    let mut event: LineWebhookEvent = event;
                    event.json = value.to_string();

                    let event_type = event.webhook_type.to_string();

                    event.object = match event_type.as_str() {
                        "message" => {
                            let message =
                                LineUtil::value_to_value("message", &value).unwrap_or_default();

                            LineWebhookEventObject::Message(LineWebhookEventMessage::from_value(
                                &message,
                            ))
                        }
                        "unsend" => LineWebhookEventObject::UnSend(
                            serde_json::from_value(
                                LineUtil::value_to_value("unsend", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "follow" => LineWebhookEventObject::Follow,
                        "unfollow" => LineWebhookEventObject::UnFollow,
                        "join" => LineWebhookEventObject::Join,
                        "leave" => LineWebhookEventObject::Leave,
                        "memberJoined" => LineWebhookEventObject::MemberJoined(
                            serde_json::from_value(
                                LineUtil::value_to_value("joined", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "memberLeft" => LineWebhookEventObject::MemberLeft(
                            serde_json::from_value(
                                LineUtil::value_to_value("left", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "postback" => LineWebhookEventObject::Postback(
                            serde_json::from_value(
                                LineUtil::value_to_value("postback", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "videoPlayComplete" => LineWebhookEventObject::VideoPlayComplete(
                            serde_json::from_value(
                                LineUtil::value_to_value("videoPlayComplete", &value)
                                    .unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "beacon" => LineWebhookEventObject::Beacon(
                            serde_json::from_value(
                                LineUtil::value_to_value("beacon", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "accountLink" => LineWebhookEventObject::AccountLink(
                            serde_json::from_value(
                                LineUtil::value_to_value("link", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        "things" => LineWebhookEventObject::Things(
                            serde_json::from_value(
                                LineUtil::value_to_value("things", &value).unwrap_or_default(),
                            )
                            .unwrap_or_default(),
                        ),
                        _ => LineWebhookEventObject::None,
                    };
                    event
                }
                Err(e) => {
                    println!("parse_str: {}", e);
                    LineWebhookEvent {
                        webhook_type: "".to_string(),
                        mode: "".to_string(),
                        timestamp: 0,
                        source: LineWebhookSource::User(LineWebhookSourceUser {
                            source_type: "".to_string(),
                            user_id: "".to_string(),
                        }),
                        webhook_event_id: "".to_string(),
                        delivery_context: LineDeliveryContext::default(),
                        reply_token: None,
                        object: LineWebhookEventObject::None,
                        json: "".to_string(),
                    }
                }
            })
            .collect();

        LineWebhookResponse {
            destination,
            events,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookResponse {
    destination: String,
    events: Vec<LineWebhookEvent>,
}

impl LineWebhookResponse {
    pub fn events(&self) -> Vec<LineWebhookEvent> {
        self.events.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookEvent {
    #[serde(rename = "type")]
    webhook_type: String,
    mode: String,
    timestamp: u64,
    source: LineWebhookSource,
    #[serde(rename = "webhookEventId")]
    webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    delivery_context: LineDeliveryContext,
    #[serde(rename = "replyToken")]
    reply_token: Option<String>,
    #[serde(skip)]
    object: LineWebhookEventObject,
    #[serde(skip)]
    json: String,
}

impl LineWebhookEvent {
    pub fn webhook_type(&self) -> &str {
        self.webhook_type.as_str()
    }
    pub fn mode(&self) -> &str {
        self.mode.as_str()
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn source(&self) -> &LineWebhookSource {
        &self.source
    }
    pub fn webhook_event_id(&self) -> &str {
        self.webhook_event_id.as_str()
    }
    pub fn delivery_context(&self) -> &LineDeliveryContext {
        &self.delivery_context
    }
    pub fn data(&self) -> &LineWebhookEventObject {
        &self.object
    }
    pub fn json(&self) -> &str {
        self.json.as_str()
    }
    pub fn reply_token(&self) -> Option<&str> {
        match &self.reply_token {
            Some(reply_token) => Some(reply_token.as_str()),
            None => None,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineDeliveryContext {
    #[serde(rename = "isRedelivery")]
    is_redelivery: bool,
}
