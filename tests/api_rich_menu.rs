mod common;

use common::{TOKEN, setup, setup_with_data_host};
use line_bot_messaging_api::Error;
use line_bot_messaging_api::api::*;
use line_bot_messaging_api::message::{Action, PostbackAction, UriAction};
use serde_json::json;
use wiremock::matchers::{bearer_token, body_bytes, body_json, header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn rich_menu_json() -> serde_json::Value {
    json!({
        "size": {"width": 2500, "height": 1686},
        "selected": false,
        "name": "nice rich menu",
        "chatBarText": "click",
        "areas": [
            {
                "bounds": {"x": 0, "y": 0, "width": 1250, "height": 1686},
                "action": {"type": "postback", "label": "Buy", "data": "action=buy&itemid=123"}
            },
            {
                "bounds": {"x": 1250, "y": 0, "width": 1250, "height": 1686},
                "action": {"type": "uri", "label": "Site", "uri": "https://example.com/"}
            }
        ]
    })
}

fn rich_menu_request() -> RichMenuRequest {
    RichMenuRequest::new(
        RichMenuSize::new(2500, 1686),
        false,
        "nice rich menu",
        "click",
        [
            RichMenuArea::new(
                RichMenuBounds::new(0, 0, 1250, 1686),
                PostbackAction::new("action=buy&itemid=123").label("Buy"),
            ),
            RichMenuArea::new(
                RichMenuBounds::new(1250, 0, 1250, 1686),
                UriAction::new("https://example.com/").label("Site"),
            ),
        ],
    )
}

#[test]
fn rich_menu_request_serializes() {
    assert_eq!(
        serde_json::to_value(rich_menu_request()).unwrap(),
        rich_menu_json()
    );
}

#[tokio::test]
async fn create_rich_menu() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu"))
        .and(bearer_token(TOKEN))
        .and(body_json(rich_menu_json()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"richMenuId": "richmenu-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.create_rich_menu(&rich_menu_request()).await.unwrap();
    assert_eq!(res.rich_menu_id, "richmenu-1");
}

#[tokio::test]
async fn validate_rich_menu_object() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/validate"))
        .and(bearer_token(TOKEN))
        .and(body_json(rich_menu_json()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .validate_rich_menu_object(&rich_menu_request())
        .await
        .unwrap();
}

#[tokio::test]
async fn get_rich_menu_image_uses_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/richmenu%2F1/content"))
        .and(bearer_token(TOKEN))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(vec![0x89, 0x50, 0x4e, 0x47], "image/png"),
        )
        .expect(1)
        .mount(&data)
        .await;

    let content = client.get_rich_menu_image("richmenu/1").await.unwrap();
    assert_eq!(content.data, vec![0x89, 0x50, 0x4e, 0x47]);
    assert_eq!(content.content_type.as_deref(), Some("image/png"));
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn set_rich_menu_image_uses_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/richmenu-1/content"))
        .and(bearer_token(TOKEN))
        .and(header("content-type", "image/jpeg"))
        .and(body_bytes(vec![0xff, 0xd8, 0xff]))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&data)
        .await;

    client
        .set_rich_menu_image("richmenu-1", vec![0xff, 0xd8, 0xff], "image/jpeg")
        .await
        .unwrap();
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn get_rich_menu() {
    let (server, client) = setup().await;
    let mut body = rich_menu_json();
    body["richMenuId"] = json!("richmenu-1");
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/richmenu%201"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_rich_menu("richmenu 1").await.unwrap();
    let request = rich_menu_request();
    assert_eq!(
        res,
        RichMenuResponse {
            rich_menu_id: "richmenu-1".into(),
            size: request.size,
            selected: request.selected,
            name: request.name,
            chat_bar_text: request.chat_bar_text,
            areas: request.areas,
        }
    );
    assert_eq!(
        res.areas[0].action,
        Action::from(PostbackAction::new("action=buy&itemid=123").label("Buy"))
    );
}

#[tokio::test]
async fn delete_rich_menu() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/v2/bot/richmenu/richmenu-1"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.delete_rich_menu("richmenu-1").await.unwrap();
}

