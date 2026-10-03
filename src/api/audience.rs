use reqwest::Method;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::{Host, LineClient};
use crate::error::Result;

/// File name sent with the uploaded user ID / IFA list.
const AUDIENCE_FILE_NAME: &str = "audiences.txt";

impl LineClient {
    /// Creates an audience for uploading user IDs (by JSON).
    /// <https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group>
    pub async fn create_audience_group(
        &self,
        request: &CreateAudienceGroupRequest,
    ) -> Result<CreateAudienceGroupResponse> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "audienceGroup", "upload"],
        );
        self.send_json(rb.json(request)).await
    }

    /// Adds user IDs or IFAs to an audience for uploading user IDs (by JSON).
    /// <https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group>
    pub async fn add_audience_to_audience_group(
        &self,
        request: &AddAudienceToAudienceGroupRequest,
    ) -> Result<()> {
        let rb = self.request(
            Method::PUT,
            Host::Api,
            &["v2", "bot", "audienceGroup", "upload"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Creates an audience for uploading user IDs (by file).
    /// <https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group-by-file>
    pub async fn create_audience_for_uploading_user_ids(
        &self,
        request: &CreateAudienceForUploadingUserIdsRequest,
    ) -> Result<CreateAudienceGroupResponse> {
        let mut form = Form::new();
        if let Some(description) = &request.description {
            form = form.text("description", description.clone());
        }
        if let Some(is_ifa_audience) = request.is_ifa_audience {
            form = form.text("isIfaAudience", is_ifa_audience.to_string());
        }
        if let Some(upload_description) = &request.upload_description {
            form = form.text("uploadDescription", upload_description.clone());
        }
        let form = form.part("file", audience_file(&request.file));
        let rb = self.request(
            Method::POST,
            Host::Data,
            &["v2", "bot", "audienceGroup", "upload", "byFile"],
        );
        self.send_json(rb.multipart(form)).await
    }

    /// Adds user IDs or IFAs to an audience for uploading user IDs (by file).
    /// <https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group-by-file>
    pub async fn add_user_ids_to_audience(
        &self,
        request: &AddUserIdsToAudienceRequest,
    ) -> Result<()> {
        let mut form = Form::new();
        if let Some(audience_group_id) = request.audience_group_id {
            form = form.text("audienceGroupId", audience_group_id.to_string());
        }
        if let Some(upload_description) = &request.upload_description {
            form = form.text("uploadDescription", upload_description.clone());
        }
        let form = form.part("file", audience_file(&request.file));
        let rb = self.request(
            Method::PUT,
            Host::Data,
            &["v2", "bot", "audienceGroup", "upload", "byFile"],
        );
        self.send_empty(rb.multipart(form)).await
    }

    /// Creates an audience for click-based retargeting.
    /// <https://developers.line.biz/en/reference/messaging-api/#create-click-audience-group>
    pub async fn create_click_based_audience_group(
        &self,
        request: &CreateClickBasedAudienceGroupRequest,
    ) -> Result<CreateClickBasedAudienceGroupResponse> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "audienceGroup", "click"],
        );
        self.send_json(rb.json(request)).await
    }

    /// Creates an audience for impression-based retargeting.
    /// <https://developers.line.biz/en/reference/messaging-api/#create-imp-audience-group>
    pub async fn create_imp_based_audience_group(
        &self,
        request: &CreateImpBasedAudienceGroupRequest,
    ) -> Result<CreateImpBasedAudienceGroupResponse> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "audienceGroup", "imp"],
        );
        self.send_json(rb.json(request)).await
    }

    /// Renames an audience.
    /// <https://developers.line.biz/en/reference/messaging-api/#set-description-audience-group>
    pub async fn update_audience_group_description(
        &self,
        audience_group_id: i64,
        request: &UpdateAudienceGroupDescriptionRequest,
    ) -> Result<()> {
        let id = audience_group_id.to_string();
        let rb = self.request(
            Method::PUT,
            Host::Api,
            &["v2", "bot", "audienceGroup", &id, "updateDescription"],
        );
        self.send_empty(rb.json(request)).await
    }

    /// Deletes an audience.
    /// <https://developers.line.biz/en/reference/messaging-api/#delete-audience-group>
    pub async fn delete_audience_group(&self, audience_group_id: i64) -> Result<()> {
        let id = audience_group_id.to_string();
        let rb = self.request(
            Method::DELETE,
            Host::Api,
            &["v2", "bot", "audienceGroup", &id],
        );
        self.send_empty(rb).await
    }

    /// Gets audience data.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-audience-group>
    pub async fn get_audience_data(
        &self,
        audience_group_id: i64,
    ) -> Result<GetAudienceDataResponse> {
        let id = audience_group_id.to_string();
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "audienceGroup", &id]);
        self.send_json(rb).await
    }

    /// Gets data for more than one audience.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-audience-groups>
    pub async fn get_audience_groups(
        &self,
        params: &GetAudienceGroupsParams,
    ) -> Result<GetAudienceGroupsResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "audienceGroup", "list"],
            )
            .query(params);
        self.send_json(rb).await
    }

    /// Gets a shared audience (Business Manager).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-shared-audience>
    pub async fn get_shared_audience_data(
        &self,
        audience_group_id: i64,
    ) -> Result<GetSharedAudienceDataResponse> {
        let id = audience_group_id.to_string();
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "audienceGroup", "shared", &id],
        );
        self.send_json(rb).await
    }

    /// Gets a list of shared audiences (Business Manager).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-shared-audience-list>
    pub async fn get_shared_audience_groups(
        &self,
        params: &GetSharedAudienceGroupsParams,
    ) -> Result<GetSharedAudienceGroupsResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "audienceGroup", "shared", "list"],
            )
            .query(params);
        self.send_json(rb).await
    }
}

