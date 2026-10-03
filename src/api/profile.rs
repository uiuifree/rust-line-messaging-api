use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineClient;
use crate::client::Host;
use crate::error::Result;

impl LineClient {
    /// Gets the profile of a user who added the bot as a friend.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-profile>
    pub async fn get_profile(&self, user_id: &str) -> Result<UserProfileResponse> {
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "profile", user_id]);
        self.send_json(rb).await
    }

    /// Gets the user IDs of users who added the bot as a friend.
    /// `start` is the `next` value of the previous page; `limit` defaults to 300 (max 1000).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-follower-ids>
    pub async fn get_followers(
        &self,
        start: Option<&str>,
        limit: Option<u32>,
    ) -> Result<GetFollowersResponse> {
        let rb = self
            .request(Method::GET, Host::Api, &["v2", "bot", "followers", "ids"])
            .query(&[("start", start)])
            .query(&[("limit", limit)]);
        self.send_json(rb).await
    }

    /// Gets the bot's basic information.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-bot-info>
    pub async fn get_bot_info(&self) -> Result<BotInfoResponse> {
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "info"]);
        self.send_json(rb).await
    }
}

/// Profile of a user.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-profile>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponse {
    /// User's display name.
    pub display_name: String,
    /// User ID.
    pub user_id: String,
    /// Profile image URL (`https`). Not included if the user doesn't have a profile image.
    pub picture_url: Option<String>,
    /// User's status message. Not included if the user doesn't have a status message.
    pub status_message: Option<String>,
    /// User's language, as a BCP 47 language tag. Not included if the user hasn't yet consented to the LINE Privacy Policy.
    pub language: Option<String>,
}

/// User IDs of users who added the LINE Official Account as a friend.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-follower-ids>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFollowersResponse {
    /// User IDs (max 1000). Only users of LINE for iOS and LINE for Android are included.
    pub user_ids: Vec<String>,
    /// Continuation token to get the next page of user IDs. Returned only when user IDs remain.
    pub next: Option<String>,
}

/// Basic information of the bot.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-bot-info>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfoResponse {
    /// Bot's user ID.
    pub user_id: String,
    /// Bot's basic ID.
    pub basic_id: String,
    /// Bot's premium ID. Not included if the premium ID isn't set.
    pub premium_id: Option<String>,
    /// Bot's display name.
    pub display_name: String,
    /// Profile image URL (`https`). Not included if the bot doesn't have a profile image.
    pub picture_url: Option<String>,
    /// Chat settings set in the LINE Official Account Manager.
    pub chat_mode: ChatMode,
    /// Automatic read setting for messages.
    pub mark_as_read_mode: MarkAsReadMode,
}

/// Chat setting of a LINE Official Account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatMode {
    /// Chat is set to "On".
    Chat,
    /// Chat is set to "Off".
    Bot,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}

/// Automatic read setting for messages. `auto` is returned when chat is "Off", `manual` when chat is "On".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkAsReadMode {
    /// Auto read setting is enabled.
    Auto,
    /// Auto read setting is disabled.
    Manual,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}
