mod common;

use common::{TOKEN, setup, setup_with_data_host};
use line_bot_messaging_api::Error;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{
    bearer_token, body_json, header_regex, method, path, query_param, query_param_is_missing,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Returns the body of the only request received by `server`.
async fn single_body(server: &MockServer) -> String {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    String::from_utf8(requests[0].body.clone()).unwrap()
}

fn text_part(name: &str, value: &str) -> String {
    format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n")
}

fn file_part(contents: &str) -> String {
    format!(
        "Content-Disposition: form-data; name=\"file\"; filename=\"audiences.txt\"\r\nContent-Type: text/plain\r\n\r\n{contents}\r\n"
    )
}

#[tokio::test]
async fn create_audience_group() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/audienceGroup/upload"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "description": "audienceGroupName_01",
            "isIfaAudience": false,
            "uploadDescription": "uploadDescription",
            "audiences": [{"id": "U1"}, {"id": "U2"}, {"id": "U3"}]
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "audienceGroupId": 1234567890123i64,
            "createRoute": "MESSAGING_API",
            "type": "UPLOAD",
            "description": "audienceGroupName_01",
            "created": 1613698278,
            "permission": "READ_WRITE",
            "expireTimestamp": 1629250278,
            "isIfaAudience": false
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = CreateAudienceGroupRequest::default()
        .description("audienceGroupName_01")
        .is_ifa_audience(false)
        .upload_description("uploadDescription")
        .audiences(vec![
            Audience::from("U1"),
            Audience::from(String::from("U2")),
            Audience::new("U3"),
        ]);
    let res = client.create_audience_group(&request).await.unwrap();
    assert_eq!(
        res,
        CreateAudienceGroupResponse {
            audience_group_id: Some(1234567890123),
            create_route: Some(AudienceGroupCreateRoute::MessagingApi),
            audience_group_type: Some(AudienceGroupType::Upload),
            description: Some("audienceGroupName_01".into()),
            created: Some(1613698278),
            permission: Some(AudienceGroupPermission::ReadWrite),
            expire_timestamp: Some(1629250278),
            is_ifa_audience: Some(false),
        }
    );
}

#[test]
fn empty_create_audience_group_request_serializes_to_empty_object() {
    assert_eq!(
        serde_json::to_value(CreateAudienceGroupRequest::default()).unwrap(),
        json!({})
    );
}

#[tokio::test]
async fn add_audience_to_audience_group() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/audienceGroup/upload"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "audienceGroupId": 4389303728991i64,
            "uploadDescription": "fileName",
            "audiences": [{"id": "U1"}, {"id": "U2"}]
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = AddAudienceToAudienceGroupRequest::default()
        .audience_group_id(4389303728991)
        .upload_description("fileName")
        .audiences(["U1", "U2"]);
    client
        .add_audience_to_audience_group(&request)
        .await
        .unwrap();
}

#[tokio::test]
async fn create_audience_for_uploading_user_ids_sends_multipart_to_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/audienceGroup/upload/byFile"))
        .and(bearer_token(TOKEN))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "audienceGroupId": 1234567890123i64,
            "createRoute": "MESSAGING_API",
            "type": "UPLOAD",
            "description": "audienceGroupName_01",
            "created": 1613700237,
            "permission": "READ_WRITE",
            "expireTimestamp": 1629252237,
            "isIfaAudience": false
        })))
        .expect(1)
        .mount(&data)
        .await;

    let request = CreateAudienceForUploadingUserIdsRequest::new("U1\nU2")
        .description("audienceGroupName_01")
        .is_ifa_audience(false)
        .upload_description("audience_list.txt");
    let res = client
        .create_audience_for_uploading_user_ids(&request)
        .await
        .unwrap();
    assert_eq!(res.audience_group_id, Some(1234567890123));
    assert_eq!(res.created, Some(1613700237));

    let body = single_body(&data).await;
    assert!(body.contains(&text_part("description", "audienceGroupName_01")));
    assert!(body.contains(&text_part("isIfaAudience", "false")));
    assert!(body.contains(&text_part("uploadDescription", "audience_list.txt")));
    assert!(body.contains(&file_part("U1\nU2")));
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_audience_for_uploading_user_ids_sends_only_the_file_by_default() {
    let (_api, data, client) = setup_with_data_host().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/audienceGroup/upload/byFile"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&data)
        .await;

    client
        .create_audience_for_uploading_user_ids(&CreateAudienceForUploadingUserIdsRequest::new(
            b"U1".to_vec(),
        ))
        .await
        .unwrap();

    let body = single_body(&data).await;
    assert_eq!(body.matches("Content-Disposition").count(), 1);
    assert!(body.contains(&file_part("U1")));
}