fn audience_file(file: &[u8]) -> Part {
    Part::bytes(file.to_vec())
        .file_name(AUDIENCE_FILE_NAME)
        .mime_str("text/plain")
        .expect("`text/plain` is a valid MIME type")
}

/// A user ID or IFA.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Audience {
    /// A user ID or IFA.
    pub id: Option<String>,
}

impl Audience {
    /// Creates an audience entry from a user ID or IFA.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
        }
    }
}

impl From<&str> for Audience {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Audience {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Request body to create an audience for uploading user IDs (by JSON).
/// <https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAudienceGroupRequest {
    /// The audience's name (max 120 characters, case-insensitive).
    pub description: Option<String>,
    /// `true` to specify recipients by IFAs, `false` or omitted for user IDs.
    pub is_ifa_audience: Option<bool>,
    /// Description registered for the job (`jobs[].description`).
    pub upload_description: Option<String>,
    /// Up to 10,000 user IDs or IFAs.
    pub audiences: Option<Vec<Audience>>,
}

impl CreateAudienceGroupRequest {
    /// Sets the audience's name (max 120 characters, case-insensitive).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether recipients are specified by IFAs (`true`) or user IDs (`false`).
    pub fn is_ifa_audience(mut self, is_ifa_audience: bool) -> Self {
        self.is_ifa_audience = Some(is_ifa_audience);
        self
    }

    /// Sets the description to register for the job.
    pub fn upload_description(mut self, upload_description: impl Into<String>) -> Self {
        self.upload_description = Some(upload_description.into());
        self
    }

    /// Sets the user IDs or IFAs (up to 10,000).
    pub fn audiences(mut self, audiences: impl IntoIterator<Item = impl Into<Audience>>) -> Self {
        self.audiences = Some(audiences.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body to add user IDs or IFAs to an audience for uploading user IDs (by JSON).
/// <https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAudienceToAudienceGroupRequest {
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Description registered for the job.
    pub upload_description: Option<String>,
    /// Up to 10,000 user IDs or IFAs.
    pub audiences: Option<Vec<Audience>>,
}

impl AddAudienceToAudienceGroupRequest {
    /// Sets the audience ID.
    pub fn audience_group_id(mut self, audience_group_id: i64) -> Self {
        self.audience_group_id = Some(audience_group_id);
        self
    }

    /// Sets the description to register for the job.
    pub fn upload_description(mut self, upload_description: impl Into<String>) -> Self {
        self.upload_description = Some(upload_description.into());
        self
    }

    /// Sets the user IDs or IFAs (up to 10,000).
    pub fn audiences(mut self, audiences: impl IntoIterator<Item = impl Into<Audience>>) -> Self {
        self.audiences = Some(audiences.into_iter().map(Into::into).collect());
        self
    }
}

/// Multipart request of [`LineClient::create_audience_for_uploading_user_ids`].
/// <https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group-by-file>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAudienceForUploadingUserIdsRequest {
    /// Text file with one user ID or IFA per line (sent as `text/plain`, up to 1,500,000 lines).
    pub file: Vec<u8>,
    /// The audience's name (max 120 characters, case-insensitive).
    pub description: Option<String>,
    /// `true` to specify recipients by IFAs, `false` or omitted for user IDs.
    pub is_ifa_audience: Option<bool>,
    /// Description registered for the job (`jobs[].description`).
    pub upload_description: Option<String>,
}

impl CreateAudienceForUploadingUserIdsRequest {
    /// Creates a request with the file of user IDs or IFAs.
    pub fn new(file: impl Into<Vec<u8>>) -> Self {
        Self {
            file: file.into(),
            description: None,
            is_ifa_audience: None,
            upload_description: None,
        }
    }

