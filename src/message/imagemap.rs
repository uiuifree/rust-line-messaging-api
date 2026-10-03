use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{QuickReply, Sender};

/// An image with multiple tappable areas.
///
/// <https://developers.line.biz/en/reference/messaging-api/#imagemap-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "imagemap", rename_all = "camelCase")]
pub struct ImagemapMessage {
    /// Base URL of the image.
    pub base_url: String,
    /// Alternative text shown where the message cannot be displayed.
    pub alt_text: String,
    /// Size of the base image.
    pub base_size: ImagemapBaseSize,
    /// Actions performed when the areas of the image are tapped.
    pub actions: Vec<ImagemapAction>,
    /// Video played within the image.
    pub video: Option<ImagemapVideo>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl ImagemapMessage {
    /// Creates an imagemap message from the required `base_url`, `alt_text`, `base_size` and `actions`.
    pub fn new(
        base_url: impl Into<String>,
        alt_text: impl Into<String>,
        base_size: ImagemapBaseSize,
        actions: impl IntoIterator<Item = impl Into<ImagemapAction>>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            alt_text: alt_text.into(),
            base_size,
            actions: actions.into_iter().map(Into::into).collect(),
            video: None,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(ImagemapMessage {
    video: ImagemapVideo,
    quick_reply: QuickReply,
    sender: Sender,
});

/// Size of the base image in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapBaseSize {
    /// Width of the base image.
    pub width: u32,
    /// Height of the base image.
    pub height: u32,
}

impl ImagemapBaseSize {
    /// Creates a base size from `width` and `height`.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Tappable area of an imagemap, in pixels of the base image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapArea {
    /// Horizontal position of the top-left corner of the area.
    pub x: u32,
    /// Vertical position of the top-left corner of the area.
    pub y: u32,
    /// Width of the area.
    pub width: u32,
    /// Height of the area.
    pub height: u32,
}

impl ImagemapArea {
    /// Creates an area from its top-left corner (`x`, `y`), `width` and `height`.
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

tagged_enum! {
    /// An action performed when an imagemap area is tapped.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#imagemap-action-objects>
    pub enum ImagemapAction {
        "message" => Message(MessageImagemapAction),
        "uri" => Uri(UriImagemapAction),
        "clipboard" => Clipboard(ClipboardImagemapAction),
    }
}

/// Sends a text message when the area is tapped.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "message", rename_all = "camelCase")]
pub struct MessageImagemapAction {
    /// Tappable area.
    pub area: ImagemapArea,
    /// Text sent when the area is tapped.
    pub text: String,
    /// Label for the action.
    pub label: Option<String>,
}

impl MessageImagemapAction {
    /// Creates a message action that sends `text` when `area` is tapped.
    pub fn new(text: impl Into<String>, area: ImagemapArea) -> Self {
        Self {
            area,
            text: text.into(),
            label: None,
        }
    }
}

setters!(MessageImagemapAction { label: String });

/// Opens a URI when the area is tapped.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "uri", rename_all = "camelCase")]
pub struct UriImagemapAction {
    /// Tappable area.
    pub area: ImagemapArea,
    /// URI opened when the area is tapped.
    pub link_uri: String,
    /// Label for the action.
    pub label: Option<String>,
}

impl UriImagemapAction {
    /// Creates a URI action that opens `link_uri` when `area` is tapped.
    pub fn new(link_uri: impl Into<String>, area: ImagemapArea) -> Self {
        Self {
            area,
            link_uri: link_uri.into(),
            label: None,
        }
    }
}

setters!(UriImagemapAction { label: String });

/// Copies text to the clipboard when the area is tapped.
///
/// <https://developers.line.biz/en/reference/messaging-api/#imagemap-clipboard-action-object>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "clipboard", rename_all = "camelCase")]
pub struct ClipboardImagemapAction {
    /// Tappable area.
    pub area: ImagemapArea,
    /// Text that is copied to the clipboard (max 1000 characters).
    pub clipboard_text: String,
    /// Label for the action.
    pub label: Option<String>,
}

impl ClipboardImagemapAction {
    /// Creates a clipboard action that copies `clipboard_text` (1 to 1000 characters) when `area` is tapped.
    pub fn new(clipboard_text: impl Into<String>, area: ImagemapArea) -> Self {
        Self {
            area,
            clipboard_text: clipboard_text.into(),
            label: None,
        }
    }
}

setters!(ClipboardImagemapAction { label: String });

/// Video played within an imagemap.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapVideo {
    /// URL of the video file.
    pub original_content_url: Option<String>,
    /// URL of the preview image.
    pub preview_image_url: Option<String>,
    /// Area in which the video is played.
    pub area: Option<ImagemapArea>,
    /// Link shown after the video finishes playing.
    pub external_link: Option<ImagemapExternalLink>,
}

impl ImagemapVideo {
    /// Creates an imagemap video from `original_content_url`, `preview_image_url` and the `area` it plays in.
    pub fn new(
        original_content_url: impl Into<String>,
        preview_image_url: impl Into<String>,
        area: ImagemapArea,
    ) -> Self {
        Self::default()
            .original_content_url(original_content_url)
            .preview_image_url(preview_image_url)
            .area(area)
    }
}

setters!(ImagemapVideo {
    original_content_url: String,
    preview_image_url: String,
    area: ImagemapArea,
    external_link: ImagemapExternalLink,
});

/// Link shown after the imagemap video finishes playing.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapExternalLink {
    /// URI opened when the link is tapped.
    pub link_uri: Option<String>,
    /// Label of the link.
    pub label: Option<String>,
}

impl ImagemapExternalLink {
    /// Creates an external link from `link_uri` and `label`.
    pub fn new(link_uri: impl Into<String>, label: impl Into<String>) -> Self {
        Self::default().link_uri(link_uri).label(label)
    }
}

setters!(ImagemapExternalLink {
    link_uri: String,
    label: String,
});
