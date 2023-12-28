use line_bot_messaging_api::webhook::{LineWebhook, LineWebhookEventObject};

#[tokio::test]
async fn test() {
    let test_data_list = vec![
        r#"
       {
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "type": "unsend",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "group",
        "groupId": "Ca56f94637c...",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "unsend": {
        "messageId": "325708"
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "follow",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "type": "unfollow",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "join",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "group",
        "groupId": "C4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "type": "leave",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "group",
        "groupId": "C4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "0f3779fba3b349968c5d07db31eabf65",
      "type": "memberJoined",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "group",
        "groupId": "C4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "joined": {
        "members": [
          {
            "type": "user",
            "userId": "U4af4980629..."
          },
          {
            "type": "user",
            "userId": "U91eeaf62d9..."
          }
        ]
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "type": "memberLeft",
      "mode": "active",
      "timestamp": 1462629479960,
      "source": {
        "type": "group",
        "groupId": "C4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "left": {
        "members": [
          {
            "type": "user",
            "userId": "U4af4980629..."
          },
          {
            "type": "user",
            "userId": "U91eeaf62d9..."
          }
        ]
      }
    }
  ]
}"#,
        r#"{
    "destination": "xxxxxxxxxx",
    "events": [
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1513669370317,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "storeId=12345",
                "params": {
                    "datetime": "2017-12-25T01:00"
                }
            }
        }
    ]
}
"#,
        r#"{
    "destination": "xxxxxxxxxx",
    "events": [
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1619754620404,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "richmenu-changed-to-b",
                "params": {
                    "newRichMenuAliasId": "richmenu-alias-b",
                    "status": "SUCCESS"
                }
            }
        }
    ]
}
"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "videoPlayComplete",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "videoPlayComplete": {
        "trackingId": "track-id"
      }
    }
  ]
}
"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "beacon",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "beacon": {
        "hwid": "d41d8cd98f",
        "type": "enter"
      }
    }
  ]
}
"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "b60d432864f44d079f6d8efe86cf404b",
      "type": "accountLink",
      "mode": "active",
      "source": {
        "userId": "U91eeaf62d...",
        "type": "user"
      },
      "timestamp": 1513669370317,
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "link": {
        "result": "ok",
        "nonce": "xxxxxxxxxxxxxxx"
      }
    }
  ]
}
"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "things",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U91eeaf62d..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "things": {
        "deviceId": "t2c449c9d1...",
        "type": "link"
      }
    }
  ]
}
"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "things",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U91eeaf62d..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "things": {
        "deviceId": "t2c449c9d1...",
        "type": "unlink"
      }
    }
  ]
}
"#,
        r#"{
  "events": [
    {
      "replyToken": "0f3779fba3b349968c5d07db31eab56f",
      "type": "things",
      "mode": "active",
      "source": {
        "userId": "uXXX",
        "type": "user"
      },
      "timestamp": 1547817848122,
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "things": {
        "type": "scenarioResult",
        "deviceId": "tXXX",
        "result": {
          "scenarioId": "XXX",
          "revision": 2,
          "startTime": 1547817845950,
          "endTime": 1547817845952,
          "resultCode": "success",
          "bleNotificationPayload": "AQ==",
          "actionResults": [
            {
              "type": "binary",
              "data": "/w=="
            }
          ]
        }
      }
    }
  ],
  "destination": "xxxxxxxxxx"
}
"#,
    ];

    for test in test_data_list {
        println!("----------------");
        for event in LineWebhook::parse_str(test).events() {
            println!("event {:?}", event);
            println!("reply_token {:?}", event.reply_token());
            println!("data {:?}", event.data());
            println!("source {:?}", event.source());
            println!("json {:?}", event.json());

            match event.data() {
                LineWebhookEventObject::Message(_) => {}
                LineWebhookEventObject::UnSend(_) => {}
                LineWebhookEventObject::Follow => {}
                LineWebhookEventObject::UnFollow => {}
                LineWebhookEventObject::Join => {}
                LineWebhookEventObject::Leave => {}
                LineWebhookEventObject::MemberJoined(_) => {}
                LineWebhookEventObject::MemberLeft(_) => {}
                LineWebhookEventObject::Postback(_) => {}
                LineWebhookEventObject::VideoPlayComplete(_) => {}
                LineWebhookEventObject::Beacon(_) => {}
                LineWebhookEventObject::AccountLink(_) => {}
                LineWebhookEventObject::Things(_) => {}
                LineWebhookEventObject::None => {}
            }
        }
    }
}