    /// Sets the audience's name (max 120 characters, case-insensitive).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether recipients are specified by IFAs (`true`) or user IDs (`false`).
    pub fn is_ifa_audience(mut self, is_ifa_audience: bool) -> Self {
        self.is_ifa_audience = Some(is_ifa_audience);
        self
    }

    /// Sets the description to register for the job.
    pub fn upload_description(mut self, upload_description: impl Into<String>) -> Self {
        self.upload_description = Some(upload_description.into());
        self
    }
}

/// Multipart request of [`LineClient::add_user_ids_to_audience`].
/// <https://developers.line.biz/en/reference/messaging-api/#update-upload-audience-group-by-file>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUserIdsToAudienceRequest {
    /// Text file with one user ID or IFA per line (sent as `text/plain`, up to 1,500,000 lines).
    pub file: Vec<u8>,
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Description registered for the job.
    pub upload_description: Option<String>,
}

impl AddUserIdsToAudienceRequest {
    /// Creates a request with the file of user IDs or IFAs.
    pub fn new(file: impl Into<Vec<u8>>) -> Self {
        Self {
            file: file.into(),
            audience_group_id: None,
            upload_description: None,
        }
    }

    /// Sets the audience ID.
    pub fn audience_group_id(mut self, audience_group_id: i64) -> Self {
        self.audience_group_id = Some(audience_group_id);
        self
    }

    /// Sets the description to register for the job.
    pub fn upload_description(mut self, upload_description: impl Into<String>) -> Self {
        self.upload_description = Some(upload_description.into());
        self
    }
}

/// Response of creating an audience for uploading user IDs.
/// <https://developers.line.biz/en/reference/messaging-api/#create-upload-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAudienceGroupResponse {
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Always `MESSAGING_API`.
    pub create_route: Option<AudienceGroupCreateRoute>,
    /// Audience group type.
    #[serde(rename = "type")]
    pub audience_group_type: Option<AudienceGroupType>,
    /// The audience's name.
    pub description: Option<String>,
    /// UNIX time in seconds.
    pub created: Option<i64>,
    /// Audience's update permission; audiences linked to the same channel are `READ_WRITE`.
    pub permission: Option<AudienceGroupPermission>,
    /// UNIX time in seconds. Only returned for specific audiences.
    pub expire_timestamp: Option<i64>,
    /// `true` if accounts are specified with IFAs, `false` (default) if with user IDs.
    pub is_ifa_audience: Option<bool>,
}

/// Request body to create an audience for click-based retargeting.
/// <https://developers.line.biz/en/reference/messaging-api/#create-click-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClickBasedAudienceGroupRequest {
    /// The audience's name (max 120 characters, case-insensitive).
    pub description: Option<String>,
    /// Request ID of a broadcast or narrowcast message sent in the past 60 days.
    pub request_id: Option<String>,
    /// URL clicked by the user. When omitted, users who clicked any URL are included.
    pub click_url: Option<String>,
}

impl CreateClickBasedAudienceGroupRequest {
    /// Sets the audience's name (max 120 characters, case-insensitive).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the request ID of a broadcast or narrowcast message sent in the past 60 days.
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Sets the URL clicked by the user (max 2,000 characters).
    pub fn click_url(mut self, click_url: impl Into<String>) -> Self {
        self.click_url = Some(click_url.into());
        self
    }
}

