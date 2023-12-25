use line_bot_messaging_api::action::{LineActionUri, LineMessageActionCamera};
use line_bot_messaging_api::message::{
    LineMessageFlex, LineMessageImage, LineMessageImageMap, LineMessageImageMapActionUri,
    LineMessageImageMapArea, LineMessageImageMapBaseSize, LineMessageTemplate,
    LineMessageTemplateButton, LineMessageTemplateCarousel, LineMessageTemplateCarouselColumn,
    LineMessageTemplateCarouselImage, LineMessageTemplateCarouselImageColumn,
    LineMessageTemplateConfirm, LineMessageText, LineMessageVideo, LineMessagesBuilder,
};
use line_bot_messaging_api::{LineClient, LineError};
use serde_json::{json, Value};

fn get_client() -> LineClient {
    let token = env!("TOKEN");
    LineClient::new(token)
}

#[tokio::test]
async fn test_auth() {
    // let token = env!("TOKEN");
    let token = "as";
    let client = LineClient::new(token);

    let res = client.bot_info().await;

    let error = match res.err() {
        None => {
            assert!(false, "認証エラーが発生してません");
            return;
        }
        Some(v) => v,
    };
    let error = match error {
        LineError::ApiError(v) => v,
        _ => {
            assert!(false);
            return;
        }
    };
    assert_eq!(error.status, 401);
    assert_eq!(error.error.message, "Authentication failed. Confirm that the access token in the authorization header is valid.");
}

#[tokio::test]
async fn test_error_response() {
    let client = get_client();
    let a = json!({
        "size": {
            "widtha": 2500,
        },
    });
    let error = match client.rich_menu_create(a).await {
        Ok(_) => {
            assert!(false, "エラーが発生してません");
            return;
        }
        Err(v) => v,
    };
    error.api_error().unwrap().debug_print();
}

#[tokio::test]
async fn test_profile() {
    let user_id = "";
    let client = get_client();

    if let Ok(res) = client.profile(user_id).await {
        assert_eq!(res.language, Some("ja".to_string()));
    } else {
        assert!(false);
    };
}

#[tokio::test]
async fn test_リッチメニュー正常ケーステスト() {
    println!("リッチメニュー正常ケーステスト");
    let client = get_client();
    let user_id = "";

    let mut delete_ids = vec![];
    if let Ok(res) = client.rich_menu_list().await {
        println!("rich_menu_list");
        for menu in res.rich_menus {
            println!(" {:?}", menu);
            if menu.name.contains("Nice rich menu") {
                delete_ids.push(menu.rich_menu_id);
            }
        }
    } else {
        assert!(false);
    };
    if let Ok(res) = client.rich_menu_validate_object(get_test_rich_menu()).await {
        println!("{:?}", res);
    } else {
        assert!(false);
    };
    let rich = match client.rich_menu_create(get_test_rich_menu()).await {
        Ok(v) => v,
        Err(_) => {
            assert!(false);
            return;
        }
    };
    let rich_menu_id = rich.rich_menu_id;

    let res = client.rich_menu_get(rich_menu_id.as_str()).await;
    println!("{:?}", res);
    return;

    let file = std::fs::read("./test_line.jpg").unwrap();
    let res = client
        .rich_menu_content_upload(rich_menu_id.as_str(), file)
        .await;
    println!("{:?}", res);
    // delete_ids.push(rich_menu_id.clone());

    if let Ok(res) = client
        .rich_menu_set_default_menu(rich_menu_id.as_str())
        .await
    {
        println!("rich_menu_set_default");
    } else {
        assert!(false);
    };

    // if let Ok(res) = client.rich_menu_get_default_menu_id().await {
    //     assert_eq!(res.rich_menu_id, rich_menu_id);
    // } else {
    //     assert!(false);
    // };
    // if let Ok(res) = client.rich_menu_user_get_user_rich_menu_id(user_id).await {
    //     assert_eq!(res.rich_menu_id, rich_menu_id);
    // } else {
    //     assert!(false);
    // };

    // println!("{:?}",client.rich_menu_user_link_menu(rich_menu_id.as_str(),user_id).await);
    println!(
        "{:?}",
        client.rich_menu_user_get_user_rich_menu_id(user_id).await
    );

    if let Ok(res) = client.rich_menu_delete_default_menu().await {
    } else {
        assert!(false);
    };

    let rich = client
        .rich_menu_content_download(rich_menu_id.as_str())
        .await;
    //
    // let content = rich.unwrap();
    // let mut file = std::fs::File::create("./test_download.jpg").unwrap();
    // file.write(&content).unwrap();
    // file.flush().unwrap();
    for rich_menu_id in delete_ids {
        if let Ok(res) = client.rich_menu_delete(&rich_menu_id).await {
            println!("{:?}", res);
        } else {
            assert!(false);
        };
    }
}

fn get_test_rich_menu() -> Value {
    json!({
        "size": {
          "width": 2500,
          "height": 1686
        },
        "selected": false,
        "name": "Nice rich menu",
        "chatBarText": "Tap to open",
        "areas": [
          {
            "bounds": {
              "x": 0,
              "y": 0,
              "width": 2500,
              "height": 1686
            },
            "action": {
              "type": "postback",
              "data": "action=buy&itemid=123"
            }
          }
       ]
    })
}

