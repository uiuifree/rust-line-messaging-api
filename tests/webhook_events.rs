use std::collections::BTreeMap;

use line_bot_messaging_api::webhook::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

const USER_ID: &str = "U4af4980629";

/// Decodes `value`, asserts that it serializes back to the same JSON and returns it.
fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> T {
    let decoded: T = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(&decoded).unwrap(), value);
    decoded
}

/// Round-trips a known event and checks that its common properties are exposed.
fn known_event(value: Value) -> Event {
    let event: Event = roundtrip(value.clone());
    let common = event.common().expect("known event");
    assert_eq!(common.webhook_event_id, value["webhookEventId"]);
    event
}

/// A webhook event of type `kind` from a user source with the given extra properties.
fn event_json(kind: &str, extra: Value) -> Value {
    let mut event = json!({
        "type": kind,
        "mode": "active",
        "timestamp": 1462629479859_i64,
        "source": {"type": "user", "userId": USER_ID},
        "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
        "deliveryContext": {"isRedelivery": false},
    });
    event
        .as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    event
}

fn common() -> EventCommon {
    EventCommon {
        source: Some(Source::from(UserSource {
            user_id: Some(USER_ID.into()),
        })),
        timestamp: 1462629479859,
        mode: EventMode::Active,
        webhook_event_id: "01FZ74A0TDDPYRVKNK77XKC3ZR".into(),
        delivery_context: DeliveryContext {
            is_redelivery: false,
        },
    }
}

fn line_provider() -> ContentProvider {
    ContentProvider {
        r#type: ContentProviderType::Line,
        original_content_url: None,
        preview_image_url: None,
    }
}

fn message_event(message: Value) -> MessageContent {
    let event = known_event(event_json(
        "message",
        json!({"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA", "message": message}),
    ));
    assert_eq!(event.reply_token(), Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA"));
    let message: MessageContent = serde_json::from_value(message).unwrap();
    assert_eq!(
        event,
        Event::from(MessageEvent {
            common: common(),
            reply_token: Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA".into()),
            message: message.clone(),
        })
    );
    message
}

#[test]
fn callback_request_keeps_unknown_events() {
    let request: CallbackRequest = roundtrip(json!({
        "destination": "Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "events": [
            event_json("unfollow", json!({})),
            {"type": "somethingNew", "foo": 1},
        ],
    }));
    assert_eq!(
        request.events,
        vec![
            Event::from(UnfollowEvent { common: common() }),
            Event::Unknown(json!({"type": "somethingNew", "foo": 1})),
        ]
    );
}

#[test]
fn events_without_reply_token_still_parse() {
    // The spec marks replyToken as required for these events; a payload without it
    // must not make the whole callback request fail.
    let kinds = [
        ("follow", json!({"follow": {"isUnblocked": false}})),
        ("join", json!({})),
        (
            "memberJoined",
            json!({"joined": {"members": [{"type": "user", "userId": USER_ID}]}}),
        ),
        (
            "videoPlayComplete",
            json!({"videoPlayComplete": {"trackingId": "track-1"}}),
        ),
        (
            "beacon",
            json!({"beacon": {"hwid": "d41d8cd98f", "type": "enter"}}),
        ),
        (
            "membership",
            json!({"membership": {"type": "joined", "membershipId": 3189}}),
        ),
    ];
    for (kind, extra) in kinds {
        let mut value = event_json(kind, extra);
        value["mode"] = json!("standby");
        let request: CallbackRequest = roundtrip(json!({
            "destination": "Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "events": [value, event_json("unfollow", json!({}))],
        }));
        assert_eq!(request.events.len(), 2, "{kind}");
        assert!(
            request.events[0].common().is_some(),
            "{kind} decoded as a known event"
        );
        assert_eq!(request.events[0].reply_token(), None, "{kind}");
    }
}

#[test]
fn unknown_event_has_no_common_properties() {
    let event: Event = roundtrip(json!({"type": "somethingNew", "replyToken": "x"}));
    assert_eq!(event.common(), None);
    assert_eq!(event.source(), None);
    assert_eq!(event.reply_token(), None);
}

#[test]
fn event_without_type_is_an_error() {
    let err = serde_json::from_value::<Event>(json!({"timestamp": 1})).unwrap_err();
    assert!(err.to_string().contains("missing field `type`"), "{err}");
}