#[tokio::test]
async fn test2() {
    let test_data_list = vec![
        r#"{"destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "group",
        "groupId": "Ca56f94637c...",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "444573844083572737",
        "type": "text",
        "quoteToken": "q3Plxr4AgKd...",
        "text": "@All @example Good Morning!! (love)",
        "emojis": [
          {
            "index": 29,
            "length": 6,
            "productId": "5ac1bfd5040ab15980c9b435",
            "emojiId": "001"
          }
        ],
        "mention": {
          "mentionees": [
            {
              "index": 0,
              "length": 4,
              "type": "all"
            },
            {
              "index": 5,
              "length": 8,
              "userId": "U49585cd0d5...",
              "type": "user"
            }
          ]
        }
      }
    }
  ]
}"#,
        r#"{
    "destination": "xxxxxxxxxx",
    "events": [
        {
            "type": "message",
            "message": {
                "type": "image",
                "id": "354718705033693859",
                "quoteToken": "q3Plxr4AgKd...",
                "contentProvider": {
                    "type": "line"
                },
                "imageSet": {
                    "id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855",
                    "index": 1,
                    "total": 2
                }
            },
            "timestamp": 1627356924513,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "replyToken": "7840b71058e24a5d91f9b5726c7512c9",
            "mode": "active"
        }
    ]
}"#,
        r#" { "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "325708",
        "type": "video",
        "quoteToken": "q3Plxr4AgKd...",
        "duration": 60000,
        "contentProvider": {
          "type": "external",
          "originalContentUrl": "https://example.com/original.mp4",
          "previewImageUrl": "https://example.com/preview.jpg"
        }
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "325708",
        "type": "video",
        "quoteToken": "q3Plxr4AgKd...",
        "duration": 60000,
        "contentProvider": {
          "type": "external",
          "originalContentUrl": "https://example.com/original.mp4",
          "previewImageUrl": "https://example.com/preview.jpg"
        }
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "325708",
        "type": "audio",
        "duration": 60000,
        "contentProvider": {
          "type": "line"
        }
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "325708",
        "type": "file",
        "fileName": "file.txt",
        "fileSize": 2138
      }
    }
  ]
}"#,
        r#"{
  "destination": "xxxxxxxxxx",
  "events": [
    {
      "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
      "type": "message",
      "mode": "active",
      "timestamp": 1462629479859,
      "source": {
        "type": "user",
        "userId": "U4af4980629..."
      },
      "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
      "deliveryContext": {
        "isRedelivery": false
      },
      "message": {
        "id": "325708",
        "type": "location",
        "title": "my location",
        "address": "日本、〒102-8282 東京都千代田区紀尾井町1番3号",
        "latitude": 35.67966,
        "longitude": 139.73669
      }
    }
  ]
}"#,
        r#"{
    "destination": "xxxxxxxxxx",
    "events": [
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "type": "sticker",
                "id": "1501597916",
                "quoteToken": "q3Plxr4AgKd...",
                "stickerId": "52002738",
                "packageId": "11537",
                "stickerResourceType": "ANIMATION",
                "keywords": [
                    "cony",
                    "sally",
                    "Staring",
                    "hi",
                    "whatsup",
                    "line",
                    "howdy",
                    "HEY",
                    "Peeking",
                    "wave",
                    "peek",
                    "Hello",
                    "yo",
                    "greetings"
                ]
            }
        }
    ]
}"#,
        // r#""#,
    ];

    for test in test_data_list {
        println!("----------------");
        for event in LineWebhook::parse_str(test).events() {
            println!("event {:?}", event);
            println!("reply_token {:?}", event.reply_token());
            println!("data {:?}", event.data());
            println!("source {:?}", event.source());
            println!("json {:?}", event.json());

            match event.data() {
                LineWebhookEventObject::Message(v) => {
                    println!("message {:?}", v);
                }
                _ => {
                    assert!(false)
                }
            }
        }
    }
}