#[tokio::test]
async fn add_user_ids_to_audience_sends_multipart_to_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/audienceGroup/upload/byFile"))
        .and(bearer_token(TOKEN))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&data)
        .await;

    let request = AddUserIdsToAudienceRequest::new("U1\nU2")
        .audience_group_id(4389303728991)
        .upload_description("fileName");
    client.add_user_ids_to_audience(&request).await.unwrap();

    let body = single_body(&data).await;
    assert!(body.contains(&text_part("audienceGroupId", "4389303728991")));
    assert!(body.contains(&text_part("uploadDescription", "fileName")));
    assert!(body.contains(&file_part("U1\nU2")));
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn add_user_ids_to_audience_sends_only_the_file_by_default() {
    let (_api, data, client) = setup_with_data_host().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/audienceGroup/upload/byFile"))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&data)
        .await;

    client
        .add_user_ids_to_audience(&AddUserIdsToAudienceRequest::new("U1"))
        .await
        .unwrap();

    let body = single_body(&data).await;
    assert_eq!(body.matches("Content-Disposition").count(), 1);
    assert!(body.contains(&file_part("U1")));
}

#[tokio::test]
async fn create_click_based_audience_group() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/audienceGroup/click"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "description": "audienceGroupName_01",
            "requestId": "bb9744f9-47fa-4a29-941e-1234567890ab",
            "clickUrl": "https://developers.line.biz/"
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "audienceGroupId": 1234567890123i64,
            "createRoute": "MESSAGING_API",
            "type": "CLICK",
            "description": "audienceGroupName_01",
            "created": 1613705240,
            "permission": "READ_WRITE",
            "expireTimestamp": 1629257239,
            "isIfaAudience": false,
            "requestId": "bb9744f9-47fa-4a29-941e-1234567890ab",
            "clickUrl": "https://developers.line.biz/"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = CreateClickBasedAudienceGroupRequest::default()
        .description("audienceGroupName_01")
        .request_id("bb9744f9-47fa-4a29-941e-1234567890ab")
        .click_url("https://developers.line.biz/");
    let res = client
        .create_click_based_audience_group(&request)
        .await
        .unwrap();
    assert_eq!(
        res,
        CreateClickBasedAudienceGroupResponse {
            audience_group_id: Some(1234567890123),
            audience_group_type: Some(AudienceGroupType::Click),
            description: Some("audienceGroupName_01".into()),
            created: Some(1613705240),
            request_id: Some("bb9744f9-47fa-4a29-941e-1234567890ab".into()),
            click_url: Some("https://developers.line.biz/".into()),
            create_route: Some(AudienceGroupCreateRoute::MessagingApi),
            permission: Some(AudienceGroupPermission::ReadWrite),
            expire_timestamp: Some(1629257239),
            is_ifa_audience: Some(false),
        }
    );
}

#[tokio::test]
async fn create_imp_based_audience_group() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/audienceGroup/imp"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "description": "audienceGroupName_01",
            "requestId": "bb9744f9-47fa-4a29-941e-1234567890ab"
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "audienceGroupId": 1234567890123i64,
            "createRoute": "MESSAGING_API",
            "type": "IMP",
            "description": "audienceGroupName_01",
            "created": 1613707097,
            "permission": "READ_WRITE",
            "expireTimestamp": 1629259095,
            "isIfaAudience": false,
            "requestId": "bb9744f9-47fa-4a29-941e-1234567890ab"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = CreateImpBasedAudienceGroupRequest::default()
        .description("audienceGroupName_01")
        .request_id("bb9744f9-47fa-4a29-941e-1234567890ab");
    let res = client
        .create_imp_based_audience_group(&request)
        .await
        .unwrap();
    assert_eq!(
        res,
        CreateImpBasedAudienceGroupResponse {
            audience_group_id: Some(1234567890123),
            audience_group_type: Some(AudienceGroupType::Imp),
            description: Some("audienceGroupName_01".into()),
            created: Some(1613707097),
            request_id: Some("bb9744f9-47fa-4a29-941e-1234567890ab".into()),
        }
    );
}