#[test]
fn malformed_known_event_is_an_error() {
    let err = serde_json::from_value::<Event>(event_json("message", json!({}))).unwrap_err();
    assert!(err.to_string().contains("message"), "{err}");
    let err = serde_json::from_value::<CallbackRequest>(json!({
        "destination": "U",
        "events": [event_json("follow", json!({"replyToken": "r"}))],
    }))
    .unwrap_err();
    assert!(err.to_string().contains("follow"), "{err}");
}

#[test]
fn malformed_known_source_and_message_are_errors() {
    assert!(serde_json::from_value::<Source>(json!({"type": "group"})).is_err());
    assert!(serde_json::from_value::<MessageContent>(json!({"type": "text", "id": "1"})).is_err());
    assert!(serde_json::from_value::<Mentionee>(json!({"type": "user"})).is_err());
    assert!(serde_json::from_value::<MembershipContent>(json!({"type": "joined"})).is_err());
    assert!(serde_json::from_value::<ModuleContent>(json!({"type": "attached"})).is_err());
}

#[test]
fn event_without_source() {
    let mut value = event_json("botSuspended", json!({}));
    value.as_object_mut().unwrap().remove("source");
    let event = known_event(value);
    assert_eq!(event.source(), None);
    assert_eq!(event.common().map(|c| c.timestamp), Some(1462629479859));
}

#[test]
fn text_message() {
    let message = message_event(json!({
        "type": "text",
        "id": "468789577898262530",
        "quoteToken": "q3Plxr4AgKd",
        "text": "@All @example Good Morning!! (love)",
        "emojis": [{"index": 29, "length": 6, "productId": "5ac1bfd5040ab15980c9b435", "emojiId": "001"}],
        "mention": {"mentionees": [
            {"index": 0, "length": 4, "type": "all"},
            {"index": 5, "length": 8, "userId": "U49585cd0d5", "type": "user", "isSelf": false},
        ]},
        "quotedMessageId": "468789532432007169",
        "markAsReadToken": "30yhdy232",
    }));
    assert_eq!(message.id(), Some("468789577898262530"));
    assert_eq!(
        message,
        MessageContent::from(TextMessageContent {
            id: "468789577898262530".into(),
            text: "@All @example Good Morning!! (love)".into(),
            emojis: Some(vec![Emoji {
                index: 29,
                length: 6,
                product_id: "5ac1bfd5040ab15980c9b435".into(),
                emoji_id: "001".into(),
            }]),
            mention: Some(Mention {
                mentionees: vec![
                    Mentionee::from(AllMentionee {
                        index: 0,
                        length: 4
                    }),
                    Mentionee::from(UserMentionee {
                        index: 5,
                        length: 8,
                        user_id: Some("U49585cd0d5".into()),
                        is_self: Some(false),
                    }),
                ],
            }),
            quote_token: "q3Plxr4AgKd".into(),
            quoted_message_id: Some("468789532432007169".into()),
            mark_as_read_token: Some("30yhdy232".into()),
        })
    );
}

#[test]
fn unknown_mentionee_roundtrips() {
    let mentionee: Mentionee = roundtrip(json!({"type": "channel", "index": 0, "length": 3}));
    assert!(matches!(mentionee, Mentionee::Unknown(_)));
}

#[test]
fn image_message() {
    let message = message_event(json!({
        "type": "image",
        "id": "354718705033693859",
        "quoteToken": "q3Plxr4AgKd",
        "contentProvider": {"type": "line"},
        "imageSet": {"id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855", "index": 1, "total": 2},
        "markAsReadToken": "30yhdy232",
    }));
    assert_eq!(message.id(), Some("354718705033693859"));
    assert_eq!(
        message,
        MessageContent::from(ImageMessageContent {
            id: "354718705033693859".into(),
            content_provider: line_provider(),
            image_set: Some(ImageSet {
                id: "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855".into(),
                index: Some(1),
                total: Some(2),
            }),
            quote_token: "q3Plxr4AgKd".into(),
            mark_as_read_token: Some("30yhdy232".into()),
        })
    );
}

#[test]
fn video_message() {
    let message = message_event(json!({
        "type": "video",
        "id": "325708",
        "quoteToken": "q3Plxr4AgKd",
        "duration": 60000,
        "contentProvider": {
            "type": "external",
            "originalContentUrl": "https://example.com/original.mp4",
            "previewImageUrl": "https://example.com/preview.jpg",
        },
    }));
    assert_eq!(message.id(), Some("325708"));
    assert_eq!(
        message,
        MessageContent::from(VideoMessageContent {
            id: "325708".into(),
            duration: Some(60000),
            content_provider: ContentProvider {
                r#type: ContentProviderType::External,
                original_content_url: Some("https://example.com/original.mp4".into()),
                preview_image_url: Some("https://example.com/preview.jpg".into()),
            },
            quote_token: "q3Plxr4AgKd".into(),
            mark_as_read_token: None,
        })
    );
}

