//! Message objects: messages, actions, quick replies, imagemaps, templates and Flex Messages.
//!
//! ```
//! use line_bot_messaging_api::message::{Message, PostbackAction, QuickReply, QuickReplyItem, TextMessage};
//!
//! let message: Message = TextMessage::new("Hello")
//!     .quick_reply(QuickReply::new([QuickReplyItem::new(PostbackAction::new("action=buy").label("Buy"))]))
//!     .into();
//! ```
//!
//! Flex Message JSON exported from the [Flex Message Simulator](https://developers.line.biz/flex-simulator/)
//! decodes into [`FlexContainer`] with `serde_json::from_str`.

mod action;
mod flex;
mod imagemap;
mod media;
mod quick_reply;
mod template;
mod text;

pub use action::*;
pub use flex::*;
pub use imagemap::*;
pub use media::*;
pub use quick_reply::*;
pub use template::*;
pub use text::*;

tagged_enum! {
    /// A message to send.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#message-common-properties>
    pub enum Message {
        "text" => Text(TextMessage),
        "textV2" => TextV2(TextMessageV2),
        "sticker" => Sticker(StickerMessage),
        "image" => Image(ImageMessage),
        "video" => Video(VideoMessage),
        "audio" => Audio(AudioMessage),
        "location" => Location(LocationMessage),
        "imagemap" => Imagemap(ImagemapMessage),
        "template" => Template(TemplateMessage),
        "flex" => Flex(FlexMessage),
        "coupon" => Coupon(CouponMessage),
    }
}
