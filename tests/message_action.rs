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

#[test]
fn postback_action_minimal_is_exact() {
    let action = PostbackAction::new("d").label("l");
    assert_eq!(
        serde_json::to_string(&action).unwrap(),
        r#"{"type":"postback","label":"l","data":"d"}"#
    );
    assert_eq!(
        serde_json::to_string(&Action::from(action)).unwrap(),
        r#"{"type":"postback","label":"l","data":"d"}"#
    );
}

#[test]
fn uri_action_minimal_is_exact() {
    let action = UriAction::new("https://e").label("l");
    assert_eq!(
        serde_json::to_string(&action).unwrap(),
        r#"{"type":"uri","label":"l","uri":"https://e"}"#
    );
    assert_eq!(
        serde_json::to_string(&Action::from(action)).unwrap(),
        r#"{"type":"uri","label":"l","uri":"https://e"}"#
    );
}

#[test]
fn postback_action_all_fields() {
    let action: Action = PostbackAction::default()
        .label("Buy")
        .data("action=buy&itemid=111")
        .display_text("Buy it")
        .text("unused")
        .input_option(PostbackInputOption::OpenKeyboard)
        .fill_in_text("---\nName: ")
        .into();
    assert_json(
        &action,
        json!({
            "type": "postback",
            "label": "Buy",
            "data": "action=buy&itemid=111",
            "displayText": "Buy it",
            "text": "unused",
            "inputOption": "openKeyboard",
            "fillInText": "---\nName: "
        }),
    );
}

#[test]
fn postback_input_option_values() {
    let values = [
        PostbackInputOption::CloseRichMenu,
        PostbackInputOption::OpenRichMenu,
        PostbackInputOption::OpenKeyboard,
        PostbackInputOption::OpenVoice,
    ];
    assert_eq!(
        serde_json::to_value(values).unwrap(),
        json!(["closeRichMenu", "openRichMenu", "openKeyboard", "openVoice"])
    );
    let unknown: PostbackInputOption = serde_json::from_value(json!("openSomething")).unwrap();
    assert_eq!(unknown, PostbackInputOption::Unknown);
}

#[test]
fn uri_action_all_fields() {
    let action: Action = UriAction::default()
        .label("View")
        .uri("https://example.com/page")
        .alt_uri(AltUri::new("https://example.com/pc"))
        .into();
    assert_json(
        &action,
        json!({
            "type": "uri",
            "label": "View",
            "uri": "https://example.com/page",
            "altUri": {"desktop": "https://example.com/pc"}
        }),
    );
    assert_json(&AltUri::default(), json!({}));
}

#[test]
fn message_action() {
    let action: Action = MessageAction::new("Yes").label("Yes!").into();
    assert_json(
        &action,
        json!({"type": "message", "label": "Yes!", "text": "Yes"}),
    );
    let action: Action = MessageAction::default().text("t").into();
    assert_json(&action, json!({"type": "message", "text": "t"}));
}

#[test]
fn datetimepicker_action() {
    let action: Action = DatetimePickerAction::new("storeId=12345", DatetimePickerMode::Datetime)
        .label("Select date")
        .initial("2017-12-25t00:00")
        .max("2018-01-24t23:59")
        .min("2017-12-25t00:00")
        .into();
    assert_json(
        &action,
        json!({
            "type": "datetimepicker",
            "label": "Select date",
            "data": "storeId=12345",
            "mode": "datetime",
            "initial": "2017-12-25t00:00",
            "max": "2018-01-24t23:59",
            "min": "2017-12-25t00:00"
        }),
    );
    let action = DatetimePickerAction::default()
        .data("d")
        .mode(DatetimePickerMode::Date);
    assert_json(
        &action,
        json!({"type": "datetimepicker", "data": "d", "mode": "date"}),
    );
    assert_eq!(
        serde_json::to_value([
            DatetimePickerMode::Date,
            DatetimePickerMode::Time,
            DatetimePickerMode::Datetime
        ])
        .unwrap(),
        json!(["date", "time", "datetime"])
    );
    let unknown: DatetimePickerMode = serde_json::from_value(json!("week")).unwrap();
    assert_eq!(unknown, DatetimePickerMode::Unknown);
}

#[test]
fn camera_camera_roll_location_actions() {
    let action: Action = CameraAction::default().label("Camera").into();
    assert_json(&action, json!({"type": "camera", "label": "Camera"}));
    let action: Action = CameraRollAction::default().label("Camera roll").into();
    assert_json(
        &action,
        json!({"type": "cameraRoll", "label": "Camera roll"}),
    );
    let action: Action = LocationAction::default().label("Location").into();
    assert_json(&action, json!({"type": "location", "label": "Location"}));
}

#[test]
fn richmenuswitch_action() {
    let action: Action = RichMenuSwitchAction::new("richmenu-alias-b", "richmenu-changed-to-b")
        .label("Go to B")
        .into();
    assert_json(
        &action,
        json!({
            "type": "richmenuswitch",
            "label": "Go to B",
            "data": "richmenu-changed-to-b",
            "richMenuAliasId": "richmenu-alias-b"
        }),
    );
    let action = RichMenuSwitchAction::default()
        .data("d")
        .rich_menu_alias_id("a");
    assert_json(
        &action,
        json!({"type": "richmenuswitch", "data": "d", "richMenuAliasId": "a"}),
    );
}

#[test]
fn clipboard_action() {
    let action: Action = ClipboardAction::new("3B48740B").label("Copy").into();
    assert_json(
        &action,
        json!({"type": "clipboard", "label": "Copy", "clipboardText": "3B48740B"}),
    );
}

#[test]
fn clipboard_action_requires_clipboard_text() {
    let err = serde_json::from_value::<Action>(json!({"type": "clipboard", "label": "Copy"}))
        .unwrap_err();
    assert!(err.to_string().contains("clipboardText"), "{err}");
}

#[test]
fn unknown_action_round_trips() {
    let raw = json!({"type": "newAction", "label": "New", "extra": {"x": 1}});
    let action: Action = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(action, Action::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&action).unwrap(), raw);
}

#[test]
fn action_without_type_is_error() {
    let err = serde_json::from_value::<Action>(json!({"label": "x"})).unwrap_err();
    assert!(err.to_string().contains("missing field `type`"), "{err}");
    let err = serde_json::from_value::<Action>(json!({"type": 1})).unwrap_err();
    assert!(err.to_string().contains("missing field `type`"), "{err}");
}

#[test]
fn quick_reply() {
    let quick_reply = QuickReply::new([
        QuickReplyItem::new(MessageAction::new("Sushi").label("Sushi"))
            .image_url("https://example.com/sushi.png"),
        QuickReplyItem::new(CameraAction::default().label("Camera")),
    ]);
    assert_json(
        &quick_reply,
        json!({
            "items": [
                {
                    "type": "action",
                    "imageUrl": "https://example.com/sushi.png",
                    "action": {"type": "message", "label": "Sushi", "text": "Sushi"}
                },
                {"type": "action", "action": {"type": "camera", "label": "Camera"}}
            ]
        }),
    );
    assert_json(&QuickReply::default(), json!({}));
}

#[test]
fn quick_reply_item_setters() {
    let item = QuickReplyItem::default().action(LocationAction::default().label("Location"));
    assert_json(
        &item,
        json!({"type": "action", "action": {"type": "location", "label": "Location"}}),
    );
}

#[test]
fn sender() {
    let sender = Sender::default()
        .name("Cony")
        .icon_url("https://example.com/cony.png");
    assert_json(
        &sender,
        json!({"name": "Cony", "iconUrl": "https://example.com/cony.png"}),
    );
}