#[test]
fn audio_message() {
    let message = message_event(json!({
        "type": "audio",
        "id": "325708",
        "duration": 60000,
        "contentProvider": {"type": "line"},
    }));
    assert_eq!(message.id(), Some("325708"));
    assert_eq!(
        message,
        MessageContent::from(AudioMessageContent {
            id: "325708".into(),
            content_provider: line_provider(),
            duration: Some(60000),
            mark_as_read_token: None,
        })
    );
}

#[test]
fn file_message() {
    let message = message_event(json!({
        "type": "file",
        "id": "325708",
        "fileName": "file.txt",
        "fileSize": 2138,
    }));
    assert_eq!(message.id(), Some("325708"));
    assert_eq!(
        message,
        MessageContent::from(FileMessageContent {
            id: "325708".into(),
            file_name: "file.txt".into(),
            file_size: 2138,
            mark_as_read_token: None,
        })
    );
}

#[test]
fn location_message() {
    let message = message_event(json!({
        "type": "location",
        "id": "325708",
        "title": "my location",
        "address": "1-3 Kioicho, Chiyoda-ku, Tokyo, 102-8282, Japan",
        "latitude": 35.67966,
        "longitude": 139.73669,
    }));
    assert_eq!(message.id(), Some("325708"));
    assert_eq!(
        message,
        MessageContent::from(LocationMessageContent {
            id: "325708".into(),
            title: Some("my location".into()),
            address: Some("1-3 Kioicho, Chiyoda-ku, Tokyo, 102-8282, Japan".into()),
            latitude: 35.67966,
            longitude: 139.73669,
            mark_as_read_token: None,
        })
    );
}

#[test]
fn sticker_message() {
    let message = message_event(json!({
        "type": "sticker",
        "id": "1501597916",
        "quoteToken": "q3Plxr4AgKd",
        "stickerId": "52002738",
        "packageId": "11537",
        "stickerResourceType": "ANIMATION",
        "keywords": ["cony", "sally", "Staring", "hi"],
        "text": "Let's hang out this weekend!",
        "quotedMessageId": "468789532432007169",
    }));
    assert_eq!(message.id(), Some("1501597916"));
    assert_eq!(
        message,
        MessageContent::from(StickerMessageContent {
            id: "1501597916".into(),
            package_id: "11537".into(),
            sticker_id: "52002738".into(),
            sticker_resource_type: StickerResourceType::Animation,
            keywords: Some(vec![
                "cony".into(),
                "sally".into(),
                "Staring".into(),
                "hi".into()
            ]),
            text: Some("Let's hang out this weekend!".into()),
            quote_token: "q3Plxr4AgKd".into(),
            quoted_message_id: Some("468789532432007169".into()),
            mark_as_read_token: None,
        })
    );
}

#[test]
fn unknown_message_roundtrips() {
    let message = message_event(json!({"type": "poll", "id": "1", "question": "?"}));
    assert_eq!(message.id(), None);
    assert!(matches!(message, MessageContent::Unknown(_)));
}

#[test]
fn string_enums_tolerate_unknown_values() {
    let unknown = json!("brandNewValue");
    assert_eq!(
        serde_json::from_value::<EventMode>(unknown.clone()).unwrap(),
        EventMode::Unknown
    );
    assert_eq!(
        serde_json::from_value::<ContentProviderType>(unknown.clone()).unwrap(),
        ContentProviderType::Unknown
    );
    assert_eq!(
        serde_json::from_value::<StickerResourceType>(unknown.clone()).unwrap(),
        StickerResourceType::Unknown
    );
    assert_eq!(
        serde_json::from_value::<BeaconEventType>(unknown.clone()).unwrap(),
        BeaconEventType::Unknown
    );
    assert_eq!(
        serde_json::from_value::<LinkResult>(unknown.clone()).unwrap(),
        LinkResult::Unknown
    );
    assert_eq!(
        serde_json::from_value::<DetachedReason>(unknown).unwrap(),
        DetachedReason::Unknown
    );
}

