use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineClient;
use crate::client::Host;
use crate::error::Result;

impl LineClient {
    /// Gets the profile of a group chat member.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-group-member-profile>
    pub async fn get_group_member_profile(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Result<GroupUserProfileResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "group", group_id, "member", user_id],
        );
        self.send_json(rb).await
    }

    /// Gets the user IDs of group chat members.
    /// `start` is the `next` value of the previous page.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-group-member-user-ids>
    pub async fn get_group_members_ids(
        &self,
        group_id: &str,
        start: Option<&str>,
    ) -> Result<MembersIdsResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "group", group_id, "members", "ids"],
            )
            .query(&[("start", start)]);
        self.send_json(rb).await
    }

    /// Leaves a group chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#leave-group>
    pub async fn leave_group(&self, group_id: &str) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "group", group_id, "leave"],
        );
        self.send_empty(rb).await
    }

    /// Gets the group ID, name and icon URL of a group chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-group-summary>
    pub async fn get_group_summary(&self, group_id: &str) -> Result<GroupSummaryResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "group", group_id, "summary"],
        );
        self.send_json(rb).await
    }

    /// Gets the number of users in a group chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-members-group-count>
    pub async fn get_group_member_count(&self, group_id: &str) -> Result<GroupMemberCountResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "group", group_id, "members", "count"],
        );
        self.send_json(rb).await
    }
}

/// Profile of a group chat member.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-group-member-profile>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupUserProfileResponse {
    /// User's display name.
    pub display_name: String,
    /// User ID.
    pub user_id: String,
    /// Profile image URL (`https`). Not included if the user doesn't have a profile image.
    pub picture_url: Option<String>,
}

/// User IDs of group chat or multi-person chat members.
/// <https://developers.line.biz/en/reference/messaging-api/#get-group-member-user-ids>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersIdsResponse {
    /// User IDs of members. Only users of LINE for iOS and LINE for Android are included (max 100).
    pub member_ids: Vec<String>,
    /// Continuation token to get the next page of user IDs. Returned only when user IDs remain.
    pub next: Option<String>,
}

/// Summary of a group chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-group-summary>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSummaryResponse {
    /// Group ID.
    pub group_id: String,
    /// Group name.
    pub group_name: String,
    /// Group icon URL. Not included if the group profile icon isn't set.
    pub picture_url: Option<String>,
}

/// Number of members in a group chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-members-group-count>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberCountResponse {
    /// The count of members in the group chat, excluding the LINE Official Account.
    pub count: u32,
}
