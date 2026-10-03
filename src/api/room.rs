use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineClient;
use crate::api::MembersIdsResponse;
use crate::client::Host;
use crate::error::Result;

impl LineClient {
    /// Gets the profile of a multi-person chat member.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-room-member-profile>
    pub async fn get_room_member_profile(
        &self,
        room_id: &str,
        user_id: &str,
    ) -> Result<RoomUserProfileResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "room", room_id, "member", user_id],
        );
        self.send_json(rb).await
    }

    /// Gets the user IDs of multi-person chat members.
    /// `start` is the `next` value of the previous page.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-room-member-user-ids>
    pub async fn get_room_members_ids(
        &self,
        room_id: &str,
        start: Option<&str>,
    ) -> Result<MembersIdsResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "room", room_id, "members", "ids"],
            )
            .query(&[("start", start)]);
        self.send_json(rb).await
    }

    /// Leaves a multi-person chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#leave-room>
    pub async fn leave_room(&self, room_id: &str) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "room", room_id, "leave"],
        );
        self.send_empty(rb).await
    }

    /// Gets the number of users in a multi-person chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-members-room-count>
    pub async fn get_room_member_count(&self, room_id: &str) -> Result<RoomMemberCountResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "room", room_id, "members", "count"],
        );
        self.send_json(rb).await
    }
}

/// Profile of a multi-person chat member.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-room-member-profile>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomUserProfileResponse {
    /// User's display name.
    pub display_name: String,
    /// User ID.
    pub user_id: String,
    /// Profile image URL (`https`). Not included if the user doesn't have a profile image.
    pub picture_url: Option<String>,
}

/// Number of members in a multi-person chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-members-room-count>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMemberCountResponse {
    /// The count of members in the multi-person chat, excluding the LINE Official Account.
    pub count: u32,
}