#[test]
fn string_enum_values() {
    let all = [
        StickerResourceType::Static,
        StickerResourceType::Animation,
        StickerResourceType::Sound,
        StickerResourceType::AnimationSound,
        StickerResourceType::Popup,
        StickerResourceType::PopupSound,
        StickerResourceType::Custom,
        StickerResourceType::Message,
        StickerResourceType::NameText,
        StickerResourceType::PerStickerText,
    ];
    assert_eq!(
        serde_json::to_value(all).unwrap(),
        json!([
            "STATIC",
            "ANIMATION",
            "SOUND",
            "ANIMATION_SOUND",
            "POPUP",
            "POPUP_SOUND",
            "CUSTOM",
            "MESSAGE",
            "NAME_TEXT",
            "PER_STICKER_TEXT"
        ])
    );
    assert_eq!(
        serde_json::to_value([EventMode::Active, EventMode::Standby]).unwrap(),
        json!(["active", "standby"])
    );
    assert_eq!(
        serde_json::to_value([
            BeaconEventType::Enter,
            BeaconEventType::Banner,
            BeaconEventType::Stay
        ])
        .unwrap(),
        json!(["enter", "banner", "stay"])
    );
    assert_eq!(
        serde_json::to_value([LinkResult::Ok, LinkResult::Failed]).unwrap(),
        json!(["ok", "failed"])
    );
}

#[test]
fn sources() {
    let user: Source = roundtrip(json!({"type": "user", "userId": USER_ID}));
    assert_eq!(user.user_id(), Some(USER_ID));

    let group: Source =
        roundtrip(json!({"type": "group", "groupId": "Ca56f9475", "userId": USER_ID}));
    assert_eq!(group.user_id(), Some(USER_ID));
    assert_eq!(
        group,
        Source::from(GroupSource {
            group_id: "Ca56f9475".into(),
            user_id: Some(USER_ID.into()),
        })
    );

    let room: Source = roundtrip(json!({"type": "room", "roomId": "Ra8dbf4673c"}));
    assert_eq!(room.user_id(), None);
    assert_eq!(
        room,
        Source::from(RoomSource {
            room_id: "Ra8dbf4673c".into(),
            user_id: None,
        })
    );

    let unknown: Source = roundtrip(json!({"type": "square", "squareId": "S1"}));
    assert_eq!(unknown.user_id(), None);
    assert!(matches!(unknown, Source::Unknown(_)));

    let anonymous: Source = roundtrip(json!({"type": "user"}));
    assert_eq!(anonymous, Source::from(UserSource::default()));
}

#[test]
fn unsend_event() {
    let event = known_event(event_json(
        "unsend",
        json!({"unsend": {"messageId": "325708"}}),
    ));
    assert_eq!(event.reply_token(), None);
    assert_eq!(
        event,
        Event::from(UnsendEvent {
            common: common(),
            unsend: UnsendDetail {
                message_id: "325708".into()
            },
        })
    );
}

#[test]
fn follow_event() {
    let event = known_event(event_json(
        "follow",
        json!({"replyToken": "85cbe770fa8b4f45bbe077b1d4be4a36", "follow": {"isUnblocked": false}}),
    ));
    assert_eq!(
        event.reply_token(),
        Some("85cbe770fa8b4f45bbe077b1d4be4a36")
    );
    assert_eq!(event.source().and_then(Source::user_id), Some(USER_ID));
    assert_eq!(
        event,
        Event::from(FollowEvent {
            common: common(),
            reply_token: Some("85cbe770fa8b4f45bbe077b1d4be4a36".into()),
            follow: FollowDetail {
                is_unblocked: false
            },
        })
    );
}

#[test]
fn unfollow_event() {
    let event = known_event(event_json("unfollow", json!({})));
    assert_eq!(event.reply_token(), None);
    assert_eq!(event, Event::from(UnfollowEvent { common: common() }));
}

#[test]
fn join_event() {
    let event = known_event(event_json(
        "join",
        json!({"replyToken": "0f3779fba3b349968c5d07db31eab56f"}),
    ));
    assert_eq!(
        event.reply_token(),
        Some("0f3779fba3b349968c5d07db31eab56f")
    );
    assert_eq!(
        event,
        Event::from(JoinEvent {
            common: common(),
            reply_token: Some("0f3779fba3b349968c5d07db31eab56f".into()),
        })
    );
}

#[test]
fn leave_event() {
    let event = known_event(event_json("leave", json!({})));
    assert_eq!(event, Event::from(LeaveEvent { common: common() }));
}

