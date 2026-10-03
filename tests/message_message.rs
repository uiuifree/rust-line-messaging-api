use std::fmt::Debug;

use line_bot_messaging_api::message::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// Asserts that `value` serializes to `expected` and that `expected` decodes back to `value`.
fn assert_json<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T, expected: Value) {
    assert_eq!(serde_json::to_value(value).unwrap(), expected);
    assert_eq!(&serde_json::from_value::<T>(expected).unwrap(), value);
}

fn quick_reply() -> QuickReply {
    QuickReply::new([QuickReplyItem::new(MessageAction::new("Hi").label("Hi"))])
}

fn quick_reply_json() -> Value {
    json!({"items": [{"type": "action", "action": {"type": "message", "label": "Hi", "text": "Hi"}}]})
}

fn sender() -> Sender {
    Sender::default().name("Bot")
}

#[test]
fn text_message_minimal_is_exact() {
    let message = TextMessage::new("x");
    assert_eq!(
        serde_json::to_string(&message).unwrap(),
        r#"{"type":"text","text":"x"}"#
    );
    assert_eq!(
        serde_json::to_string(&Message::from(message)).unwrap(),
        r#"{"type":"text","text":"x"}"#
    );
}

#[test]
fn text_message_all_fields() {
    let message: Message = TextMessage::new("$ LINE emoji $")
        .emojis([
            Emoji::new(0, "5ac1bfd5040ab15980c9b435", "001"),
            Emoji::default()
                .index(13)
                .product_id("5ac1bfd5040ab15980c9b435")
                .emoji_id("002"),
        ])
        .quote_token("q3Plxr4AgKd...")
        .quick_reply(quick_reply())
        .sender(sender())
        .into();
    assert_json(
        &message,
        json!({
            "type": "text",
            "text": "$ LINE emoji $",
            "emojis": [
                {"index": 0, "productId": "5ac1bfd5040ab15980c9b435", "emojiId": "001"},
                {"index": 13, "productId": "5ac1bfd5040ab15980c9b435", "emojiId": "002"}
            ],
            "quoteToken": "q3Plxr4AgKd...",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn text_message_v2_with_substitution() {
    let message: Message =
        TextMessageV2::new("Welcome, {user1}! {laugh}\n{everyone} There is a newcomer!")
            .substitution(
                "user1",
                MentionSubstitutionObject::new(UserMentionTarget::new("U49585cd0d5...")),
            )
            .substitution(
                "laugh",
                EmojiSubstitutionObject::new("5a8555cfe6256cc92ea23c1a", "002"),
            )
            .substitution(
                "everyone",
                MentionSubstitutionObject::new(AllMentionTarget::default()),
            )
            .quote_token("token")
            .quick_reply(quick_reply())
            .sender(sender())
            .into();
    assert_json(
        &message,
        json!({
            "type": "textV2",
            "text": "Welcome, {user1}! {laugh}\n{everyone} There is a newcomer!",
            "substitution": {
                "user1": {"type": "mention", "mentionee": {"type": "user", "userId": "U49585cd0d5..."}},
                "laugh": {"type": "emoji", "productId": "5a8555cfe6256cc92ea23c1a", "emojiId": "002"},
                "everyone": {"type": "mention", "mentionee": {"type": "all"}}
            },
            "quoteToken": "token",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
    assert_json(
        &TextMessageV2::new("hi"),
        json!({"type": "textV2", "text": "hi"}),
    );
}

#[test]
fn substitution_and_mention_target_tolerate_unknown_types() {
    let raw = json!({"type": "sticker", "id": "1"});
    let substitution: SubstitutionObject = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(substitution, SubstitutionObject::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&substitution).unwrap(), raw);

    let raw = json!({"type": "group"});
    let target: MentionTarget = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(target, MentionTarget::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&target).unwrap(), raw);
}

#[test]
fn sticker_message() {
    let message: Message = StickerMessage::new("446", "1988")
        .quote_token("token")
        .quick_reply(quick_reply())
        .sender(sender())
        .into();
    assert_json(
        &message,
        json!({
            "type": "sticker",
            "packageId": "446",
            "stickerId": "1988",
            "quoteToken": "token",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn image_message() {
    let message: Message = ImageMessage::new(
        "https://example.com/original.jpg",
        "https://example.com/preview.jpg",
    )
    .quick_reply(quick_reply())
    .sender(sender())
    .into();
    assert_json(
        &message,
        json!({
            "type": "image",
            "originalContentUrl": "https://example.com/original.jpg",
            "previewImageUrl": "https://example.com/preview.jpg",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn video_message() {
    let message: Message = VideoMessage::new(
        "https://example.com/original.mp4",
        "https://example.com/preview.jpg",
    )
    .tracking_id("track-id")
    .quick_reply(quick_reply())
    .sender(sender())
    .into();
    assert_json(
        &message,
        json!({
            "type": "video",
            "originalContentUrl": "https://example.com/original.mp4",
            "previewImageUrl": "https://example.com/preview.jpg",
            "trackingId": "track-id",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn audio_message() {
    let message: Message = AudioMessage::new("https://example.com/original.m4a", 60000)
        .quick_reply(quick_reply())
        .sender(sender())
        .into();
    assert_json(
        &message,
        json!({
            "type": "audio",
            "originalContentUrl": "https://example.com/original.m4a",
            "duration": 60000,
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn location_message() {
    let message: Message = LocationMessage::new(
        "my location",
        "1-3 Kioicho, Chiyoda-ku, Tokyo, 102-8282, Japan",
        35.67966,
        139.73669,
    )
    .quick_reply(quick_reply())
    .sender(sender())
    .into();
    assert_json(
        &message,
        json!({
            "type": "location",
            "title": "my location",
            "address": "1-3 Kioicho, Chiyoda-ku, Tokyo, 102-8282, Japan",
            "latitude": 35.67966,
            "longitude": 139.73669,
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn coupon_message() {
    let message: Message = CouponMessage::new("01JYNW8JMQVFBNSFQGWVKRDQ4Q")
        .delivery_tag("campaign_202507")
        .quick_reply(quick_reply())
        .sender(sender())
        .into();
    assert_json(
        &message,
        json!({
            "type": "coupon",
            "couponId": "01JYNW8JMQVFBNSFQGWVKRDQ4Q",
            "deliveryTag": "campaign_202507",
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn imagemap_message() {
    let area = ImagemapArea::new(0, 0, 520, 1040);
    let message: Message = ImagemapMessage::new(
        "https://example.com/bot/images/rm001",
        "This is an imagemap",
        ImagemapBaseSize::new(1040, 1040),
        [
            ImagemapAction::from(
                UriImagemapAction::new("https://example.com/", area).label("https://example.com/"),
            ),
            MessageImagemapAction::new("Hello", ImagemapArea::new(520, 0, 520, 1040))
                .label("hello")
                .into(),
            ClipboardImagemapAction::new("ABCD", area)
                .label("Copy")
                .into(),
        ],
    )
    .video(
        ImagemapVideo::new(
            "https://example.com/video.mp4",
            "https://example.com/video_preview.jpg",
            ImagemapArea::new(0, 0, 1040, 585),
        )
        .external_link(ImagemapExternalLink::new(
            "https://example.com/see_more.html",
            "See More",
        )),
    )
    .quick_reply(quick_reply())
    .sender(sender())
    .into();
    assert_json(
        &message,
        json!({
            "type": "imagemap",
            "baseUrl": "https://example.com/bot/images/rm001",
            "altText": "This is an imagemap",
            "baseSize": {"width": 1040, "height": 1040},
            "actions": [
                {
                    "type": "uri",
                    "area": {"x": 0, "y": 0, "width": 520, "height": 1040},
                    "linkUri": "https://example.com/",
                    "label": "https://example.com/"
                },
                {
                    "type": "message",
                    "area": {"x": 520, "y": 0, "width": 520, "height": 1040},
                    "text": "Hello",
                    "label": "hello"
                },
                {
                    "type": "clipboard",
                    "area": {"x": 0, "y": 0, "width": 520, "height": 1040},
                    "clipboardText": "ABCD",
                    "label": "Copy"
                }
            ],
            "video": {
                "originalContentUrl": "https://example.com/video.mp4",
                "previewImageUrl": "https://example.com/video_preview.jpg",
                "area": {"x": 0, "y": 0, "width": 1040, "height": 585},
                "externalLink": {"linkUri": "https://example.com/see_more.html", "label": "See More"}
            },
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn imagemap_optional_objects_default_to_empty() {
    assert_json(&ImagemapVideo::default(), json!({}));
    assert_json(&ImagemapExternalLink::default(), json!({}));
}

#[test]
fn imagemap_action_tolerates_unknown_type() {
    let raw = json!({"type": "postback", "area": {"x": 0, "y": 0, "width": 1, "height": 1}});
    let action: ImagemapAction = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(action, ImagemapAction::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&action).unwrap(), raw);
    assert!(
        serde_json::from_value::<ImagemapAction>(json!({"type": "uri", "linkUri": "x"})).is_err()
    );
}

#[test]
fn buttons_template_message() {
    let message: Message = TemplateMessage::new(
        "This is a buttons template",
        ButtonsTemplate::new(
            "Please select",
            [
                Action::from(PostbackAction::new("action=buy&itemid=123").label("Buy")),
                UriAction::new("http://example.com/page/123")
                    .label("View detail")
                    .into(),
            ],
        )
        .thumbnail_image_url("https://example.com/bot/images/image.jpg")
        .image_aspect_ratio("rectangle")
        .image_size("cover")
        .image_background_color("#FFFFFF")
        .title("Menu")
        .default_action(UriAction::new("http://example.com/page/123").label("View detail")),
    )
    .quick_reply(quick_reply())
    .sender(sender())
    .into();
    assert_json(
        &message,
        json!({
            "type": "template",
            "altText": "This is a buttons template",
            "template": {
                "type": "buttons",
                "thumbnailImageUrl": "https://example.com/bot/images/image.jpg",
                "imageAspectRatio": "rectangle",
                "imageSize": "cover",
                "imageBackgroundColor": "#FFFFFF",
                "title": "Menu",
                "text": "Please select",
                "defaultAction": {"type": "uri", "label": "View detail", "uri": "http://example.com/page/123"},
                "actions": [
                    {"type": "postback", "label": "Buy", "data": "action=buy&itemid=123"},
                    {"type": "uri", "label": "View detail", "uri": "http://example.com/page/123"}
                ]
            },
            "quickReply": quick_reply_json(),
            "sender": {"name": "Bot"}
        }),
    );
}

#[test]
fn confirm_template() {
    let template: Template = ConfirmTemplate::new(
        "Are you sure?",
        [
            MessageAction::new("yes").label("Yes"),
            MessageAction::new("no").label("No"),
        ],
    )
    .into();
    assert_json(
        &template,
        json!({
            "type": "confirm",
            "text": "Are you sure?",
            "actions": [
                {"type": "message", "label": "Yes", "text": "yes"},
                {"type": "message", "label": "No", "text": "no"}
            ]
        }),
    );
}

#[test]
fn carousel_template() {
    let template: Template = CarouselTemplate::new([
        CarouselColumn::new(
            "description",
            [PostbackAction::new("action=buy&itemid=111").label("Buy")],
        )
        .thumbnail_image_url("https://example.com/bot/images/item1.jpg")
        .image_background_color("#FFFFFF")
        .title("this is menu")
        .default_action(UriAction::new("http://example.com/page/123").label("View detail")),
        CarouselColumn::new(
            "description 2",
            [PostbackAction::new("action=buy&itemid=222").label("Buy")],
        ),
    ])
    .image_aspect_ratio("rectangle")
    .image_size("cover")
    .into();
    assert_json(
        &template,
        json!({
            "type": "carousel",
            "columns": [
                {
                    "thumbnailImageUrl": "https://example.com/bot/images/item1.jpg",
                    "imageBackgroundColor": "#FFFFFF",
                    "title": "this is menu",
                    "text": "description",
                    "defaultAction": {"type": "uri", "label": "View detail", "uri": "http://example.com/page/123"},
                    "actions": [{"type": "postback", "label": "Buy", "data": "action=buy&itemid=111"}]
                },
                {
                    "text": "description 2",
                    "actions": [{"type": "postback", "label": "Buy", "data": "action=buy&itemid=222"}]
                }
            ],
            "imageAspectRatio": "rectangle",
            "imageSize": "cover"
        }),
    );
}

#[test]
fn image_carousel_template() {
    let template: Template = ImageCarouselTemplate::new([ImageCarouselColumn::new(
        "https://example.com/bot/images/item1.jpg",
        PostbackAction::new("action=buy&itemid=111").label("Buy"),
    )])
    .into();
    assert_json(
        &template,
        json!({
            "type": "image_carousel",
            "columns": [{
                "imageUrl": "https://example.com/bot/images/item1.jpg",
                "action": {"type": "postback", "label": "Buy", "data": "action=buy&itemid=111"}
            }]
        }),
    );
}

#[test]
fn template_tolerates_unknown_type() {
    let raw = json!({"type": "list", "items": []});
    let template: Template = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(template, Template::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&template).unwrap(), raw);
}

#[test]
fn unknown_message_round_trips() {
    let raw = json!({"type": "hologram", "url": "https://example.com/h", "nested": {"a": [1, 2]}});
    let message: Message = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(message, Message::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&message).unwrap(), raw);
}

#[test]
fn malformed_known_message_is_error() {
    let err = serde_json::from_value::<Message>(json!({"type": "text"})).unwrap_err();
    assert!(err.to_string().contains("missing field `text`"), "{err}");
    let err = serde_json::from_value::<Message>(
        json!({"type": "sticker", "packageId": 1, "stickerId": "1"}),
    )
    .unwrap_err();
    assert!(err.to_string().contains("invalid type"), "{err}");
}

#[test]
fn message_without_type_is_error() {
    let err = serde_json::from_value::<Message>(json!({"text": "hi"})).unwrap_err();
    assert!(err.to_string().contains("missing field `type`"), "{err}");
    assert!(serde_json::from_value::<Message>(json!("text")).is_err());
}

#[test]
fn invalid_json_is_error() {
    assert!(serde_json::from_str::<Message>(r#"{"type":"text","#).is_err());
}

#[test]
fn message_decodes_from_str() {
    let message: Message =
        serde_json::from_str(r#"{"type":"text","text":"hi","quoteToken":"q"}"#).unwrap();
    assert_eq!(message, TextMessage::new("hi").quote_token("q").into());
}