#[tokio::test]
async fn get_rich_menu_list() {
    let (server, client) = setup().await;
    let mut menu = rich_menu_json();
    menu["richMenuId"] = json!("richmenu-1");
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/list"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"richmenus": [menu]})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_rich_menu_list().await.unwrap();
    assert_eq!(res.richmenus.len(), 1);
    assert_eq!(res.richmenus[0].rich_menu_id, "richmenu-1");
    assert_eq!(res.richmenus[0].size, RichMenuSize::new(2500, 1686));
    assert_eq!(
        res.richmenus[0].areas[1].bounds,
        RichMenuBounds::new(1250, 0, 1250, 1686)
    );
}

#[tokio::test]
async fn set_default_rich_menu() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/user/all/richmenu/richmenu-1"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.set_default_rich_menu("richmenu-1").await.unwrap();
}

#[tokio::test]
async fn get_default_rich_menu_id() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/user/all/richmenu"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"richMenuId": "richmenu-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_default_rich_menu_id().await.unwrap();
    assert_eq!(
        res,
        RichMenuIdResponse {
            rich_menu_id: "richmenu-1".into()
        }
    );
}

#[tokio::test]
async fn cancel_default_rich_menu() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/v2/bot/user/all/richmenu"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.cancel_default_rich_menu().await.unwrap();
}

#[tokio::test]
async fn create_rich_menu_alias() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/alias"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "richMenuAliasId": "richmenu-alias-a",
            "richMenuId": "richmenu-1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .create_rich_menu_alias(&CreateRichMenuAliasRequest::new(
            "richmenu-alias-a",
            "richmenu-1",
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn get_rich_menu_alias() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/alias/richmenu-alias-a"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "richMenuAliasId": "richmenu-alias-a",
            "richMenuId": "richmenu-88c05ef6921ae53f8b58a25f3a65faf7"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_rich_menu_alias("richmenu-alias-a")
        .await
        .unwrap();
    assert_eq!(
        res,
        RichMenuAliasResponse {
            rich_menu_alias_id: "richmenu-alias-a".into(),
            rich_menu_id: "richmenu-88c05ef6921ae53f8b58a25f3a65faf7".into(),
        }
    );
}

#[tokio::test]
async fn update_rich_menu_alias() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/alias/richmenu-alias-a"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({"richMenuId": "richmenu-2"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .update_rich_menu_alias(
            "richmenu-alias-a",
            &UpdateRichMenuAliasRequest::new("richmenu-2"),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_rich_menu_alias() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/v2/bot/richmenu/alias/richmenu-alias-a"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .delete_rich_menu_alias("richmenu-alias-a")
        .await
        .unwrap();
}

#[tokio::test]
async fn get_rich_menu_alias_list() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/alias/list"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "aliases": [
                {"richMenuAliasId": "richmenu-alias-a", "richMenuId": "richmenu-862e6ad6c267d2ddf3f42bc78554f6a4"},
                {"richMenuAliasId": "richmenu-alias-b", "richMenuId": "richmenu-88c05ef6921ae53f8b58a25f3a65faf7"}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_rich_menu_alias_list().await.unwrap();
    assert_eq!(res.aliases.len(), 2);
    assert_eq!(res.aliases[1].rich_menu_alias_id, "richmenu-alias-b");
}

#[tokio::test]
async fn get_rich_menu_id_of_user() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/user/U1234/richmenu"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"richMenuId": "richmenu-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_rich_menu_id_of_user("U1234").await.unwrap();
    assert_eq!(res.rich_menu_id, "richmenu-1");
}

#[tokio::test]
async fn unlink_rich_menu_id_from_user() {
    let (server, client) = setup().await;
    Mock::given(method("DELETE"))
        .and(path("/v2/bot/user/U1234/richmenu"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.unlink_rich_menu_id_from_user("U1234").await.unwrap();
}

#[tokio::test]
async fn link_rich_menu_id_to_user() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/user/U1234/richmenu/richmenu-1"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .link_rich_menu_id_to_user("U1234", "richmenu-1")
        .await
        .unwrap();
}

#[tokio::test]
async fn link_rich_menu_id_to_users() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/bulk/link"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "richMenuId": "richmenu-1",
            "userIds": ["U1", "U2"]
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .link_rich_menu_id_to_users(&RichMenuBulkLinkRequest::new("richmenu-1", ["U1", "U2"]))
        .await
        .unwrap();
}

