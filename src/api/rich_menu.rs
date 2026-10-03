use reqwest::Method;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::{Content, Host, LineClient};
use crate::error::Result;
use crate::message::Action;

impl LineClient {
    /// Creates a rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#create-rich-menu>
    pub async fn create_rich_menu(&self, request: &RichMenuRequest) -> Result<RichMenuIdResponse> {
        let rb = self.request(Method::POST, Host::Api, &["v2", "bot", "richmenu"]);
        self.send_json(rb.json(request)).await
    }

    /// Validates a rich menu object.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-rich-menu-object>
    pub async fn validate_rich_menu_object(&self, request: &RichMenuRequest) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "richmenu", "validate"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Downloads the image of a rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#download-rich-menu-image>
    pub async fn get_rich_menu_image(&self, rich_menu_id: &str) -> Result<Content> {
        let rb = self.request(
            Method::GET,
            Host::Data,
            &["v2", "bot", "richmenu", rich_menu_id, "content"],
        );
        self.send_bytes(rb).await
    }

    /// Uploads the image of a rich menu (`image/jpeg` or `image/png`).
    /// <https://developers.line.biz/en/reference/messaging-api/#upload-rich-menu-image>
    pub async fn set_rich_menu_image(
        &self,
        rich_menu_id: &str,
        data: impl Into<Vec<u8>>,
        content_type: &str,
    ) -> Result<()> {
        let rb = self
            .request(
                Method::POST,
                Host::Data,
                &["v2", "bot", "richmenu", rich_menu_id, "content"],
            )
            .header(CONTENT_TYPE, content_type)
            .body(data.into());
        self.send_empty(rb).await
    }