/// Response of creating an audience for click-based retargeting.
/// <https://developers.line.biz/en/reference/messaging-api/#create-click-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClickBasedAudienceGroupResponse {
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Audience group type.
    #[serde(rename = "type")]
    pub audience_group_type: Option<AudienceGroupType>,
    /// The audience's name.
    pub description: Option<String>,
    /// UNIX time in seconds.
    pub created: Option<i64>,
    /// The request ID that was specified when the audience was created.
    pub request_id: Option<String>,
    /// The URL that was specified when the audience was created.
    pub click_url: Option<String>,
    /// Always `MESSAGING_API`.
    pub create_route: Option<AudienceGroupCreateRoute>,
    /// Audience's update permission; audiences linked to the same channel are `READ_WRITE`.
    pub permission: Option<AudienceGroupPermission>,
    /// UNIX time in seconds. Only returned for specific audiences.
    pub expire_timestamp: Option<i64>,
    /// `true` if accounts are specified with IFAs, `false` (default) if with user IDs.
    pub is_ifa_audience: Option<bool>,
}

/// Request body to create an audience for impression-based retargeting.
/// <https://developers.line.biz/en/reference/messaging-api/#create-imp-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateImpBasedAudienceGroupRequest {
    /// The audience's name (1-120 characters, case-insensitive).
    pub description: Option<String>,
    /// Request ID of a broadcast or narrowcast message sent in the past 60 days.
    pub request_id: Option<String>,
}

impl CreateImpBasedAudienceGroupRequest {
    /// Sets the audience's name (1-120 characters, case-insensitive).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the request ID of a broadcast or narrowcast message sent in the past 60 days.
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
}

/// Response of creating an audience for impression-based retargeting.
/// <https://developers.line.biz/en/reference/messaging-api/#create-imp-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateImpBasedAudienceGroupResponse {
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Audience group type.
    #[serde(rename = "type")]
    pub audience_group_type: Option<AudienceGroupType>,
    /// The audience's name.
    pub description: Option<String>,
    /// UNIX time in seconds.
    pub created: Option<i64>,
    /// The request ID that was specified when the audience was created.
    pub request_id: Option<String>,
}

/// Request body to rename an audience.
/// <https://developers.line.biz/en/reference/messaging-api/#set-description-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UpdateAudienceGroupDescriptionRequest {
    /// The audience's name (1-120 characters, case-insensitive).
    pub description: Option<String>,
}

impl UpdateAudienceGroupDescriptionRequest {
    /// Creates a request with the audience's new name (1-120 characters, case-insensitive).
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: Some(description.into()),
        }
    }
}

/// Query parameters of [`LineClient::get_audience_groups`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAudienceGroupsParams {
    /// Page to return, starting at 1.
    pub page: u64,
    /// Partial, case-insensitive match on the audience's name.
    pub description: Option<String>,
    /// Status of the audiences to return; not used as a search criterion when omitted.
    pub status: Option<AudienceGroupStatus>,
    /// Audiences per page (default 20, max 40).
    pub size: Option<u64>,
    /// `true` (default): include public audiences of all channels linked to the same bot.
    pub includes_external_public_groups: Option<bool>,
    /// How the audiences were created; all audiences are included when omitted.
    pub create_route: Option<AudienceGroupCreateRoute>,
}

impl GetAudienceGroupsParams {
    /// Creates parameters for the given page (1 or higher).
    pub fn new(page: u64) -> Self {
        Self {
            page,
            description: None,
            status: None,
            size: None,
            includes_external_public_groups: None,
            create_route: None,
        }
    }

    /// Sets the audience name to filter by (partial, case-insensitive match).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the status of the audiences to return.
    pub fn status(mut self, status: AudienceGroupStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the number of audiences per page (default 20, max 40).
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets whether to include public audiences of all channels linked to the same bot (`true`, default) or only audiences of the same channel (`false`).
    pub fn includes_external_public_groups(
        mut self,
        includes_external_public_groups: bool,
    ) -> Self {
        self.includes_external_public_groups = Some(includes_external_public_groups);
        self
    }

    /// Sets how the audiences to return were created.
    pub fn create_route(mut self, create_route: AudienceGroupCreateRoute) -> Self {
        self.create_route = Some(create_route);
        self
    }
}

/// Query parameters of [`LineClient::get_shared_audience_groups`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedAudienceGroupsParams {
    /// Page to return, starting at 1.
    pub page: u64,
    /// Partial, case-insensitive match on the audience's name.
    pub description: Option<String>,
    /// Status of the audiences to return; not used as a search criterion when omitted.
    pub status: Option<AudienceGroupStatus>,
    /// Audiences per page (default 20, max 40).
    pub size: Option<u64>,
    /// How the audiences were created; all audiences are included when omitted.
    pub create_route: Option<AudienceGroupCreateRoute>,
    /// `true`: include audiences owned by LINE Official Account Manager.
    /// `false` (default): only audiences shared by Business Manager.
    pub includes_owned_audience_groups: Option<bool>,
}

