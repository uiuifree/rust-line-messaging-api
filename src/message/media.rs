use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{QuickReply, Sender};

/// A sticker message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#sticker-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "sticker", rename_all = "camelCase")]
pub struct StickerMessage {
    /// Package ID of the sticker set.
    pub package_id: String,
    /// Sticker ID.
    pub sticker_id: String,
    /// Quote token of the message you want to quote.
    pub quote_token: Option<String>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl StickerMessage {
    /// Creates a sticker message from the required `package_id` and `sticker_id`.
    pub fn new(package_id: impl Into<String>, sticker_id: impl Into<String>) -> Self {
        Self {
            package_id: package_id.into(),
            sticker_id: sticker_id.into(),
            quote_token: None,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(StickerMessage {
    quote_token: String,
    quick_reply: QuickReply,
    sender: Sender,
});

/// An image message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#image-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "image", rename_all = "camelCase")]
pub struct ImageMessage {
    /// URL of the image file.
    pub original_content_url: String,
    /// URL of the preview image.
    pub preview_image_url: String,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl ImageMessage {
    /// Creates an image message from the required `original_content_url` and `preview_image_url`.
    pub fn new(
        original_content_url: impl Into<String>,
        preview_image_url: impl Into<String>,
    ) -> Self {
        Self {
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(ImageMessage {
    quick_reply: QuickReply,
    sender: Sender,
});

/// A video message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#video-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "video", rename_all = "camelCase")]
pub struct VideoMessage {
    /// URL of the video file.
    pub original_content_url: String,
    /// URL of the preview image.
    pub preview_image_url: String,
    /// ID used to identify the video in the video viewing complete event.
    pub tracking_id: Option<String>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl VideoMessage {
    /// Creates a video message from the required `original_content_url` and `preview_image_url`.
    pub fn new(
        original_content_url: impl Into<String>,
        preview_image_url: impl Into<String>,
    ) -> Self {
        Self {
            original_content_url: original_content_url.into(),
            preview_image_url: preview_image_url.into(),
            tracking_id: None,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(VideoMessage {
    tracking_id: String,
    quick_reply: QuickReply,
    sender: Sender,
});

/// An audio message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#audio-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "audio", rename_all = "camelCase")]
pub struct AudioMessage {
    /// URL of the audio file.
    pub original_content_url: String,
    /// Length of the audio file in milliseconds.
    pub duration: u64,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl AudioMessage {
    /// Creates an audio message from the required `original_content_url` and `duration` (milliseconds).
    pub fn new(original_content_url: impl Into<String>, duration: u64) -> Self {
        Self {
            original_content_url: original_content_url.into(),
            duration,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(AudioMessage {
    quick_reply: QuickReply,
    sender: Sender,
});

/// A location message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#location-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "location", rename_all = "camelCase")]
pub struct LocationMessage {
    /// Title of the location.
    pub title: String,
    /// Address of the location.
    pub address: String,
    /// Latitude.
    pub latitude: f64,
    /// Longitude.
    pub longitude: f64,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl LocationMessage {
    /// Creates a location message from the required `title`, `address`, `latitude` and `longitude`.
    pub fn new(
        title: impl Into<String>,
        address: impl Into<String>,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            title: title.into(),
            address: address.into(),
            latitude,
            longitude,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(LocationMessage {
    quick_reply: QuickReply,
    sender: Sender,
});

/// A coupon message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#coupon-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "coupon", rename_all = "camelCase")]
pub struct CouponMessage {
    /// Unique identifier of the coupon.
    pub coupon_id: String,
    /// Delivery route tag, usable for analysis in LINE Official Account Manager (max 30 characters).
    pub delivery_tag: Option<String>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl CouponMessage {
    /// Creates a coupon message with the required `coupon_id`.
    pub fn new(coupon_id: impl Into<String>) -> Self {
        Self {
            coupon_id: coupon_id.into(),
            delivery_tag: None,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(CouponMessage {
    delivery_tag: String,
    quick_reply: QuickReply,
    sender: Sender,
});