#[tokio::test]
async fn test_配信正常ケーステスト() {
    println!("配信正常ケーステスト");
    let client = get_client();
    let user_id = "";

    let test_thumbnail = "";
    let test_mp4 = "https://www.home-movie.biz/mov/hts-samp001.mp4";
    let test_url = "https://google.com";
    let test_image =
        "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
    // let msg = LineMessageText::new("テスト".to_string());
    // let location = LineMessageLocation::new("テスト".to_string(), "東京都世田谷区".to_string(), 35.6440304,139.6752379);
    // let sticker = LineMessageSticker::new("789","10855");
    // let image = LineMessageImage::new(test_image_url,test_image_url);
    //
    // let mut image = LineMessageImageMap::new(
    //     test_image_url,
    //     "test",
    //     LineMessageImageMapBaseSize::new(1040, 1040),
    // );
    //
    // let uri = LineMessageImageMapActionUri::new("https://google.com",LineMessageImageMapArea::new(0,0,1040,1040));
    // image.append_action(uri);
    //
    //
    //
    // let mut builder = LineMessagesBuilder::new();
    // builder.append(image);

    // テンプレート_ボタン
    let mut builder = LineMessagesBuilder::new();

    let mut template_button = LineMessageTemplateButton::new("sub_title");
    template_button.set_title("title");
    template_button.set_image_aspect_ratio("square");
    template_button.set_image_size("contain");
    template_button.set_thumbnail_image_url(test_image);
    template_button.set_default_action(LineActionUri::new("default", test_url));
    template_button.append_action(LineActionUri::new("hoge", test_url));
    template_button.append_action(LineActionUri::new("hoge2", test_url));
    template_button.append_action(LineActionUri::new("hoge3", test_url));
    let mut template = LineMessageTemplate::new("alt", template_button);
    builder.append(template);

    // テンプレート_確認
    let mut builder = LineMessagesBuilder::new();
    let mut template_confirm = LineMessageTemplateConfirm::new("sub_title");
    template_confirm.append_action(LineMessageActionCamera::new("aaaaaaaaaaaaaaaaaaaa"));
    template_confirm.append_action(LineActionUri::new("hoge2", test_url));
    let mut template = LineMessageTemplate::new("alt", template_confirm);
    builder.append(template);

    // テンプレート_カルーセル
    let mut builder = LineMessagesBuilder::new();
    let mut template_confirm = LineMessageTemplateCarousel::new(vec![]);

    let mut col1 = LineMessageTemplateCarouselColumn::new("col1");
    col1.append_action(LineActionUri::new("col1_url", test_url));
    let mut col2 = LineMessageTemplateCarouselColumn::new("col2");
    col2.append_action(LineActionUri::new("col2_url", test_url));

    template_confirm.append_column(col1);
    template_confirm.append_column(col2);
    let mut template = LineMessageTemplate::new("alt", template_confirm);
    builder.append(template);

    // テンプレート_画像
    let mut builder = LineMessagesBuilder::new();
    let mut template_confirm = LineMessageTemplateCarouselImage::new(vec![]);

    let mut col1 = LineMessageTemplateCarouselImageColumn::new(
        test_image,
        LineActionUri::new("col1_url", test_url),
    );
    let mut col2 = LineMessageTemplateCarouselImageColumn::new(
        test_image,
        LineActionUri::new("col2_url", test_url),
    );

    template_confirm.append_column(col1);
    template_confirm.append_column(col2);
    let mut template = LineMessageTemplate::new("alt", template_confirm);
    builder.append(template);
    // Flex
    let mut builder = LineMessagesBuilder::new();
    let mut template_confirm = LineMessageFlex::new(
        "aa",
        json!( {
          "type": "bubble",
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "hello"
              },
              {
                "type": "text",
                "text": "world"
              }
            ]
          }
        }),
    );
    builder.append(template_confirm);

    // let mut builder = LineMessagesBuilder::new();
    // builder.append(uri);
    //
    // // builder.append(location);
    //
    //
    println!(
        "{}",
        json!(builder.to_multi_cast_request(vec![user_id.to_string()])).to_string()
    );
    // // let a = client.message_push(&builder.to_push_request(user_id)).await;
    let a = client
        .message_send_push(&builder.to_push_request(user_id))
        .await;
    println!("{:?}", a);
}

#[tokio::test]
async fn test_配信メッセージ集計() {
    println!("配信正常ケーステスト");
    let client = get_client();
    let res = client.message_get_quota().await;
    println!("{:?}", res);
    let res = client.message_get_quota_consumption().await;
    println!("{:?}", res);
    let res = client
        .message_get_number_of_reply_messages("20231201")
        .await;
    println!("{:?}", res);
    let res = client.message_get_number_of_push_messages("20231201").await;
    println!("{:?}", res);
    let res = client
        .message_get_number_of_multicast_messages("20231201")
        .await;
    println!("{:?}", res);
    let res = client
        .message_get_number_of_broadcast_messages("20231201")
        .await;
    println!("{:?}", res);

    let msg = LineMessageText::new("テスト".to_string());
    let mut builder = LineMessagesBuilder::new();
    builder.append(msg);
    let res = client.message_validate_reply(&builder.messages).await;
    println!("{:?}", res);
    let res = client.message_aggregation_info().await;
    println!("{:?}", res);
    let res = client.message_aggregation_list().await;
    println!("{:?}", res);
}