    /// Gets a rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu>
    pub async fn get_rich_menu(&self, rich_menu_id: &str) -> Result<RichMenuResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "richmenu", rich_menu_id],
        );
        self.send_json(rb).await
    }

    /// Deletes a rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#delete-rich-menu>
    pub async fn delete_rich_menu(&self, rich_menu_id: &str) -> Result<()> {
        let rb = self.request(
            Method::DELETE,
            Host::Api,
            &["v2", "bot", "richmenu", rich_menu_id],
        );
        self.send_empty(rb).await
    }

    /// Gets the list of rich menus.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-list>
    pub async fn get_rich_menu_list(&self) -> Result<RichMenuListResponse> {
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "richmenu", "list"]);
        self.send_json(rb).await
    }

    /// Sets the default rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#set-default-rich-menu>
    pub async fn set_default_rich_menu(&self, rich_menu_id: &str) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "user", "all", "richmenu", rich_menu_id],
        );
        self.send_empty(rb).await
    }

    /// Gets the ID of the default rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-default-rich-menu-id>
    pub async fn get_default_rich_menu_id(&self) -> Result<RichMenuIdResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "user", "all", "richmenu"],
        );
        self.send_json(rb).await
    }

    /// Cancels the default rich menu.
    /// <https://developers.line.biz/en/reference/messaging-api/#cancel-default-rich-menu>
    pub async fn cancel_default_rich_menu(&self) -> Result<()> {
        let rb = self.request(
            Method::DELETE,
            Host::Api,
            &["v2", "bot", "user", "all", "richmenu"],
        );
        self.send_empty(rb).await
    }

    /// Creates a rich menu alias.
    /// <https://developers.line.biz/en/reference/messaging-api/#create-rich-menu-alias>
    pub async fn create_rich_menu_alias(&self, request: &CreateRichMenuAliasRequest) -> Result<()> {
        let rb = self.request(Method::POST, Host::Api, &["v2", "bot", "richmenu", "alias"]);
        self.send_empty(rb.json(request)).await
    }

    /// Gets a rich menu alias.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-alias-by-id>
    pub async fn get_rich_menu_alias(
        &self,
        rich_menu_alias_id: &str,
    ) -> Result<RichMenuAliasResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "richmenu", "alias", rich_menu_alias_id],
        );
        self.send_json(rb).await
    }

    /// Updates the rich menu a rich menu alias points to.
    /// <https://developers.line.biz/en/reference/messaging-api/#update-rich-menu-alias>
    pub async fn update_rich_menu_alias(
        &self,
        rich_menu_alias_id: &str,
        request: &UpdateRichMenuAliasRequest,
    ) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "richmenu", "alias", rich_menu_alias_id],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Deletes a rich menu alias.
    /// <https://developers.line.biz/en/reference/messaging-api/#delete-rich-menu-alias>
    pub async fn delete_rich_menu_alias(&self, rich_menu_alias_id: &str) -> Result<()> {
        let rb = self.request(
            Method::DELETE,
            Host::Api,
            &["v2", "bot", "richmenu", "alias", rich_menu_alias_id],
        );
        self.send_empty(rb).await
    }

    /// Gets the list of rich menu aliases.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-alias-list>
    pub async fn get_rich_menu_alias_list(&self) -> Result<RichMenuAliasListResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "richmenu", "alias", "list"],
        );
        self.send_json(rb).await
    }

    /// Gets the ID of the rich menu linked to a user.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-id-of-user>
    pub async fn get_rich_menu_id_of_user(&self, user_id: &str) -> Result<RichMenuIdResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "user", user_id, "richmenu"],
        );
        self.send_json(rb).await
    }

    /// Unlinks the rich menu from a user.
    /// <https://developers.line.biz/en/reference/messaging-api/#unlink-rich-menu-from-user>
    pub async fn unlink_rich_menu_id_from_user(&self, user_id: &str) -> Result<()> {
        let rb = self.request(
            Method::DELETE,
            Host::Api,
            &["v2", "bot", "user", user_id, "richmenu"],
        );
        self.send_empty(rb).await
    }

    /// Links a rich menu to a user.
    /// <https://developers.line.biz/en/reference/messaging-api/#link-rich-menu-to-user>
    pub async fn link_rich_menu_id_to_user(&self, user_id: &str, rich_menu_id: &str) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "user", user_id, "richmenu", rich_menu_id],
        );
        self.send_empty(rb).await
    }

    /// Links a rich menu to multiple users.
    /// <https://developers.line.biz/en/reference/messaging-api/#link-rich-menu-to-users>
    pub async fn link_rich_menu_id_to_users(
        &self,
        request: &RichMenuBulkLinkRequest,
    ) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "richmenu", "bulk", "link"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Unlinks rich menus from multiple users.
    /// <https://developers.line.biz/en/reference/messaging-api/#unlink-rich-menu-from-users>
    pub async fn unlink_rich_menu_id_from_users(
        &self,
        request: &RichMenuBulkUnlinkRequest,
    ) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "richmenu", "bulk", "unlink"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Replaces or unlinks the rich menus linked to users in a batch.
    /// <https://developers.line.biz/en/reference/messaging-api/#batch-control-rich-menus-of-users>
    ///
    /// The batch runs asynchronously: pass the returned
    /// [`RichMenuBatchResponse::request_id`] to [`LineClient::get_rich_menu_batch_progress`].
    pub async fn rich_menu_batch(
        &self,
        request: &RichMenuBatchRequest,
    ) -> Result<RichMenuBatchResponse> {
        let rb = self.request(Method::POST, Host::Api, &["v2", "bot", "richmenu", "batch"]);
        let (mut response, request_id): (RichMenuBatchResponse, _) =
            self.send_json_with_request_id(rb.json(request)).await?;
        response.request_id = request_id;
        Ok(response)
    }

    /// Validates a request for [`LineClient::rich_menu_batch`].
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-batch-control-rich-menus-request>
    pub async fn validate_rich_menu_batch_request(
        &self,
        request: &RichMenuBatchRequest,
    ) -> Result<()> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "richmenu", "validate", "batch"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Gets the progress of a rich menu batch control request.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-batch-control-rich-menus-progress-status>
    pub async fn get_rich_menu_batch_progress(
        &self,
        request_id: &str,
    ) -> Result<RichMenuBatchProgressResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "richmenu", "progress", "batch"],
            )
            .query(&[("requestId", request_id)]);
        self.send_json(rb).await
    }
}

/// Rich menu object sent to [`LineClient::create_rich_menu`] and [`LineClient::validate_rich_menu_object`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuRequest {
    /// Rich menu size.
    pub size: RichMenuSize,
    /// `true` to display the rich menu by default.
    pub selected: bool,
    /// Name used to manage rich menus; not displayed to users (max 300 characters).
    pub name: String,
    /// Text displayed in the chat bar (max 14 characters).
    pub chat_bar_text: String,
    /// Areas that define the coordinates and size of tappable areas.
    pub areas: Vec<RichMenuArea>,
}