#[test]
fn member_joined_event() {
    let event = known_event(event_json(
        "memberJoined",
        json!({
            "replyToken": "0f3779fba3b349968c5d07db31eabf65",
            "joined": {"members": [{"type": "user", "userId": "U4af4980629"}, {"type": "user", "userId": "U91eeaf62d9"}]},
        }),
    ));
    assert_eq!(
        event.reply_token(),
        Some("0f3779fba3b349968c5d07db31eabf65")
    );
    assert_eq!(
        event,
        Event::from(MemberJoinedEvent {
            common: common(),
            reply_token: Some("0f3779fba3b349968c5d07db31eabf65".into()),
            joined: JoinedMembers {
                members: vec![
                    UserSource {
                        user_id: Some("U4af4980629".into())
                    },
                    UserSource {
                        user_id: Some("U91eeaf62d9".into())
                    },
                ],
            },
        })
    );
}

#[test]
fn member_left_event() {
    let value = event_json(
        "memberLeft",
        json!({"left": {"members": [{"type": "user", "userId": "U4af4980629"}]}}),
    );
    let event = known_event(value);
    assert_eq!(
        event,
        Event::from(MemberLeftEvent {
            common: common(),
            left: LeftMembers {
                members: vec![UserSource {
                    user_id: Some("U4af4980629".into())
                }],
            },
        })
    );
}

#[test]
fn postback_event() {
    let event = known_event(event_json(
        "postback",
        json!({
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "postback": {
                "data": "action=buy&itemid=111",
                "params": {"newRichMenuAliasId": "richmenu-alias-b", "status": "SUCCESS"},
            },
        }),
    ));
    assert_eq!(
        event.reply_token(),
        Some("b60d432864f44d079f6d8efe86cf404b")
    );
    assert_eq!(
        event,
        Event::from(PostbackEvent {
            common: common(),
            reply_token: Some("b60d432864f44d079f6d8efe86cf404b".into()),
            postback: PostbackContent {
                data: "action=buy&itemid=111".into(),
                params: Some(BTreeMap::from([
                    ("newRichMenuAliasId".into(), "richmenu-alias-b".into()),
                    ("status".into(), "SUCCESS".into()),
                ])),
            },
        })
    );
}

