use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Action, QuickReply, Sender};

/// A message with a predefined layout.
///
/// <https://developers.line.biz/en/reference/messaging-api/#template-messages>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "template", rename_all = "camelCase")]
pub struct TemplateMessage {
    /// Alternative text shown where the message cannot be displayed.
    pub alt_text: String,
    /// Layout of the message.
    pub template: Template,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl TemplateMessage {
    /// Creates a template message from the required `alt_text` and `template`.
    pub fn new(alt_text: impl Into<String>, template: impl Into<Template>) -> Self {
        Self {
            alt_text: alt_text.into(),
            template: template.into(),
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(TemplateMessage {
    quick_reply: QuickReply,
    sender: Sender,
});

tagged_enum! {
    /// Layout of a [`TemplateMessage`].
    pub enum Template {
        "buttons" => Buttons(ButtonsTemplate),
        "confirm" => Confirm(ConfirmTemplate),
        "carousel" => Carousel(CarouselTemplate),
        "image_carousel" => ImageCarousel(ImageCarouselTemplate),
    }
}

/// Template with an image, title, text and multiple action buttons.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "buttons", rename_all = "camelCase")]
pub struct ButtonsTemplate {
    /// URL of the image.
    pub thumbnail_image_url: Option<String>,
    /// Aspect ratio of the image: `rectangle` (1.51:1) or `square` (1:1).
    pub image_aspect_ratio: Option<String>,
    /// Size of the image: `cover` (fills the image area, cropping what does not fit) or
    /// `contain` (shows the entire image, with a background in the unused area).
    pub image_size: Option<String>,
    /// Background color of the image.
    pub image_background_color: Option<String>,
    /// Title.
    pub title: Option<String>,
    /// Message text.
    pub text: String,
    /// Action performed when the image, title or text area is tapped.
    pub default_action: Option<Action>,
    /// Actions performed when the buttons are tapped.
    pub actions: Vec<Action>,
}

impl ButtonsTemplate {
    /// Creates a buttons template from the required `text` and `actions`.
    pub fn new(
        text: impl Into<String>,
        actions: impl IntoIterator<Item = impl Into<Action>>,
    ) -> Self {
        Self {
            thumbnail_image_url: None,
            image_aspect_ratio: None,
            image_size: None,
            image_background_color: None,
            title: None,
            text: text.into(),
            default_action: None,
            actions: actions.into_iter().map(Into::into).collect(),
        }
    }
}

setters!(ButtonsTemplate {
    thumbnail_image_url: String,
    image_aspect_ratio: String,
    image_size: String,
    image_background_color: String,
    title: String,
    default_action: Action,
});

/// Template with two action buttons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "confirm", rename_all = "camelCase")]
pub struct ConfirmTemplate {
    /// Message text.
    pub text: String,
    /// Actions performed when the buttons are tapped.
    pub actions: Vec<Action>,
}

impl ConfirmTemplate {
    /// Creates a confirm template from the required `text` and `actions`.
    pub fn new(
        text: impl Into<String>,
        actions: impl IntoIterator<Item = impl Into<Action>>,
    ) -> Self {
        Self {
            text: text.into(),
            actions: actions.into_iter().map(Into::into).collect(),
        }
    }
}

/// Template with multiple columns that can be cycled like a carousel.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "carousel", rename_all = "camelCase")]
pub struct CarouselTemplate {
    /// Columns of the carousel.
    pub columns: Vec<CarouselColumn>,
    /// Aspect ratio of the images.
    pub image_aspect_ratio: Option<String>,
    /// Size of the images.
    pub image_size: Option<String>,
}

impl CarouselTemplate {
    /// Creates a carousel template from the required `columns`.
    pub fn new(columns: impl IntoIterator<Item = impl Into<CarouselColumn>>) -> Self {
        Self {
            columns: columns.into_iter().map(Into::into).collect(),
            image_aspect_ratio: None,
            image_size: None,
        }
    }
}

setters!(CarouselTemplate {
    image_aspect_ratio: String,
    image_size: String,
});

/// Column of a [`CarouselTemplate`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselColumn {
    /// URL of the image.
    pub thumbnail_image_url: Option<String>,
    /// Background color of the image.
    pub image_background_color: Option<String>,
    /// Title.
    pub title: Option<String>,
    /// Message text.
    pub text: String,
    /// Action performed when the image, title or text area is tapped.
    pub default_action: Option<Action>,
    /// Actions performed when the buttons are tapped.
    pub actions: Vec<Action>,
}

impl CarouselColumn {
    /// Creates a carousel column from the required `text` and `actions`.
    pub fn new(
        text: impl Into<String>,
        actions: impl IntoIterator<Item = impl Into<Action>>,
    ) -> Self {
        Self {
            thumbnail_image_url: None,
            image_background_color: None,
            title: None,
            text: text.into(),
            default_action: None,
            actions: actions.into_iter().map(Into::into).collect(),
        }
    }
}

setters!(CarouselColumn {
    thumbnail_image_url: String,
    image_background_color: String,
    title: String,
    default_action: Action,
});

/// Template with multiple images that can be cycled like a carousel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "image_carousel", rename_all = "camelCase")]
pub struct ImageCarouselTemplate {
    /// Columns of the carousel.
    pub columns: Vec<ImageCarouselColumn>,
}

impl ImageCarouselTemplate {
    /// Creates an image carousel template from the required `columns`.
    pub fn new(columns: impl IntoIterator<Item = impl Into<ImageCarouselColumn>>) -> Self {
        Self {
            columns: columns.into_iter().map(Into::into).collect(),
        }
    }
}

/// Column of an [`ImageCarouselTemplate`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCarouselColumn {
    /// URL of the image.
    pub image_url: String,
    /// Action performed when the image is tapped.
    pub action: Action,
}

impl ImageCarouselColumn {
    /// Creates an image carousel column from the required `image_url` and `action`.
    pub fn new(image_url: impl Into<String>, action: impl Into<Action>) -> Self {
        Self {
            image_url: image_url.into(),
            action: action.into(),
        }
    }
}
