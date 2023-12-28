use crate::webhook::account_link::LineWebhookEventAccountLink;
use crate::webhook::beacon::LineWebhookEventBeacon;
use crate::webhook::member_joined::LineWebhookEventMemberJoined;
use crate::webhook::member_left::LineWebhookEventMemberLeft;
use crate::webhook::message::LineWebhookEventMessage;
use crate::webhook::postback::LineWebhookEventPostback;
use crate::webhook::things::LineWebhookEventThings;
use crate::webhook::unsend::LineWebhookEventUnSend;
use crate::webhook::video_play_complete::LineWebhookEventVideoPlayComplete;
use serde_derive::{Deserialize, Serialize};

pub mod account_link;
pub mod beacon;
pub mod member_joined;
pub mod member_left;
pub mod message;
pub mod postback;
pub mod things;
pub mod unsend;
pub mod video_play_complete;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(untagged)]
pub enum LineWebhookEventObject {
    Message(LineWebhookEventMessage),
    UnSend(LineWebhookEventUnSend),
    Follow,
    UnFollow,
    Join,
    Leave,
    MemberJoined(LineWebhookEventMemberJoined),
    MemberLeft(LineWebhookEventMemberLeft),
    Postback(LineWebhookEventPostback),
    VideoPlayComplete(LineWebhookEventVideoPlayComplete),
    Beacon(LineWebhookEventBeacon),
    AccountLink(LineWebhookEventAccountLink),
    Things(LineWebhookEventThings),
    #[default]
    None,
}
