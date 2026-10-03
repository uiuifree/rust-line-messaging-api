use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Action;

/// Quick reply buttons shown at the bottom of the chat screen.
///
/// <https://developers.line.biz/en/reference/messaging-api/#items-object>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickReply {
    /// Quick reply buttons (max 13).
    pub items: Option<Vec<QuickReplyItem>>,
}

impl QuickReply {
    /// Creates quick reply buttons from `items` (max 13).
    pub fn new(items: impl IntoIterator<Item = impl Into<QuickReplyItem>>) -> Self {
        Self::default().items(items)
    }
}

setters!(QuickReply {
    items: [QuickReplyItem]
});

/// A quick reply button. Serialized with `"type": "action"`.
///
/// <https://developers.line.biz/en/reference/messaging-api/#items-object>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "action", rename_all = "camelCase")]
pub struct QuickReplyItem {
    /// URL of the icon displayed at the beginning of the button (max 2000 characters).
    pub image_url: Option<String>,
    /// Action performed when the button is tapped.
    pub action: Option<Action>,
}

impl QuickReplyItem {
    /// Creates a quick reply button that performs `action`.
    pub fn new(action: impl Into<Action>) -> Self {
        Self::default().action(action)
    }
}

setters!(QuickReplyItem {
    image_url: String,
    action: Action,
});

/// Changes the icon and display name of the sender.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    /// Display name (max 20 characters). Certain words such as `LINE` may not be used.
    pub name: Option<String>,
    /// URL of the image displayed as the icon (max 2000 characters).
    pub icon_url: Option<String>,
}

setters!(Sender {
    name: String,
    icon_url: String,
});