#[tokio::test]
async fn unlink_rich_menu_id_from_users() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/bulk/unlink"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({"userIds": ["U1", "U2"]})))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .unlink_rich_menu_id_from_users(&RichMenuBulkUnlinkRequest::new(vec![
            String::from("U1"),
            String::from("U2"),
        ]))
        .await
        .unwrap();
}

fn batch_request() -> RichMenuBatchRequest {
    RichMenuBatchRequest::new([
        RichMenuBatchOperation::from(RichMenuBatchLinkOperation::new("richmenu-a", "richmenu-b")),
        RichMenuBatchUnlinkOperation::new("richmenu-c").into(),
        RichMenuBatchOperation::UnlinkAll,
    ])
    .resume_request_key("key-1")
}

fn batch_json() -> serde_json::Value {
    json!({
        "operations": [
            {"type": "link", "from": "richmenu-a", "to": "richmenu-b"},
            {"type": "unlink", "from": "richmenu-c"},
            {"type": "unlinkAll"}
        ],
        "resumeRequestKey": "key-1"
    })
}

#[test]
fn batch_request_without_resume_key_omits_it() {
    assert_eq!(
        serde_json::to_value(RichMenuBatchRequest::new([
            RichMenuBatchOperation::UnlinkAll
        ]))
        .unwrap(),
        json!({"operations": [{"type": "unlinkAll"}]})
    );
}

#[tokio::test]
async fn rich_menu_batch() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/batch"))
        .and(bearer_token(TOKEN))
        .and(body_json(batch_json()))
        .respond_with(
            ResponseTemplate::new(202)
                .insert_header("x-line-request-id", "batch-req-1")
                .set_body_json(json!({})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = client.rich_menu_batch(&batch_request()).await.unwrap();
    assert_eq!(response.request_id.as_deref(), Some("batch-req-1"));
}

#[tokio::test]
async fn validate_rich_menu_batch_request() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/richmenu/validate/batch"))
        .and(bearer_token(TOKEN))
        .and(body_json(batch_json()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client
        .validate_rich_menu_batch_request(&batch_request())
        .await
        .unwrap();
}

#[tokio::test]
async fn get_rich_menu_batch_progress() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/progress/batch"))
        .and(query_param("requestId", "req-1"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "phase": "succeeded",
            "acceptedTime": "2023-06-08T10:15:30.121Z",
            "completedTime": "2023-06-08T10:15:31.121Z"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_rich_menu_batch_progress("req-1").await.unwrap();
    assert_eq!(
        res,
        RichMenuBatchProgressResponse {
            phase: RichMenuBatchProgressPhase::Succeeded,
            accepted_time: "2023-06-08T10:15:30.121Z".into(),
            completed_time: Some("2023-06-08T10:15:31.121Z".into()),
        }
    );
}

#[test]
fn batch_progress_phase_decodes_all_values() {
    let phases: Vec<RichMenuBatchProgressPhase> =
        serde_json::from_value(json!(["ongoing", "succeeded", "failed", "paused"])).unwrap();
    assert_eq!(
        phases,
        [
            RichMenuBatchProgressPhase::Ongoing,
            RichMenuBatchProgressPhase::Succeeded,
            RichMenuBatchProgressPhase::Failed,
            RichMenuBatchProgressPhase::Unknown,
        ]
    );
}

#[tokio::test]
async fn rich_menu_error_response() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/richmenu/richmenu-404"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("x-line-request-id", "req-404")
                .set_body_json(json!({
                    "message": "Not found",
                    "details": [{"message": "rich menu not found", "property": "richMenuId"}]
                })),
        )
        .mount(&server)
        .await;

    let err = client.get_rich_menu("richmenu-404").await.unwrap_err();
    let Error::Api(api) = err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 404);
    assert_eq!(api.request_id.as_deref(), Some("req-404"));
    assert_eq!(api.body.message, "Not found");
    assert_eq!(
        api.body.details[0].message.as_deref(),
        Some("rich menu not found")
    );
    assert_eq!(api.body.details[0].property.as_deref(), Some("richMenuId"));
}
