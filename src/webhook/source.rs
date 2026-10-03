use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tagged_enum! {
    /// Source of an event.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#source-user>
    pub enum Source {
        /// One-on-one chat with a user.
        "user" => User(UserSource),
        /// Group chat.
        "group" => Group(GroupSource),
        /// Multi-person chat.
        "room" => Room(RoomSource),
    }
}

impl Source {
    /// Returns the ID of the user who caused the event, when included.
    pub fn user_id(&self) -> Option<&str> {
        match self {
            Source::User(s) => s.user_id.as_deref(),
            Source::Group(s) => s.user_id.as_deref(),
            Source::Room(s) => s.user_id.as_deref(),
            Source::Unknown(_) => None,
        }
    }
}

/// <https://developers.line.biz/en/reference/messaging-api/#source-user>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "user", rename_all = "camelCase")]
pub struct UserSource {
    /// ID of the source user.
    pub user_id: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#source-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "group", rename_all = "camelCase")]
pub struct GroupSource {
    /// Group ID of the source group chat.
    pub group_id: String,
    /// ID of the source user. Only included in message events.
    pub user_id: Option<String>,
}

/// <https://developers.line.biz/en/reference/messaging-api/#source-room>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "room", rename_all = "camelCase")]
pub struct RoomSource {
    /// Room ID of the source multi-person chat.
    pub room_id: String,
    /// ID of the source user. Only included in message events.
    pub user_id: Option<String>,
}