impl RichMenuRequest {
    /// Creates a rich menu object.
    pub fn new(
        size: RichMenuSize,
        selected: bool,
        name: impl Into<String>,
        chat_bar_text: impl Into<String>,
        areas: impl IntoIterator<Item = RichMenuArea>,
    ) -> Self {
        Self {
            size,
            selected,
            name: name.into(),
            chat_bar_text: chat_bar_text.into(),
            areas: areas.into_iter().collect(),
        }
    }
}

/// Rich menu object returned by the rich menu endpoints.
///
/// <https://developers.line.biz/en/reference/messaging-api/#rich-menu-response-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuResponse {
    /// ID of the rich menu.
    pub rich_menu_id: String,
    /// Rich menu size.
    pub size: RichMenuSize,
    /// `true` to display the rich menu by default.
    pub selected: bool,
    /// Name used to manage rich menus; not displayed to users (max 300 characters).
    pub name: String,
    /// Text displayed in the chat bar (max 14 characters).
    pub chat_bar_text: String,
    /// Areas that define the coordinates and size of tappable areas (max 20).
    pub areas: Vec<RichMenuArea>,
}

/// Rich menu size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichMenuSize {
    /// Width (min 1).
    pub width: u64,
    /// Height (min 1).
    pub height: u64,
}

impl RichMenuSize {
    /// Creates a rich menu size.
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

/// Tappable area of a rich menu.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichMenuArea {
    /// Position and size of the area.
    pub bounds: RichMenuBounds,
    /// Action performed when the area is tapped.
    pub action: Action,
}

impl RichMenuArea {
    /// Creates an area with `bounds` that performs `action` when tapped.
    pub fn new(bounds: RichMenuBounds, action: impl Into<Action>) -> Self {
        Self {
            bounds,
            action: action.into(),
        }
    }
}

/// Position and size of a rich menu area.
///
/// <https://developers.line.biz/en/reference/messaging-api/#bounds-object>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichMenuBounds {
    /// Horizontal position relative to the top-left corner of the area.
    pub x: u64,
    /// Vertical position relative to the top-left corner of the area.
    pub y: u64,
    /// Width of the area (min 1).
    pub width: u64,
    /// Height of the area (min 1).
    pub height: u64,
}

impl RichMenuBounds {
    /// Creates bounds from a position and a size.
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Response holding a rich menu ID.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuIdResponse {
    /// Rich menu ID.
    pub rich_menu_id: String,
}

/// List of rich menus.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-list>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RichMenuListResponse {
    /// Rich menus.
    pub richmenus: Vec<RichMenuResponse>,
}

/// Request body of [`LineClient::create_rich_menu_alias`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#create-rich-menu-alias>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRichMenuAliasRequest {
    /// Any ID unique within the channel, matching `^[a-z0-9_-]{1,32}$`.
    pub rich_menu_alias_id: String,
    /// The rich menu ID to be associated with the rich menu alias.
    pub rich_menu_id: String,
}

impl CreateRichMenuAliasRequest {
    /// Creates a request that associates `rich_menu_alias_id` with `rich_menu_id`.
    pub fn new(rich_menu_alias_id: impl Into<String>, rich_menu_id: impl Into<String>) -> Self {
        Self {
            rich_menu_alias_id: rich_menu_alias_id.into(),
            rich_menu_id: rich_menu_id.into(),
        }
    }
}

/// Request body of [`LineClient::update_rich_menu_alias`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#update-rich-menu-alias>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRichMenuAliasRequest {
    /// The rich menu ID to be associated with the rich menu alias.
    pub rich_menu_id: String,
}

impl UpdateRichMenuAliasRequest {
    /// Creates a request that associates the alias with `rich_menu_id`.
    pub fn new(rich_menu_id: impl Into<String>) -> Self {
        Self {
            rich_menu_id: rich_menu_id.into(),
        }
    }
}

/// Rich menu alias.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuAliasResponse {
    /// Rich menu alias ID.
    pub rich_menu_alias_id: String,
    /// The rich menu ID associated with the rich menu alias.
    pub rich_menu_id: String,
}

/// List of rich menu aliases.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-alias-list>
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RichMenuAliasListResponse {
    /// Rich menu aliases.
    pub aliases: Vec<RichMenuAliasResponse>,
}

/// Request body of [`LineClient::link_rich_menu_id_to_users`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#link-rich-menu-to-users>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuBulkLinkRequest {
    /// ID of the rich menu.
    pub rich_menu_id: String,
    /// User IDs (1 to 500), found in the `source` object of webhook events. Do not use LINE IDs.
    pub user_ids: Vec<String>,
}