impl GetSharedAudienceGroupsParams {
    /// Creates parameters for the given page (1 or higher).
    pub fn new(page: u64) -> Self {
        Self {
            page,
            description: None,
            status: None,
            size: None,
            create_route: None,
            includes_owned_audience_groups: None,
        }
    }

    /// Sets the audience name to filter by (partial, case-insensitive match).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the status of the audiences to return.
    pub fn status(mut self, status: AudienceGroupStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the number of audiences per page (default 20, max 40).
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets how the audiences to return were created.
    pub fn create_route(mut self, create_route: AudienceGroupCreateRoute) -> Self {
        self.create_route = Some(create_route);
        self
    }

    /// Sets whether to include audiences owned by LINE Official Account Manager (`true`) or only audiences shared by Business Manager (`false`, default).
    pub fn includes_owned_audience_groups(mut self, includes_owned_audience_groups: bool) -> Self {
        self.includes_owned_audience_groups = Some(includes_owned_audience_groups);
        self
    }
}

/// Audience data.
/// <https://developers.line.biz/en/reference/messaging-api/#get-audience-group>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAudienceDataResponse {
    /// Audience group.
    pub audience_group: Option<AudienceGroup>,
    /// Jobs adding user IDs or IFAs (max 50). Empty for audiences not created by upload.
    pub jobs: Option<Vec<AudienceGroupJob>>,
    /// Ad account.
    pub adaccount: Option<Adaccount>,
}

/// Shared audience data.
/// <https://developers.line.biz/en/reference/messaging-api/#get-shared-audience>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedAudienceDataResponse {
    /// Audience group.
    pub audience_group: Option<AudienceGroup>,
    /// Jobs adding user IDs or IFAs (max 50). Empty for audiences not created by upload.
    pub jobs: Option<Vec<AudienceGroupJob>>,
    /// Owner of this audience group.
    pub owner: Option<DetailedOwner>,
}

/// Ad account.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Adaccount {
    /// Ad account name.
    pub name: Option<String>,
}

/// Owner of a shared audience.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedOwner {
    /// Service where the audience was created (e.g. `lap`).
    pub service_type: Option<String>,
    /// Owner ID in the service.
    pub id: Option<String>,
    /// Owner account name.
    pub name: Option<String>,
}

/// Data for more than one audience.
/// <https://developers.line.biz/en/reference/messaging-api/#get-audience-groups>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAudienceGroupsResponse {
    /// Audience data; empty if no audiences match the filter.
    pub audience_groups: Option<Vec<AudienceGroup>>,
    /// `true` when this is not the last page.
    pub has_next_page: Option<bool>,
    /// Total number of audiences that can be returned with the filter.
    pub total_count: Option<u64>,
    /// Number of matching audiences with the `READ_WRITE` permission.
    pub read_write_audience_group_total_count: Option<u64>,
    /// The current page number.
    pub page: Option<u64>,
    /// The maximum number of audiences on the current page.
    pub size: Option<u64>,
}

/// Data for more than one shared audience.
/// <https://developers.line.biz/en/reference/messaging-api/#get-shared-audience-list>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedAudienceGroupsResponse {
    /// Audience data; empty if no audiences match the filter.
    pub audience_groups: Option<Vec<AudienceGroup>>,
    /// `true` when this is not the last page.
    pub has_next_page: Option<bool>,
    /// Total number of audiences that can be returned with the filter.
    pub total_count: Option<u64>,
    /// Number of matching audiences with the `READ_WRITE` permission.
    pub read_write_audience_group_total_count: Option<u64>,
    /// The current page number.
    pub page: Option<u64>,
    /// The maximum number of audiences on the current page.
    pub size: Option<u64>,
}

