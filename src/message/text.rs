use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{QuickReply, Sender};

/// A text message, optionally with LINE emojis.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "text", rename_all = "camelCase")]
pub struct TextMessage {
    /// Message text.
    pub text: String,
    /// LINE emojis embedded in `text`.
    pub emojis: Option<Vec<Emoji>>,
    /// Quote token of the message you want to quote.
    pub quote_token: Option<String>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl TextMessage {
    /// Creates a text message with the required `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            emojis: None,
            quote_token: None,
            quick_reply: None,
            sender: None,
        }
    }
}

setters!(TextMessage {
    emojis: [Emoji],
    quote_token: String,
    quick_reply: QuickReply,
    sender: Sender,
});

/// A LINE emoji embedded in [`TextMessage::text`] at the `$` placeholder at `index`.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    /// Index of the `$` placeholder in the text that the emoji replaces.
    pub index: Option<u32>,
    /// Product ID of the LINE emoji set.
    pub product_id: Option<String>,
    /// ID of the LINE emoji within the set.
    pub emoji_id: Option<String>,
}

impl Emoji {
    /// Creates an emoji from the placeholder `index`, `product_id` and `emoji_id`.
    pub fn new(index: u32, product_id: impl Into<String>, emoji_id: impl Into<String>) -> Self {
        Self {
            index: Some(index),
            product_id: Some(product_id.into()),
            emoji_id: Some(emoji_id.into()),
        }
    }
}

setters!(Emoji {
    index: u32,
    product_id: String,
    emoji_id: String,
});

/// Text message whose `{key}` placeholders are replaced with mentions or emojis.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message-v2>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "textV2", rename_all = "camelCase")]
pub struct TextMessageV2 {
    /// Message text, with `{key}` placeholders.
    pub text: String,
    /// Replacement for each `{key}` placeholder in `text`.
    pub substitution: Option<BTreeMap<String, SubstitutionObject>>,
    /// Quote token of the message you want to quote.
    pub quote_token: Option<String>,
    /// Quick reply buttons shown with the message.
    pub quick_reply: Option<QuickReply>,
    /// Icon and display name to show as the sender.
    pub sender: Option<Sender>,
}

impl TextMessageV2 {
    /// Creates a text message (v2) with the required `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            substitution: None,
            quote_token: None,
            quick_reply: None,
            sender: None,
        }
    }

    /// Adds the replacement for the `{key}` placeholder.
    pub fn substitution(
        mut self,
        key: impl Into<String>,
        value: impl Into<SubstitutionObject>,
    ) -> Self {
        self.substitution
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }
}

setters!(TextMessageV2 {
    quote_token: String,
    quick_reply: QuickReply,
    sender: Sender,
});

tagged_enum! {
    /// Replacement value for a placeholder in [`TextMessageV2::text`].
    pub enum SubstitutionObject {
        "mention" => Mention(MentionSubstitutionObject),
        "emoji" => Emoji(EmojiSubstitutionObject),
    }
}

/// A mention substitution.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message-v2-mention-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "mention", rename_all = "camelCase")]
pub struct MentionSubstitutionObject {
    /// Target to be mentioned.
    pub mentionee: MentionTarget,
}

impl MentionSubstitutionObject {
    /// Creates a mention substitution with the required `mentionee`.
    pub fn new(mentionee: impl Into<MentionTarget>) -> Self {
        Self {
            mentionee: mentionee.into(),
        }
    }
}

tagged_enum! {
    /// Target of a [`MentionSubstitutionObject`].
    pub enum MentionTarget {
        "user" => User(UserMentionTarget),
        "all" => All(AllMentionTarget),
    }
}

/// Mentions a specific user.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message-v2-mentionee-user>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "user", rename_all = "camelCase")]
pub struct UserMentionTarget {
    /// User ID of the user to mention.
    pub user_id: String,
}

impl UserMentionTarget {
    /// Creates a user mention target with the required `user_id`.
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

/// Mentions all members of the chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message-v2-mentionee-all>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "all")]
pub struct AllMentionTarget {}

/// An emoji substitution.
///
/// <https://developers.line.biz/en/reference/messaging-api/#text-message-v2-emoji-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emoji", rename_all = "camelCase")]
pub struct EmojiSubstitutionObject {
    /// Product ID of the LINE emoji set.
    pub product_id: String,
    /// ID of the LINE emoji within the set.
    pub emoji_id: String,
}

impl EmojiSubstitutionObject {
    /// Creates an emoji substitution from the required `product_id` and `emoji_id`.
    pub fn new(product_id: impl Into<String>, emoji_id: impl Into<String>) -> Self {
        Self {
            product_id: product_id.into(),
            emoji_id: emoji_id.into(),
        }
    }
}