#[tokio::test]
async fn update_audience_group_description() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path(
            "/v2/bot/audienceGroup/1234567890123/updateDescription",
        ))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({"description": "audienceGroupName"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .update_audience_group_description(
            1234567890123,
            &UpdateAudienceGroupDescriptionRequest::new("audienceGroupName"),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_audience_group() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/v2/bot/audienceGroup/1234567890123"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.delete_audience_group(1234567890123).await.unwrap();
}

fn uploading_audience_json() -> serde_json::Value {
    json!({
        "audienceGroupId": 1234567890123i64,
        "createRoute": "OA_MANAGER",
        "type": "UPLOAD",
        "description": "audienceGroupName_01",
        "status": "READY",
        "audienceCount": 1887,
        "created": 1608617466,
        "permission": "READ",
        "isIfaAudience": false,
        "expireTimestamp": 1624342266
    })
}

fn uploading_jobs_json() -> serde_json::Value {
    json!([{
        "audienceGroupJobId": 12345678,
        "audienceGroupId": 1234567890123i64,
        "description": "audience_list.txt",
        "type": "DIFF_ADD",
        "status": "FINISHED",
        "failedType": "AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT",
        "audienceCount": 0,
        "created": 1608617472,
        "jobStatus": "FINISHED"
    }])
}

fn expected_uploading_audience() -> AudienceGroup {
    AudienceGroup {
        audience_group_id: Some(1234567890123),
        audience_group_type: Some(AudienceGroupType::Upload),
        description: Some("audienceGroupName_01".into()),
        status: Some(AudienceGroupStatus::Ready),
        failed_type: None,
        audience_count: Some(1887),
        created: Some(1608617466),
        request_id: None,
        click_url: None,
        is_ifa_audience: Some(false),
        permission: Some(AudienceGroupPermission::Read),
        create_route: Some(AudienceGroupCreateRoute::OaManager),
    }
}

fn expected_uploading_jobs() -> Vec<AudienceGroupJob> {
    vec![AudienceGroupJob {
        audience_group_job_id: Some(12345678),
        audience_group_id: Some(1234567890123),
        description: Some("audience_list.txt".into()),
        job_type: Some(AudienceGroupJobType::DiffAdd),
        job_status: Some(AudienceGroupJobStatus::Finished),
        failed_type: Some(AudienceGroupJobFailedType::AudienceGroupAudienceInsufficient),
        audience_count: Some(0),
        created: Some(1608617472),
    }]
}

#[tokio::test]
async fn get_audience_data() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/1234567890123"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "audienceGroup": uploading_audience_json(),
            "jobs": uploading_jobs_json(),
            "adaccount": {"name": "Ad Account Name"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_audience_data(1234567890123).await.unwrap();
    assert_eq!(
        res,
        GetAudienceDataResponse {
            audience_group: Some(expected_uploading_audience()),
            jobs: Some(expected_uploading_jobs()),
            adaccount: Some(Adaccount {
                name: Some("Ad Account Name".into()),
            }),
        }
    );
}

#[tokio::test]
async fn get_audience_groups_with_all_params() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/list"))
        .and(bearer_token(TOKEN))
        .and(query_param("page", "1"))
        .and(query_param("description", "audienceGroupName"))
        .and(query_param("status", "IN_PROGRESS"))
        .and(query_param("size", "40"))
        .and(query_param("includesExternalPublicGroups", "false"))
        .and(query_param("createRoute", "MESSAGING_API"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "audienceGroups": [
                {
                    "audienceGroupId": 1234567890123i64,
                    "createRoute": "OA_MANAGER",
                    "type": "CLICK",
                    "description": "audienceGroupName_01",
                    "status": "IN_PROGRESS",
                    "audienceCount": 8619,
                    "created": 1611114828,
                    "permission": "READ",
                    "isIfaAudience": false,
                    "expireTimestamp": 1626753228,
                    "requestId": "c10c3d86-f565-...",
                    "clickUrl": "https://example.com/"
                },
                {
                    "audienceGroupId": 2345678901234i64,
                    "createRoute": "AD_MANAGER",
                    "type": "APP_EVENT",
                    "description": "audienceGroupName_02",
                    "status": "READY",
                    "audienceCount": 3368,
                    "created": 1608619802,
                    "permission": "READ",
                    "activated": 1610068515,
                    "inactiveTimestamp": 1625620516,
                    "isIfaAudience": false
                }
            ],
            "hasNextPage": false,
            "totalCount": 2,
            "readWriteAudienceGroupTotalCount": 0,
            "size": 40,
            "page": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let params = GetAudienceGroupsParams::new(1)
        .description("audienceGroupName")
        .status(AudienceGroupStatus::InProgress)
        .size(40)
        .includes_external_public_groups(false)
        .create_route(AudienceGroupCreateRoute::MessagingApi);
    let res = client.get_audience_groups(&params).await.unwrap();
    let groups = res.audience_groups.unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].request_id.as_deref(), Some("c10c3d86-f565-..."));
    assert_eq!(groups[0].click_url.as_deref(), Some("https://example.com/"));
    assert_eq!(
        groups[1].create_route,
        Some(AudienceGroupCreateRoute::AdManager)
    );
    assert_eq!(
        groups[1].audience_group_type,
        Some(AudienceGroupType::AppEvent)
    );
    assert_eq!(res.has_next_page, Some(false));
    assert_eq!(res.total_count, Some(2));
    assert_eq!(res.read_write_audience_group_total_count, Some(0));
    assert_eq!(res.size, Some(40));
    assert_eq!(res.page, Some(1));
}

#[tokio::test]
async fn get_audience_groups_sends_only_page_by_default() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "audienceGroups": [],
            "hasNextPage": false,
            "totalCount": 0,
            "readWriteAudienceGroupTotalCount": 0,
            "size": 40,
            "page": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_audience_groups(&GetAudienceGroupsParams::new(1))
        .await
        .unwrap();
    assert_eq!(res.audience_groups, Some(vec![]));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].url.query(), Some("page=1"));
}