/// Audience group.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudienceGroup {
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// Audience group type.
    #[serde(rename = "type")]
    pub audience_group_type: Option<AudienceGroupType>,
    /// The audience's name.
    pub description: Option<String>,
    /// Audience status.
    pub status: Option<AudienceGroupStatus>,
    /// Reason the audience failed.
    pub failed_type: Option<AudienceGroupFailedType>,
    /// Number of users included in the audience.
    pub audience_count: Option<u64>,
    /// UNIX time in seconds.
    pub created: Option<i64>,
    /// Only included when the type is `CLICK` or `IMP`.
    pub request_id: Option<String>,
    /// Only included when the type is `CLICK` and a URL was specified.
    pub click_url: Option<String>,
    /// `true` if accounts are specified with IFAs, `false` if with user IDs.
    pub is_ifa_audience: Option<bool>,
    /// Audience's update permission.
    pub permission: Option<AudienceGroupPermission>,
    /// How the audience was created.
    pub create_route: Option<AudienceGroupCreateRoute>,
}

/// Job adding user IDs or IFAs to an audience.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudienceGroupJob {
    /// A job ID.
    pub audience_group_job_id: Option<i64>,
    /// An audience ID.
    pub audience_group_id: Option<i64>,
    /// The job's description.
    pub description: Option<String>,
    /// Job type.
    #[serde(rename = "type")]
    pub job_type: Option<AudienceGroupJobType>,
    /// Job status.
    pub job_status: Option<AudienceGroupJobStatus>,
    /// Reason the job failed.
    pub failed_type: Option<AudienceGroupJobFailedType>,
    /// Number of accounts added or removed.
    pub audience_count: Option<u64>,
    /// UNIX time in seconds.
    pub created: Option<i64>,
}

/// Audience group type.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupType {
    /// `UPLOAD`.
    Upload,
    /// `CLICK`.
    Click,
    /// `IMP`.
    Imp,
    /// `CHAT_TAG`.
    ChatTag,
    /// `FRIEND_PATH`.
    FriendPath,
    /// `RESERVATION`.
    Reservation,
    /// `APP_EVENT`.
    AppEvent,
    /// `VIDEO_VIEW`.
    VideoView,
    /// `WEBTRAFFIC`.
    Webtraffic,
    /// `IMAGE_CLICK`.
    ImageClick,
    /// `RICHMENU_IMP`.
    RichmenuImp,
    /// `RICHMENU_CLICK`.
    RichmenuClick,
    /// `POP_AD_IMP`.
    PopAdImp,
    /// `TRACKINGTAG_WEBTRAFFIC`.
    TrackingtagWebtraffic,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Audience status.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupStatus {
    /// `IN_PROGRESS`.
    InProgress,
    /// `READY`.
    Ready,
    /// `FAILED`.
    Failed,
    /// `EXPIRED`.
    Expired,
    /// `INACTIVE`.
    Inactive,
    /// `ACTIVATING`.
    Activating,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Reason an audience failed.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupFailedType {
    /// `AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT`.
    AudienceGroupAudienceInsufficient,
    /// `INTERNAL_ERROR`.
    InternalError,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// How the audience was created.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupCreateRoute {
    /// `OA_MANAGER`: created with LINE Official Account Manager.
    OaManager,
    /// `MESSAGING_API`: created with the Messaging API.
    MessagingApi,
    /// `POINT_AD`: created with LINE Points Ads (Japanese only).
    PointAd,
    /// `AD_MANAGER`: created with LINE Ads.
    AdManager,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Audience's update permission.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupPermission {
    /// `READ`: can use only.
    Read,
    /// `READ_WRITE`: can use and update.
    ReadWrite,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Audience job type.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupJobType {
    /// `DIFF_ADD`.
    DiffAdd,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Audience job status.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupJobStatus {
    /// `QUEUED`.
    Queued,
    /// `WORKING`.
    Working,
    /// `FINISHED`.
    Finished,
    /// `FAILED`.
    Failed,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Reason an audience job failed.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudienceGroupJobFailedType {
    /// `INTERNAL_ERROR`.
    InternalError,
    /// `AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT`.
    AudienceGroupAudienceInsufficient,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}
