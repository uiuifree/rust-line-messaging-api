use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Action, QuickReply, Sender};

/// A message with a customizable layout.
///
/// <https://developers.line.biz/en/reference/messaging-api/#flex-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "flex", rename_all = "camelCase")]
pub struct FlexMessage {
    /// Alternative text shown where the message cannot be displayed.
    pub alt_text: String,
    /// Flex Message container.
    pub contents: FlexContainer,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl FlexMessage {
    /// Creates a Flex Message from the required `alt_text` and `contents`.
    pub fn new(alt_text: impl Into<String>, contents: impl Into<FlexContainer>) -> Self {
        Self {
            alt_text: alt_text.into(),
            contents: contents.into(),
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(FlexMessage {
    quick_reply: QuickReply,
    sender: Sender,
});

tagged_enum! {
    /// Top-level structure of a [`FlexMessage`]: a bubble or a carousel of bubbles.
    ///
    /// JSON from the Flex Message Simulator decodes with `serde_json::from_str::<FlexContainer>`.
    pub enum FlexContainer {
        "bubble" => Bubble(FlexBubble),
        "carousel" => Carousel(FlexCarousel),
    }
}

/// A single message bubble.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "bubble", rename_all = "camelCase")]
pub struct FlexBubble {
    /// Text directionality and the direction of placement of components in horizontal boxes.
    pub direction: Option<FlexDirection>,
    /// Styles of each block.
    pub styles: Option<FlexBubbleStyles>,
    /// Header block.
    pub header: Option<FlexBox>,
    /// Hero block.
    pub hero: Option<FlexComponent>,
    /// Body block.
    pub body: Option<FlexBox>,
    /// Footer block.
    pub footer: Option<FlexBox>,
    /// Size of the bubble.
    pub size: Option<FlexBubbleSize>,
    /// Action performed when the bubble is tapped.
    pub action: Option<Action>,
}

setters!(FlexBubble {
    direction: FlexDirection,
    styles: FlexBubbleStyles,
    header: FlexBox,
    hero: FlexComponent,
    body: FlexBox,
    footer: FlexBox,
    size: FlexBubbleSize,
    action: Action,
});

/// Text direction of a [`FlexBubble`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexDirection {
    /// `ltr`: left to right.
    Ltr,
    /// `rtl`: right to left.
    Rtl,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Size of a [`FlexBubble`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexBubbleSize {
    /// `nano`
    Nano,
    /// `micro`
    Micro,
    /// `deca`
    Deca,
    /// `hecto`
    Hecto,
    /// `kilo`
    Kilo,
    /// `mega`
    Mega,
    /// `giga`
    Giga,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Styles of each block of a [`FlexBubble`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBubbleStyles {
    /// Style of the header block.
    pub header: Option<FlexBlockStyle>,
    /// Style of the hero block.
    pub hero: Option<FlexBlockStyle>,
    /// Style of the body block.
    pub body: Option<FlexBlockStyle>,
    /// Style of the footer block.
    pub footer: Option<FlexBlockStyle>,
}

setters!(FlexBubbleStyles {
    header: FlexBlockStyle,
    hero: FlexBlockStyle,
    body: FlexBlockStyle,
    footer: FlexBlockStyle,
});

/// Style of one block of a [`FlexBubble`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexBlockStyle {
    /// Background color of the block.
    pub background_color: Option<String>,
    /// Whether a separator is placed above the block.
    pub separator: Option<bool>,
    /// Color of the separator.
    pub separator_color: Option<String>,
}

setters!(FlexBlockStyle {
    background_color: String,
    separator: bool,
    separator_color: String,
});

/// Multiple bubbles that can be cycled horizontally.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "carousel", rename_all = "camelCase")]
pub struct FlexCarousel {
    /// Bubbles in the carousel.
    pub contents: Vec<FlexBubble>,
}

impl FlexCarousel {
    /// Creates a carousel from the required bubbles in `contents`.
    pub fn new(contents: impl IntoIterator<Item = impl Into<FlexBubble>>) -> Self {
        Self {
            contents: contents.into_iter().map(Into::into).collect(),
        }
    }
}

tagged_enum! {
    /// A component of a Flex Message bubble.
    pub enum FlexComponent {
        "box" => Box(FlexBox),
        "button" => Button(FlexButton),
        "image" => Image(FlexImage),
        "video" => Video(FlexVideo),
        "icon" => Icon(FlexIcon),
        "text" => Text(FlexText),
        "span" => Span(FlexSpan),
        "separator" => Separator(FlexSeparator),
        "filler" => Filler(FlexFiller),
    }
}

/// Lays out child components horizontally, vertically or along a baseline.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "box", rename_all = "camelCase")]
pub struct FlexBox {
    /// Placement style of the child components.
    pub layout: FlexBoxLayout,
    /// The ratio of the width or height of this component within the parent box.
    pub flex: Option<u32>,
    /// Child components.
    pub contents: Vec<FlexComponent>,
    /// Minimum space between the child components: pixels or a keyword (`none`, `xs` to `xxl`).
    pub spacing: Option<String>,
    /// The minimum amount of space to include before this component in its parent container: pixels or a keyword (`none`, `xs` to `xxl`).
    pub margin: Option<String>,
    /// Reference for the offsets: `relative` (the previous box; default) or `absolute` (the top left of the parent element).
    pub position: Option<FlexPosition>,
    /// Offset from the top: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_top: Option<String>,
    /// Offset from the bottom: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_bottom: Option<String>,
    /// Offset from the start: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_start: Option<String>,
    /// Offset from the end: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_end: Option<String>,
    /// Background color of the box.
    pub background_color: Option<String>,
    /// Color of the box border.
    pub border_color: Option<String>,
    /// Width of the box border: pixels or a keyword (`none`, `light`, `normal`, `medium`, `semi-bold`, `bold`).
    pub border_width: Option<String>,
    /// Radius of the rounded corners: pixels or a keyword (`none`, `xs` to `xxl`).
    pub corner_radius: Option<String>,
    /// Width of the box.
    pub width: Option<String>,
    /// Maximum width of the box.
    pub max_width: Option<String>,
    /// Height of the box.
    pub height: Option<String>,
    /// Maximum height of the box.
    pub max_height: Option<String>,
    /// Padding on all sides: pixels, a percentage of the parent box width, or a keyword (`none`, `xs` to `xxl`).
    pub padding_all: Option<String>,
    /// Top padding: pixels, a percentage of the parent box width, or a keyword (`none`, `xs` to `xxl`).
    pub padding_top: Option<String>,
    /// Bottom padding: pixels, a percentage of the parent box width, or a keyword (`none`, `xs` to `xxl`).
    pub padding_bottom: Option<String>,
    /// Start padding: pixels, a percentage of the parent box width, or a keyword (`none`, `xs` to `xxl`).
    pub padding_start: Option<String>,
    /// End padding: pixels, a percentage of the parent box width, or a keyword (`none`, `xs` to `xxl`).
    pub padding_end: Option<String>,
    /// Action performed when this component is tapped.
    pub action: Option<Action>,
    /// Placement of the child components along the main axis.
    pub justify_content: Option<FlexJustifyContent>,
    /// Placement of the child components along the cross axis.
    pub align_items: Option<FlexAlignItems>,
    /// Background of the box.
    pub background: Option<FlexBoxBackground>,
}

impl FlexBox {
    /// Creates a box from the required `layout` and child `contents`.
    pub fn new(
        layout: FlexBoxLayout,
        contents: impl IntoIterator<Item = impl Into<FlexComponent>>,
    ) -> Self {
        Self {
            layout,
            flex: None,
            contents: contents.into_iter().map(Into::into).collect(),
            spacing: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            background_color: None,
            border_color: None,
            border_width: None,
            corner_radius: None,
            width: None,
            max_width: None,
            height: None,
            max_height: None,
            padding_all: None,
            padding_top: None,
            padding_bottom: None,
            padding_start: None,
            padding_end: None,
            action: None,
            justify_content: None,
            align_items: None,
            background: None,
        }
    }
}

setters!(FlexBox {
    flex: u32,
    spacing: String,
    margin: String,
    position: FlexPosition,
    offset_top: String,
    offset_bottom: String,
    offset_start: String,
    offset_end: String,
    background_color: String,
    border_color: String,
    border_width: String,
    corner_radius: String,
    width: String,
    max_width: String,
    height: String,
    max_height: String,
    padding_all: String,
    padding_top: String,
    padding_bottom: String,
    padding_start: String,
    padding_end: String,
    action: Action,
    justify_content: FlexJustifyContent,
    align_items: FlexAlignItems,
    background: FlexBoxBackground,
});

/// `layout` of a [`FlexBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexBoxLayout {
    /// `horizontal`: child components are placed horizontally.
    Horizontal,
    /// `vertical`: child components are placed vertically.
    Vertical,
    /// `baseline`: child components are placed horizontally, aligned on their baselines.
    Baseline,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `position` of a Flex component: the reference for its offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexPosition {
    /// `relative`: offsets are relative to the previous box.
    Relative,
    /// `absolute`: offsets are relative to the top left of the parent element.
    Absolute,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `justifyContent` of a [`FlexBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexJustifyContent {
    /// `center`
    Center,
    /// `flex-start`
    FlexStart,
    /// `flex-end`
    FlexEnd,
    /// `space-between`
    SpaceBetween,
    /// `space-around`
    SpaceAround,
    /// `space-evenly`
    SpaceEvenly,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `alignItems` of a [`FlexBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexAlignItems {
    /// `center`
    Center,
    /// `flex-start`
    FlexStart,
    /// `flex-end`
    FlexEnd,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

tagged_enum! {
    /// Background of a [`FlexBox`].
    pub enum FlexBoxBackground {
        "linearGradient" => LinearGradient(FlexBoxLinearGradient),
    }
}

/// Linear gradient background of a [`FlexBox`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "linearGradient", rename_all = "camelCase")]
pub struct FlexBoxLinearGradient {
    /// Angle of the gradient.
    pub angle: Option<String>,
    /// Color at the start of the gradient.
    pub start_color: Option<String>,
    /// Color at the end of the gradient.
    pub end_color: Option<String>,
    /// Color in the middle of the gradient.
    pub center_color: Option<String>,
    /// Position of `center_color`.
    pub center_position: Option<String>,
}

impl FlexBoxLinearGradient {
    /// Creates a linear gradient from `angle`, `start_color` and `end_color`.
    pub fn new(
        angle: impl Into<String>,
        start_color: impl Into<String>,
        end_color: impl Into<String>,
    ) -> Self {
        Self::default()
            .angle(angle)
            .start_color(start_color)
            .end_color(end_color)
    }
}

setters!(FlexBoxLinearGradient {
    angle: String,
    start_color: String,
    end_color: String,
    center_color: String,
    center_position: String,
});

/// A button that performs an action when tapped.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "button", rename_all = "camelCase")]
pub struct FlexButton {
    /// The ratio of the width or height of this component within the parent box.
    pub flex: Option<u32>,
    /// Color of the button.
    pub color: Option<String>,
    /// Style of the button.
    pub style: Option<FlexButtonStyle>,
    /// Action performed when the button is tapped.
    pub action: Action,
    /// Alignment style in vertical direction.
    pub gravity: Option<FlexGravity>,
    /// The minimum amount of space to include before this component in its parent container: pixels or a keyword (`none`, `xs` to `xxl`).
    pub margin: Option<String>,
    /// Reference for the offsets: `relative` (the previous box; default) or `absolute` (the top left of the parent element).
    pub position: Option<FlexPosition>,
    /// Offset from the top: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_top: Option<String>,
    /// Offset from the bottom: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_bottom: Option<String>,
    /// Offset from the start: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_start: Option<String>,
    /// Offset from the end: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_end: Option<String>,
    /// Height of the button.
    pub height: Option<FlexButtonHeight>,
    /// Shrinks the font size to fit the width of the component when set.
    pub adjust_mode: Option<FlexAdjustMode>,
    /// Whether the component is scaled according to the font size setting of the LINE app.
    pub scaling: Option<bool>,
}

impl FlexButton {
    /// Creates a button that performs the required `action`.
    pub fn new(action: impl Into<Action>) -> Self {
        Self {
            flex: None,
            color: None,
            style: None,
            action: action.into(),
            gravity: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            height: None,
            adjust_mode: None,
            scaling: None,
        }
    }
}

setters!(FlexButton {
    flex: u32,
    color: String,
    style: FlexButtonStyle,
    gravity: FlexGravity,
    margin: String,
    position: FlexPosition,
    offset_top: String,
    offset_bottom: String,
    offset_start: String,
    offset_end: String,
    height: FlexButtonHeight,
    adjust_mode: FlexAdjustMode,
    scaling: bool,
});

/// `style` of a [`FlexButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexButtonStyle {
    /// `primary`
    Primary,
    /// `secondary`
    Secondary,
    /// `link`
    Link,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `height` of a [`FlexButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexButtonHeight {
    /// `md`
    Md,
    /// `sm`
    Sm,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `gravity` of a Flex component: alignment in the vertical direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexGravity {
    /// `top`
    Top,
    /// `bottom`
    Bottom,
    /// `center`
    Center,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `align` of a Flex component: alignment in the horizontal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexAlign {
    /// `start`
    Start,
    /// `end`
    End,
    /// `center`
    Center,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `adjustMode` of a Flex component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexAdjustMode {
    /// `shrink-to-fit`: shrinks the font size to fit the component.
    ShrinkToFit,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// An image.
///
/// <https://developers.line.biz/en/reference/messaging-api/#f-image>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "image", rename_all = "camelCase")]
pub struct FlexImage {
    /// Image URL (HTTPS, JPEG or PNG, max 2000 characters).
    pub url: String,
    /// The ratio of the width or height of this component within the parent box.
    pub flex: Option<u32>,
    /// The minimum amount of space to include before this component in its parent container (pixels or a keyword).
    pub margin: Option<String>,
    /// Reference for the offsets: `relative` (the previous box; default) or `absolute` (the top left of the parent element).
    pub position: Option<FlexPosition>,
    /// Offset from the top: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_top: Option<String>,
    /// Offset from the bottom: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_bottom: Option<String>,
    /// Offset from the start: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_start: Option<String>,
    /// Offset from the end: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_end: Option<String>,
    /// Alignment style in horizontal direction.
    pub align: Option<FlexAlign>,
    /// Alignment style in vertical direction.
    pub gravity: Option<FlexGravity>,
    /// Maximum image width: a keyword (`md` by default), or a `px`/`%` value.
    pub size: Option<String>,
    /// `{width}:{height}`; `1:1` by default.
    pub aspect_ratio: Option<String>,
    /// The display style of the image if the aspect ratio of the image and that specified by `aspect_ratio` do not match.
    pub aspect_mode: Option<FlexAspectMode>,
    /// Background color of the image, as a hexadecimal color code.
    pub background_color: Option<String>,
    /// Action performed when this component is tapped.
    pub action: Option<Action>,
    /// Plays an animated image (APNG) when `true`.
    pub animated: Option<bool>,
}

impl FlexImage {
    /// Creates an image from the required `url`.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            flex: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            align: None,
            gravity: None,
            size: None,
            aspect_ratio: None,
            aspect_mode: None,
            background_color: None,
            action: None,
            animated: None,
        }
    }
}

setters!(FlexImage {
    flex: u32,
    margin: String,
    position: FlexPosition,
    offset_top: String,
    offset_bottom: String,
    offset_start: String,
    offset_end: String,
    align: FlexAlign,
    gravity: FlexGravity,
    size: String,
    aspect_ratio: String,
    aspect_mode: FlexAspectMode,
    background_color: String,
    action: Action,
    animated: bool,
});

/// `aspectMode` of a [`FlexImage`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexAspectMode {
    /// `fit`: the whole image fits in the area, with background in the unused part.
    Fit,
    /// `cover`: the image fills the area, cropping the parts that do not fit.
    Cover,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// A video in the hero block.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "video", rename_all = "camelCase")]
pub struct FlexVideo {
    /// URL of the video file.
    pub url: String,
    /// URL of the preview image.
    pub preview_url: String,
    /// Component shown on LINE versions that cannot play the video.
    pub alt_content: Box<FlexComponent>,
    /// Aspect ratio of the video, in `{width}:{height}` format.
    pub aspect_ratio: Option<String>,
    /// Action performed when the video is tapped.
    pub action: Option<Action>,
}

impl FlexVideo {
    /// Creates a video from the required `url`, `preview_url` and `alt_content`.
    pub fn new(
        url: impl Into<String>,
        preview_url: impl Into<String>,
        alt_content: impl Into<FlexComponent>,
    ) -> Self {
        Self {
            url: url.into(),
            preview_url: preview_url.into(),
            alt_content: Box::new(alt_content.into()),
            aspect_ratio: None,
            action: None,
        }
    }
}

setters!(FlexVideo {
    aspect_ratio: String,
    action: Action,
});

/// An icon placed in a baseline box.
///
/// <https://developers.line.biz/en/reference/messaging-api/#icon>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "icon", rename_all = "camelCase")]
pub struct FlexIcon {
    /// URL of the icon image.
    pub url: String,
    /// Maximum width of the icon: pixels, a percentage, or a keyword (`xxs` to `5xl`).
    pub size: Option<String>,
    /// Aspect ratio of the icon, in `{width}:{height}` format.
    pub aspect_ratio: Option<String>,
    /// The minimum amount of space to include before this component in its parent container: pixels or a keyword (`none`, `xs` to `xxl`).
    pub margin: Option<String>,
    /// Reference for the offsets: `relative` (the previous box; default) or `absolute` (the top left of the parent element).
    pub position: Option<FlexPosition>,
    /// Offset from the top: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_top: Option<String>,
    /// Offset from the bottom: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_bottom: Option<String>,
    /// Offset from the start: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_start: Option<String>,
    /// Offset from the end: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_end: Option<String>,
    /// Whether the component is scaled according to the font size setting of the LINE app.
    pub scaling: Option<bool>,
}

impl FlexIcon {
    /// Creates an icon from the required `url`.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            size: None,
            aspect_ratio: None,
            margin: None,
            position: None,
            offset_top: None,
            offset_bottom: None,
            offset_start: None,
            offset_end: None,
            scaling: None,
        }
    }
}

setters!(FlexIcon {
    size: String,
    aspect_ratio: String,
    margin: String,
    position: FlexPosition,
    offset_top: String,
    offset_bottom: String,
    offset_start: String,
    offset_end: String,
    scaling: bool,
});

/// A text, optionally made of [`FlexSpan`]s with individual styles.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "text", rename_all = "camelCase")]
pub struct FlexText {
    /// The ratio of the width or height of this component within the parent box.
    pub flex: Option<u32>,
    /// Text.
    pub text: Option<String>,
    /// Font size: pixels or a keyword (`xxs` to `5xl`).
    pub size: Option<String>,
    /// Alignment style in horizontal direction.
    pub align: Option<FlexAlign>,
    /// Alignment style in vertical direction.
    pub gravity: Option<FlexGravity>,
    /// Font color.
    pub color: Option<String>,
    /// Font weight.
    pub weight: Option<FlexFontWeight>,
    /// Font style.
    pub style: Option<FlexFontStyle>,
    /// Text decoration.
    pub decoration: Option<FlexTextDecoration>,
    /// Whether the text wraps.
    pub wrap: Option<bool>,
    /// Line spacing of wrapped text.
    pub line_spacing: Option<String>,
    /// The minimum amount of space to include before this component in its parent container: pixels or a keyword (`none`, `xs` to `xxl`).
    pub margin: Option<String>,
    /// Reference for the offsets: `relative` (the previous box; default) or `absolute` (the top left of the parent element).
    pub position: Option<FlexPosition>,
    /// Offset from the top: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_top: Option<String>,
    /// Offset from the bottom: pixels, a percentage of the box height, or a keyword (`none`, `xs` to `xxl`).
    pub offset_bottom: Option<String>,
    /// Offset from the start: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_start: Option<String>,
    /// Offset from the end: pixels, a percentage of the box width, or a keyword (`none`, `xs` to `xxl`).
    pub offset_end: Option<String>,
    /// Action performed when this component is tapped.
    pub action: Option<Action>,
    /// Maximum number of lines.
    pub max_lines: Option<u32>,
    /// Spans that make up the text, each with its own style.
    pub contents: Option<Vec<FlexSpan>>,
    /// Shrinks the font size to fit the width of the component when set.
    pub adjust_mode: Option<FlexAdjustMode>,
    /// Whether the component is scaled according to the font size setting of the LINE app.
    pub scaling: Option<bool>,
}

impl FlexText {
    /// Creates a text component with `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self::default().text(text)
    }
}

setters!(FlexText {
    flex: u32,
    text: String,
    size: String,
    align: FlexAlign,
    gravity: FlexGravity,
    color: String,
    weight: FlexFontWeight,
    style: FlexFontStyle,
    decoration: FlexTextDecoration,
    wrap: bool,
    line_spacing: String,
    margin: String,
    position: FlexPosition,
    offset_top: String,
    offset_bottom: String,
    offset_start: String,
    offset_end: String,
    action: Action,
    max_lines: u32,
    contents: [FlexSpan],
    adjust_mode: FlexAdjustMode,
    scaling: bool,
});

/// A part of a [`FlexText`] with its own style.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "span", rename_all = "camelCase")]
pub struct FlexSpan {
    /// Text.
    pub text: Option<String>,
    /// Font size: pixels or a keyword (`xxs` to `5xl`).
    pub size: Option<String>,
    /// Font color.
    pub color: Option<String>,
    /// Font weight.
    pub weight: Option<FlexFontWeight>,
    /// Font style.
    pub style: Option<FlexFontStyle>,
    /// Text decoration.
    pub decoration: Option<FlexTextDecoration>,
}

impl FlexSpan {
    /// Creates a span with `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self::default().text(text)
    }
}

setters!(FlexSpan {
    text: String,
    size: String,
    color: String,
    weight: FlexFontWeight,
    style: FlexFontStyle,
    decoration: FlexTextDecoration,
});

/// `weight` of a [`FlexText`] or [`FlexSpan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexFontWeight {
    /// `regular`
    Regular,
    /// `bold`
    Bold,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `style` of a [`FlexText`] or [`FlexSpan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexFontStyle {
    /// `normal`
    Normal,
    /// `italic`
    Italic,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// `decoration` of a [`FlexText`] or [`FlexSpan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlexTextDecoration {
    /// `none`
    None,
    /// `underline`
    Underline,
    /// `line-through`
    LineThrough,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// A horizontal or vertical dividing line.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "separator", rename_all = "camelCase")]
pub struct FlexSeparator {
    /// The minimum amount of space to include before this component in its parent container: pixels or a keyword (`none`, `xs` to `xxl`).
    pub margin: Option<String>,
    /// Color of the separator.
    pub color: Option<String>,
}

setters!(FlexSeparator {
    margin: String,
    color: String,
});

/// Empty space that takes up the remaining room in a box.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "filler", rename_all = "camelCase")]
pub struct FlexFiller {
    /// The ratio of the width or height of this component within the parent box.
    pub flex: Option<u32>,
}

setters!(FlexFiller { flex: u32 });