#[test]
fn video_play_complete_event() {
    let event = known_event(event_json(
        "videoPlayComplete",
        json!({"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA", "videoPlayComplete": {"trackingId": "track-id"}}),
    ));
    assert_eq!(event.reply_token(), Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA"));
    assert_eq!(
        event,
        Event::from(VideoPlayCompleteEvent {
            common: common(),
            reply_token: Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA".into()),
            video_play_complete: VideoPlayComplete {
                tracking_id: "track-id".into()
            },
        })
    );
}

#[test]
fn beacon_event() {
    let event = known_event(event_json(
        "beacon",
        json!({"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA", "beacon": {"hwid": "d41d8cd98f", "type": "enter", "dm": "1234"}}),
    ));
    assert_eq!(event.reply_token(), Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA"));
    assert_eq!(
        event,
        Event::from(BeaconEvent {
            common: common(),
            reply_token: Some("nHuyWiB7yP5Zw52FIkcQobQuGDXCTA".into()),
            beacon: BeaconContent {
                hwid: "d41d8cd98f".into(),
                r#type: BeaconEventType::Enter,
                dm: Some("1234".into()),
            },
        })
    );
}

#[test]
fn account_link_event() {
    let event = known_event(event_json(
        "accountLink",
        json!({"replyToken": "b60d432864f44d079f6d8efe86cf404b", "link": {"result": "ok", "nonce": "xxxxxxxxxxxxxxx"}}),
    ));
    assert_eq!(
        event.reply_token(),
        Some("b60d432864f44d079f6d8efe86cf404b")
    );
    assert_eq!(
        event,
        Event::from(AccountLinkEvent {
            common: common(),
            reply_token: Some("b60d432864f44d079f6d8efe86cf404b".into()),
            link: LinkContent {
                result: LinkResult::Ok,
                nonce: "xxxxxxxxxxxxxxx".into(),
            },
        })
    );

    let failed = known_event(event_json(
        "accountLink",
        json!({"link": {"result": "failed", "nonce": "n"}}),
    ));
    assert_eq!(failed.reply_token(), None);
}

#[test]
fn membership_events() {
    let cases = [
        (
            "joined",
            MembershipContent::from(JoinedMembershipContent {
                membership_id: 3189,
            }),
        ),
        (
            "left",
            MembershipContent::from(LeftMembershipContent {
                membership_id: 3189,
            }),
        ),
        (
            "renewed",
            MembershipContent::from(RenewedMembershipContent {
                membership_id: 3189,
            }),
        ),
    ];
    for (kind, membership) in cases {
        let event = known_event(event_json(
            "membership",
            json!({"replyToken": "r", "membership": {"type": kind, "membershipId": 3189}}),
        ));
        assert_eq!(event.reply_token(), Some("r"));
        assert_eq!(
            event,
            Event::from(MembershipEvent {
                common: common(),
                reply_token: Some("r".into()),
                membership,
            })
        );
    }
    let unknown: MembershipContent = roundtrip(json!({"type": "upgraded", "membershipId": 1}));
    assert!(matches!(unknown, MembershipContent::Unknown(_)));
}

#[test]
fn module_events() {
    let attached = known_event(event_json(
        "module",
        json!({"module": {"type": "attached", "botId": "U111", "scopes": ["message:send", "message:receive"]}}),
    ));
    assert_eq!(attached.reply_token(), None);
    assert_eq!(
        attached,
        Event::from(ModuleEvent {
            common: common(),
            module: ModuleContent::from(AttachedModuleContent {
                bot_id: "U111".into(),
                scopes: vec!["message:send".into(), "message:receive".into()],
            }),
        })
    );

    let detached = known_event(event_json(
        "module",
        json!({"module": {"type": "detached", "botId": "U111", "reason": "bot_deleted"}}),
    ));
    assert_eq!(
        detached,
        Event::from(ModuleEvent {
            common: common(),
            module: ModuleContent::from(DetachedModuleContent {
                bot_id: "U111".into(),
                reason: DetachedReason::BotDeleted,
            }),
        })
    );

    let unknown: ModuleContent = roundtrip(json!({"type": "paused", "botId": "U111"}));
    assert!(matches!(unknown, ModuleContent::Unknown(_)));
}

#[test]
fn activated_event() {
    let event = known_event(event_json(
        "activated",
        json!({"chatControl": {"expireAt": 1462629479860_i64}}),
    ));
    assert_eq!(
        event,
        Event::from(ActivatedEvent {
            common: common(),
            chat_control: ChatControl {
                expire_at: 1462629479860
            },
        })
    );
}

#[test]
fn module_channel_events_without_body() {
    let mut value = event_json("deactivated", json!({}));
    value["mode"] = json!("standby");
    let deactivated = known_event(value);
    let standby = EventCommon {
        mode: EventMode::Standby,
        ..common()
    };
    assert_eq!(
        deactivated,
        Event::from(DeactivatedEvent { common: standby })
    );

    let suspended = known_event(event_json("botSuspended", json!({})));
    assert_eq!(
        suspended,
        Event::from(BotSuspendedEvent { common: common() })
    );

    let resumed = known_event(event_json("botResumed", json!({})));
    assert_eq!(resumed, Event::from(BotResumedEvent { common: common() }));
}

#[test]
fn delivery_event() {
    let event = known_event(event_json(
        "delivery",
        json!({"delivery": {"data": "8835a5f22e5c2d8bcd7f4a9a5cd08a32"}}),
    ));
    assert_eq!(
        event,
        Event::from(PnpDeliveryCompletionEvent {
            common: common(),
            delivery: PnpDelivery {
                data: "8835a5f22e5c2d8bcd7f4a9a5cd08a32".into()
            },
        })
    );
}

#[test]
fn message_edited_event() {
    let mut value = event_json(
        "messageEdited",
        json!({
            "replyToken": "r",
            "message": {"type": "text", "id": "468789577898262530", "quoteToken": "q", "text": "edited"},
        }),
    );
    value["source"] = json!({"type": "group", "groupId": "Ca56f9475", "userId": USER_ID});
    value["deliveryContext"] = json!({"isRedelivery": true});
    let event = known_event(value);
    assert_eq!(event.reply_token(), Some("r"));
    assert_eq!(event.source().and_then(Source::user_id), Some(USER_ID));
    assert_eq!(
        event,
        Event::from(MessageEditedEvent {
            common: EventCommon {
                source: Some(Source::from(GroupSource {
                    group_id: "Ca56f9475".into(),
                    user_id: Some(USER_ID.into()),
                })),
                delivery_context: DeliveryContext {
                    is_redelivery: true
                },
                ..common()
            },
            reply_token: Some("r".into()),
            message: MessageContent::from(TextMessageContent {
                id: "468789577898262530".into(),
                text: "edited".into(),
                emojis: None,
                mention: None,
                quote_token: "q".into(),
                quoted_message_id: None,
                mark_as_read_token: None,
            }),
        })
    );
}