#[tokio::test]
async fn get_shared_audience_data() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/shared/1234567890123"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "audienceGroup": uploading_audience_json(),
            "jobs": uploading_jobs_json(),
            "owner": {"serviceType": "lap", "id": "A012345678", "name": "Ad Account Name"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_shared_audience_data(1234567890123)
        .await
        .unwrap();
    assert_eq!(
        res,
        GetSharedAudienceDataResponse {
            audience_group: Some(expected_uploading_audience()),
            jobs: Some(expected_uploading_jobs()),
            owner: Some(DetailedOwner {
                service_type: Some("lap".into()),
                id: Some("A012345678".into()),
                name: Some("Ad Account Name".into()),
            }),
        }
    );
}

#[tokio::test]
async fn get_shared_audience_groups_with_all_params() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/shared/list"))
        .and(bearer_token(TOKEN))
        .and(query_param("page", "2"))
        .and(query_param("description", "name"))
        .and(query_param("status", "READY"))
        .and(query_param("size", "20"))
        .and(query_param("createRoute", "OA_MANAGER"))
        .and(query_param("includesOwnedAudienceGroups", "true"))
        .and(query_param_is_missing("includesExternalPublicGroups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "audienceGroups": [],
            "hasNextPage": false,
            "totalCount": 0,
            "readWriteAudienceGroupTotalCount": 0,
            "size": 20,
            "page": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let params = GetSharedAudienceGroupsParams::new(2)
        .description("name")
        .status(AudienceGroupStatus::Ready)
        .size(20)
        .create_route(AudienceGroupCreateRoute::OaManager)
        .includes_owned_audience_groups(true);
    let res = client.get_shared_audience_groups(&params).await.unwrap();
    assert_eq!(
        res,
        GetSharedAudienceGroupsResponse {
            audience_groups: Some(vec![]),
            has_next_page: Some(false),
            total_count: Some(0),
            read_write_audience_group_total_count: Some(0),
            page: Some(2),
            size: Some(20),
        }
    );
}

#[test]
fn audience_enums_cover_spec_and_unknown_values() {
    let types: Vec<AudienceGroupType> = serde_json::from_value(json!([
        "UPLOAD",
        "CLICK",
        "IMP",
        "CHAT_TAG",
        "FRIEND_PATH",
        "RESERVATION",
        "APP_EVENT",
        "VIDEO_VIEW",
        "WEBTRAFFIC",
        "IMAGE_CLICK",
        "RICHMENU_IMP",
        "RICHMENU_CLICK",
        "POP_AD_IMP",
        "TRACKINGTAG_WEBTRAFFIC",
        "NEW_TYPE"
    ]))
    .unwrap();
    assert_eq!(
        types,
        [
            AudienceGroupType::Upload,
            AudienceGroupType::Click,
            AudienceGroupType::Imp,
            AudienceGroupType::ChatTag,
            AudienceGroupType::FriendPath,
            AudienceGroupType::Reservation,
            AudienceGroupType::AppEvent,
            AudienceGroupType::VideoView,
            AudienceGroupType::Webtraffic,
            AudienceGroupType::ImageClick,
            AudienceGroupType::RichmenuImp,
            AudienceGroupType::RichmenuClick,
            AudienceGroupType::PopAdImp,
            AudienceGroupType::TrackingtagWebtraffic,
            AudienceGroupType::Unknown,
        ]
    );

    let statuses: Vec<AudienceGroupStatus> = serde_json::from_value(json!([
        "IN_PROGRESS",
        "READY",
        "FAILED",
        "EXPIRED",
        "INACTIVE",
        "ACTIVATING",
        "NEW"
    ]))
    .unwrap();
    assert_eq!(
        statuses,
        [
            AudienceGroupStatus::InProgress,
            AudienceGroupStatus::Ready,
            AudienceGroupStatus::Failed,
            AudienceGroupStatus::Expired,
            AudienceGroupStatus::Inactive,
            AudienceGroupStatus::Activating,
            AudienceGroupStatus::Unknown,
        ]
    );

    let failed: Vec<Option<AudienceGroupFailedType>> = serde_json::from_value(json!([
        "AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT",
        "INTERNAL_ERROR",
        "NEW",
        null
    ]))
    .unwrap();
    assert_eq!(
        failed,
        [
            Some(AudienceGroupFailedType::AudienceGroupAudienceInsufficient),
            Some(AudienceGroupFailedType::InternalError),
            Some(AudienceGroupFailedType::Unknown),
            None,
        ]
    );

    let routes: Vec<AudienceGroupCreateRoute> = serde_json::from_value(json!([
        "OA_MANAGER",
        "MESSAGING_API",
        "POINT_AD",
        "AD_MANAGER",
        "NEW"
    ]))
    .unwrap();
    assert_eq!(
        routes,
        [
            AudienceGroupCreateRoute::OaManager,
            AudienceGroupCreateRoute::MessagingApi,
            AudienceGroupCreateRoute::PointAd,
            AudienceGroupCreateRoute::AdManager,
            AudienceGroupCreateRoute::Unknown,
        ]
    );

    let permissions: Vec<AudienceGroupPermission> =
        serde_json::from_value(json!(["READ", "READ_WRITE", "NEW"])).unwrap();
    assert_eq!(
        permissions,
        [
            AudienceGroupPermission::Read,
            AudienceGroupPermission::ReadWrite,
            AudienceGroupPermission::Unknown,
        ]
    );

    let job_types: Vec<AudienceGroupJobType> =
        serde_json::from_value(json!(["DIFF_ADD", "DIFF_REMOVE"])).unwrap();
    assert_eq!(
        job_types,
        [AudienceGroupJobType::DiffAdd, AudienceGroupJobType::Unknown]
    );

    let job_statuses: Vec<AudienceGroupJobStatus> =
        serde_json::from_value(json!(["QUEUED", "WORKING", "FINISHED", "FAILED", "NEW"])).unwrap();
    assert_eq!(
        job_statuses,
        [
            AudienceGroupJobStatus::Queued,
            AudienceGroupJobStatus::Working,
            AudienceGroupJobStatus::Finished,
            AudienceGroupJobStatus::Failed,
            AudienceGroupJobStatus::Unknown,
        ]
    );

    let job_failed: Vec<AudienceGroupJobFailedType> = serde_json::from_value(json!([
        "INTERNAL_ERROR",
        "AUDIENCE_GROUP_AUDIENCE_INSUFFICIENT",
        "NEW"
    ]))
    .unwrap();
    assert_eq!(
        job_failed,
        [
            AudienceGroupJobFailedType::InternalError,
            AudienceGroupJobFailedType::AudienceGroupAudienceInsufficient,
            AudienceGroupJobFailedType::Unknown,
        ]
    );
}

#[tokio::test]
async fn audience_error_response() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/audienceGroup/1"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-400")
                .set_body_json(json!({
                    "message": "audience group not found",
                    "details": [{"message": "AUDIENCE_GROUP_NOT_FOUND"}]
                })),
        )
        .mount(&server)
        .await;

    let err = client.get_audience_data(1).await.unwrap_err();
    let Error::Api(api) = err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-400"));
    assert_eq!(api.body.message, "audience group not found");
    assert_eq!(
        api.body.details[0].message.as_deref(),
        Some("AUDIENCE_GROUP_NOT_FOUND")
    );
}