impl RichMenuBulkLinkRequest {
    /// Creates a request that links `rich_menu_id` to `user_ids` (1 to 500).
    pub fn new(
        rich_menu_id: impl Into<String>,
        user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            rich_menu_id: rich_menu_id.into(),
            user_ids: user_ids.into_iter().map(Into::into).collect(),
        }
    }
}

/// Request body of [`LineClient::unlink_rich_menu_id_from_users`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#unlink-rich-menu-from-users>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuBulkUnlinkRequest {
    /// User IDs (1 to 500), found in the `source` object of webhook events. Do not use LINE IDs.
    pub user_ids: Vec<String>,
}

impl RichMenuBulkUnlinkRequest {
    /// Creates a request that unlinks rich menus from `user_ids` (1 to 500).
    pub fn new(user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            user_ids: user_ids.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response of [`LineClient::rich_menu_batch`].
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RichMenuBatchResponse {
    /// Value of the `x-line-request-id` response header.
    /// Pass it to [`LineClient::get_rich_menu_batch_progress`].
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// Request body of [`LineClient::rich_menu_batch`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#batch-control-rich-menus-of-users>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuBatchRequest {
    /// Operations (max 1000).
    pub operations: Vec<RichMenuBatchOperation>,
    /// Key for retrying the request, matching `^[a-zA-Z0-9_-]{1,100}$`.
    pub resume_request_key: Option<String>,
}

impl RichMenuBatchRequest {
    /// Creates a request with `operations` (max 1000).
    pub fn new(operations: impl IntoIterator<Item = impl Into<RichMenuBatchOperation>>) -> Self {
        Self {
            operations: operations.into_iter().map(Into::into).collect(),
            resume_request_key: None,
        }
    }

    /// Sets the key for retrying the request.
    pub fn resume_request_key(mut self, resume_request_key: impl Into<String>) -> Self {
        self.resume_request_key = Some(resume_request_key.into());
        self
    }
}

/// Batch operation on the rich menus linked to users.
///
/// <https://developers.line.biz/en/reference/messaging-api/#batch-control-rich-menus-of-users-operations>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RichMenuBatchOperation {
    /// Replaces the rich menu `from` with `to` for all users linked to `from`.
    Link(RichMenuBatchLinkOperation),
    /// Unlinks the rich menu `from` from all users linked to it.
    Unlink(RichMenuBatchUnlinkOperation),
    /// Unlinks the rich menu from all users linked to the rich menu.
    UnlinkAll,
}

/// Replaces the rich menu `from` with `to` for all users linked to `from`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichMenuBatchLinkOperation {
    /// ID of the rich menu to replace.
    pub from: String,
    /// ID of the rich menu to link instead.
    pub to: String,
}

impl RichMenuBatchLinkOperation {
    /// Creates an operation that replaces `from` with `to`.
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl From<RichMenuBatchLinkOperation> for RichMenuBatchOperation {
    fn from(value: RichMenuBatchLinkOperation) -> Self {
        Self::Link(value)
    }
}

/// Unlinks the rich menu `from` from all users linked to it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichMenuBatchUnlinkOperation {
    /// ID of the rich menu to unlink.
    pub from: String,
}

impl RichMenuBatchUnlinkOperation {
    /// Creates an operation that unlinks `from`.
    pub fn new(from: impl Into<String>) -> Self {
        Self { from: from.into() }
    }
}

impl From<RichMenuBatchUnlinkOperation> for RichMenuBatchOperation {
    fn from(value: RichMenuBatchUnlinkOperation) -> Self {
        Self::Unlink(value)
    }
}

/// Progress of a rich menu batch control request.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-batch-control-rich-menus-progress-status-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuBatchProgressResponse {
    /// The current status.
    pub phase: RichMenuBatchProgressPhase,
    /// ISO 8601 (e.g. `2023-06-08T10:15:30.121Z`).
    pub accepted_time: String,
    /// ISO 8601. Set when `phase` is `succeeded` or `failed`.
    pub completed_time: Option<String>,
}

/// Status of a rich menu batch control request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RichMenuBatchProgressPhase {
    /// Rich menu batch control is in progress.
    Ongoing,
    /// Rich menu batch control is complete.
    Succeeded,
    /// Rich menu batch control failed: the rich menu for one or more users couldn't be controlled. Operations for some users may have completed.
    Failed,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}
