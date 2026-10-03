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

/// A bubble as exported by the Flex Message Simulator ("Restaurant" style).
const SIMULATOR_BUBBLE: &str = r##"{
  "type": "bubble",
  "hero": {
    "type": "image",
    "url": "https://developers-resource.landpress.line.me/fx/img/01_1_cafe.png",
    "size": "full",
    "aspectRatio": "20:13",
    "aspectMode": "cover",
    "action": {"type": "uri", "uri": "https://line.me/"}
  },
  "body": {
    "type": "box",
    "layout": "vertical",
    "contents": [
      {"type": "text", "text": "Brown Cafe", "weight": "bold", "size": "xl"},
      {
        "type": "box",
        "layout": "baseline",
        "margin": "md",
        "contents": [
          {"type": "icon", "size": "sm", "url": "https://developers-resource.landpress.line.me/fx/img/review_gold_star_28.png"},
          {"type": "icon", "size": "sm", "url": "https://developers-resource.landpress.line.me/fx/img/review_gray_star_28.png"},
          {"type": "text", "text": "4.0", "size": "sm", "color": "#999999", "margin": "md", "flex": 0}
        ]
      },
      {
        "type": "box",
        "layout": "vertical",
        "margin": "lg",
        "spacing": "sm",
        "contents": [
          {
            "type": "box",
            "layout": "baseline",
            "spacing": "sm",
            "contents": [
              {"type": "text", "text": "Place", "color": "#aaaaaa", "size": "sm", "flex": 1},
              {"type": "text", "text": "Flex Tower, 7-7-4 Midori-ku, Tokyo", "wrap": true, "color": "#666666", "size": "sm", "flex": 5}
            ]
          }
        ]
      }
    ]
  },
  "footer": {
    "type": "box",
    "layout": "vertical",
    "spacing": "sm",
    "contents": [
      {"type": "button", "style": "link", "height": "sm", "action": {"type": "uri", "label": "CALL", "uri": "https://line.me/"}},
      {"type": "button", "style": "link", "height": "sm", "action": {"type": "uri", "label": "WEBSITE", "uri": "https://line.me/"}},
      {"type": "box", "layout": "vertical", "contents": [], "margin": "sm"}
    ],
    "flex": 0
  }
}"##;

/// A carousel as exported by the Flex Message Simulator, including spans, gradients,
/// absolute positioning and a component type this crate does not know.
const SIMULATOR_CAROUSEL: &str = r##"{
  "type": "carousel",
  "contents": [
    {
      "type": "bubble",
      "size": "micro",
      "hero": {"type": "image", "url": "https://example.com/1.jpg", "size": "full", "aspectMode": "cover", "aspectRatio": "320:213"},
      "body": {
        "type": "box",
        "layout": "vertical",
        "contents": [
          {
            "type": "text",
            "contents": [
              {"type": "span", "text": "Hello, "},
              {"type": "span", "text": "world", "color": "#ff0000", "weight": "bold", "style": "italic", "decoration": "underline", "size": "xl"}
            ],
            "size": "sm",
            "wrap": true
          },
          {
            "type": "box",
            "layout": "horizontal",
            "position": "absolute",
            "offsetTop": "18px",
            "offsetStart": "18px",
            "backgroundColor": "#ff334b",
            "cornerRadius": "20px",
            "height": "25px",
            "width": "53px",
            "contents": [{"type": "text", "text": "SALE", "color": "#ffffff", "align": "center", "size": "xs", "offsetTop": "3px"}]
          },
          {"type": "spacer", "size": "xxl"}
        ],
        "spacing": "sm",
        "paddingAll": "13px",
        "background": {"type": "linearGradient", "angle": "0deg", "startColor": "#ff0000", "endColor": "#0000ff"}
      }
    },
    {
      "type": "bubble",
      "direction": "ltr",
      "styles": {"footer": {"separator": true, "separatorColor": "#cccccc"}},
      "body": {
        "type": "box",
        "layout": "vertical",
        "justifyContent": "space-between",
        "alignItems": "flex-start",
        "contents": [
          {"type": "separator", "margin": "md", "color": "#cccccc"},
          {"type": "filler", "flex": 2}
        ]
      },
      "footer": {
        "type": "box",
        "layout": "horizontal",
        "contents": [
          {"type": "button", "action": {"type": "message", "label": "Yes", "text": "yes"}, "style": "primary", "color": "#905c44", "adjustMode": "shrink-to-fit"}
        ]
      },
      "action": {"type": "postback", "label": "tap", "data": "bubble=2"}
    }
  ]
}"##;

#[test]
fn simulator_bubble_round_trips() {
    let raw: Value = serde_json::from_str(SIMULATOR_BUBBLE).unwrap();
    let container: FlexContainer = serde_json::from_str(SIMULATOR_BUBBLE).unwrap();
    assert_eq!(serde_json::to_value(&container).unwrap(), raw);

    let FlexContainer::Bubble(bubble) = &container else {
        panic!("expected a bubble: {container:?}");
    };
    let hero = bubble.hero.as_ref().unwrap();
    let FlexComponent::Image(image) = hero else {
        panic!("expected an image: {hero:?}");
    };
    assert_eq!(image.aspect_mode, Some(FlexAspectMode::Cover));
    assert_eq!(image.size.as_deref(), Some("full"));
    assert_eq!(bubble.footer.as_ref().unwrap().flex, Some(0));
}

#[test]
fn simulator_carousel_round_trips() {
    let raw: Value = serde_json::from_str(SIMULATOR_CAROUSEL).unwrap();
    let container: FlexContainer = serde_json::from_str(SIMULATOR_CAROUSEL).unwrap();
    assert_eq!(serde_json::to_value(&container).unwrap(), raw);

    let FlexContainer::Carousel(carousel) = &container else {
        panic!("expected a carousel: {container:?}");
    };
    let body = carousel.contents[0].body.as_ref().unwrap();
    assert_eq!(
        body.contents[2],
        FlexComponent::Unknown(json!({"type": "spacer", "size": "xxl"}))
    );
    let message = FlexMessage::new("carousel", container.clone());
    assert_eq!(
        serde_json::to_value(&message).unwrap(),
        json!({"type": "flex", "altText": "carousel", "contents": raw})
    );
}

#[test]
fn flex_message_setters() {
    let message: Message = FlexMessage::new("alt", FlexBubble::default())
        .quick_reply(QuickReply::new([QuickReplyItem::new(MessageAction::new(
            "Hi",
        ))]))
        .sender(Sender::default().icon_url("https://example.com/i.png"))
        .into();
    assert_json(
        &message,
        json!({
            "type": "flex",
            "altText": "alt",
            "contents": {"type": "bubble"},
            "quickReply": {"items": [{"type": "action", "action": {"type": "message", "text": "Hi"}}]},
            "sender": {"iconUrl": "https://example.com/i.png"}
        }),
    );
}

#[test]
fn bubble_all_fields() {
    let style = FlexBlockStyle::default()
        .background_color("#ffffff")
        .separator(true)
        .separator_color("#000000");
    let bubble = FlexBubble::default()
        .direction(FlexDirection::Rtl)
        .styles(
            FlexBubbleStyles::default()
                .header(style.clone())
                .hero(style.clone())
                .body(style.clone())
                .footer(style),
        )
        .header(FlexBox::new(
            FlexBoxLayout::Vertical,
            [FlexText::new("header")],
        ))
        .hero(FlexImage::new("https://example.com/hero.png"))
        .body(FlexBox::new(
            FlexBoxLayout::Vertical,
            [FlexText::new("body")],
        ))
        .footer(FlexBox::new(
            FlexBoxLayout::Horizontal,
            [FlexFiller::default()],
        ))
        .size(FlexBubbleSize::Giga)
        .action(UriAction::new("https://example.com"));
    let style_json =
        json!({"backgroundColor": "#ffffff", "separator": true, "separatorColor": "#000000"});
    let container = FlexContainer::from(bubble);
    assert_json(
        &container,
        json!({
            "type": "bubble",
            "direction": "rtl",
            "styles": {"header": style_json, "hero": style_json, "body": style_json, "footer": style_json},
            "header": {"type": "box", "layout": "vertical", "contents": [{"type": "text", "text": "header"}]},
            "hero": {"type": "image", "url": "https://example.com/hero.png"},
            "body": {"type": "box", "layout": "vertical", "contents": [{"type": "text", "text": "body"}]},
            "footer": {"type": "box", "layout": "horizontal", "contents": [{"type": "filler"}]},
            "size": "giga",
            "action": {"type": "uri", "uri": "https://example.com"}
        }),
    );
}

#[test]
fn carousel_from_bubbles() {
    let container: FlexContainer =
        FlexCarousel::new([FlexBubble::default(), FlexBubble::default()]).into();
    assert_json(
        &container,
        json!({"type": "carousel", "contents": [{"type": "bubble"}, {"type": "bubble"}]}),
    );
}

#[test]
fn box_all_fields() {
    let component: FlexComponent =
        FlexBox::new(FlexBoxLayout::Baseline, [FlexSeparator::default()])
            .flex(1)
            .spacing("md")
            .margin("lg")
            .position(FlexPosition::Absolute)
            .offset_top("1px")
            .offset_bottom("2px")
            .offset_start("3px")
            .offset_end("4px")
            .background_color("#111111")
            .border_color("#222222")
            .border_width("light")
            .corner_radius("md")
            .width("50%")
            .max_width("100px")
            .height("40px")
            .max_height("80px")
            .padding_all("5px")
            .padding_top("6px")
            .padding_bottom("7px")
            .padding_start("8px")
            .padding_end("9px")
            .action(PostbackAction::new("box"))
            .justify_content(FlexJustifyContent::SpaceEvenly)
            .align_items(FlexAlignItems::FlexEnd)
            .background(
                FlexBoxLinearGradient::new("90deg", "#000000", "#ffffff")
                    .center_color("#888888")
                    .center_position("30%"),
            )
            .into();
    assert_json(
        &component,
        json!({
            "type": "box",
            "layout": "baseline",
            "flex": 1,
            "contents": [{"type": "separator"}],
            "spacing": "md",
            "margin": "lg",
            "position": "absolute",
            "offsetTop": "1px",
            "offsetBottom": "2px",
            "offsetStart": "3px",
            "offsetEnd": "4px",
            "backgroundColor": "#111111",
            "borderColor": "#222222",
            "borderWidth": "light",
            "cornerRadius": "md",
            "width": "50%",
            "maxWidth": "100px",
            "height": "40px",
            "maxHeight": "80px",
            "paddingAll": "5px",
            "paddingTop": "6px",
            "paddingBottom": "7px",
            "paddingStart": "8px",
            "paddingEnd": "9px",
            "action": {"type": "postback", "data": "box"},
            "justifyContent": "space-evenly",
            "alignItems": "flex-end",
            "background": {
                "type": "linearGradient",
                "angle": "90deg",
                "startColor": "#000000",
                "endColor": "#ffffff",
                "centerColor": "#888888",
                "centerPosition": "30%"
            }
        }),
    );
}

#[test]
fn box_background_tolerates_unknown_type() {
    let raw = json!({"type": "radialGradient", "startColor": "#000000"});
    let background: FlexBoxBackground = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(background, FlexBoxBackground::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&background).unwrap(), raw);
    assert_json(
        &FlexBoxLinearGradient::default(),
        json!({"type": "linearGradient"}),
    );
}

#[test]
fn button_all_fields() {
    let component: FlexComponent = FlexButton::new(MessageAction::new("ok"))
        .flex(2)
        .color("#00ff00")
        .style(FlexButtonStyle::Secondary)
        .gravity(FlexGravity::Bottom)
        .margin("sm")
        .position(FlexPosition::Relative)
        .offset_top("1px")
        .offset_bottom("2px")
        .offset_start("3px")
        .offset_end("4px")
        .height(FlexButtonHeight::Md)
        .adjust_mode(FlexAdjustMode::ShrinkToFit)
        .scaling(true)
        .into();
    assert_json(
        &component,
        json!({
            "type": "button",
            "flex": 2,
            "color": "#00ff00",
            "style": "secondary",
            "action": {"type": "message", "text": "ok"},
            "gravity": "bottom",
            "margin": "sm",
            "position": "relative",
            "offsetTop": "1px",
            "offsetBottom": "2px",
            "offsetStart": "3px",
            "offsetEnd": "4px",
            "height": "md",
            "adjustMode": "shrink-to-fit",
            "scaling": true
        }),
    );
}

#[test]
fn image_all_fields() {
    let component: FlexComponent = FlexImage::new("https://example.com/a.png")
        .flex(3)
        .margin("none")
        .position(FlexPosition::Absolute)
        .offset_top("1px")
        .offset_bottom("2px")
        .offset_start("3px")
        .offset_end("4px")
        .align(FlexAlign::End)
        .gravity(FlexGravity::Top)
        .size("5xl")
        .aspect_ratio("1.51:1")
        .aspect_mode(FlexAspectMode::Fit)
        .background_color("#cccccc")
        .action(UriAction::new("https://example.com"))
        .animated(true)
        .into();
    assert_json(
        &component,
        json!({
            "type": "image",
            "url": "https://example.com/a.png",
            "flex": 3,
            "margin": "none",
            "position": "absolute",
            "offsetTop": "1px",
            "offsetBottom": "2px",
            "offsetStart": "3px",
            "offsetEnd": "4px",
            "align": "end",
            "gravity": "top",
            "size": "5xl",
            "aspectRatio": "1.51:1",
            "aspectMode": "fit",
            "backgroundColor": "#cccccc",
            "action": {"type": "uri", "uri": "https://example.com"},
            "animated": true
        }),
    );
}

#[test]
fn video_all_fields() {
    let component: FlexComponent = FlexVideo::new(
        "https://example.com/video.mp4",
        "https://example.com/preview.png",
        FlexImage::new("https://example.com/alt.png"),
    )
    .aspect_ratio("20:13")
    .action(UriAction::new("https://example.com").label("More"))
    .into();
    assert_json(
        &component,
        json!({
            "type": "video",
            "url": "https://example.com/video.mp4",
            "previewUrl": "https://example.com/preview.png",
            "altContent": {"type": "image", "url": "https://example.com/alt.png"},
            "aspectRatio": "20:13",
            "action": {"type": "uri", "label": "More", "uri": "https://example.com"}
        }),
    );
}

#[test]
fn icon_all_fields() {
    let component: FlexComponent = FlexIcon::new("https://example.com/icon.png")
        .size("xxs")
        .aspect_ratio("2:1")
        .margin("xs")
        .position(FlexPosition::Relative)
        .offset_top("1px")
        .offset_bottom("2px")
        .offset_start("3px")
        .offset_end("4px")
        .scaling(false)
        .into();
    assert_json(
        &component,
        json!({
            "type": "icon",
            "url": "https://example.com/icon.png",
            "size": "xxs",
            "aspectRatio": "2:1",
            "margin": "xs",
            "position": "relative",
            "offsetTop": "1px",
            "offsetBottom": "2px",
            "offsetStart": "3px",
            "offsetEnd": "4px",
            "scaling": false
        }),
    );
}

#[test]
fn text_all_fields() {
    let component: FlexComponent = FlexText::default()
        .flex(4)
        .text("hello")
        .size("lg")
        .align(FlexAlign::Start)
        .gravity(FlexGravity::Center)
        .color("#123456")
        .weight(FlexFontWeight::Regular)
        .style(FlexFontStyle::Normal)
        .decoration(FlexTextDecoration::LineThrough)
        .wrap(true)
        .line_spacing("20px")
        .margin("xxl")
        .position(FlexPosition::Relative)
        .offset_top("1px")
        .offset_bottom("2px")
        .offset_start("3px")
        .offset_end("4px")
        .action(MessageAction::new("hello"))
        .max_lines(2)
        .contents([
            FlexSpan::new("he"),
            FlexSpan::default()
                .text("llo")
                .size("sm")
                .color("#654321")
                .weight(FlexFontWeight::Bold)
                .style(FlexFontStyle::Italic)
                .decoration(FlexTextDecoration::None),
        ])
        .adjust_mode(FlexAdjustMode::ShrinkToFit)
        .scaling(true)
        .into();
    assert_json(
        &component,
        json!({
            "type": "text",
            "flex": 4,
            "text": "hello",
            "size": "lg",
            "align": "start",
            "gravity": "center",
            "color": "#123456",
            "weight": "regular",
            "style": "normal",
            "decoration": "line-through",
            "wrap": true,
            "lineSpacing": "20px",
            "margin": "xxl",
            "position": "relative",
            "offsetTop": "1px",
            "offsetBottom": "2px",
            "offsetStart": "3px",
            "offsetEnd": "4px",
            "action": {"type": "message", "text": "hello"},
            "maxLines": 2,
            "contents": [
                {"type": "span", "text": "he"},
                {
                    "type": "span",
                    "text": "llo",
                    "size": "sm",
                    "color": "#654321",
                    "weight": "bold",
                    "style": "italic",
                    "decoration": "none"
                }
            ],
            "adjustMode": "shrink-to-fit",
            "scaling": true
        }),
    );
}

#[test]
fn span_separator_filler_conversions() {
    let components: Vec<FlexComponent> = vec![
        FlexSpan::new("s").into(),
        FlexSeparator::default()
            .margin("md")
            .color("#eeeeee")
            .into(),
        FlexFiller::default().flex(1).into(),
    ];
    assert_json(
        &components,
        json!([
            {"type": "span", "text": "s"},
            {"type": "separator", "margin": "md", "color": "#eeeeee"},
            {"type": "filler", "flex": 1}
        ]),
    );
}

#[test]
fn enum_values_match_spec() {
    assert_eq!(
        serde_json::to_value([FlexDirection::Ltr, FlexDirection::Rtl]).unwrap(),
        json!(["ltr", "rtl"])
    );
    assert_eq!(
        serde_json::to_value([
            FlexBubbleSize::Nano,
            FlexBubbleSize::Micro,
            FlexBubbleSize::Deca,
            FlexBubbleSize::Hecto,
            FlexBubbleSize::Kilo,
            FlexBubbleSize::Mega,
            FlexBubbleSize::Giga
        ])
        .unwrap(),
        json!(["nano", "micro", "deca", "hecto", "kilo", "mega", "giga"])
    );
    assert_eq!(
        serde_json::to_value([
            FlexBoxLayout::Horizontal,
            FlexBoxLayout::Vertical,
            FlexBoxLayout::Baseline
        ])
        .unwrap(),
        json!(["horizontal", "vertical", "baseline"])
    );
    assert_eq!(
        serde_json::to_value([
            FlexJustifyContent::Center,
            FlexJustifyContent::FlexStart,
            FlexJustifyContent::FlexEnd,
            FlexJustifyContent::SpaceBetween,
            FlexJustifyContent::SpaceAround,
            FlexJustifyContent::SpaceEvenly
        ])
        .unwrap(),
        json!([
            "center",
            "flex-start",
            "flex-end",
            "space-between",
            "space-around",
            "space-evenly"
        ])
    );
    assert_eq!(
        serde_json::to_value([
            FlexAlignItems::Center,
            FlexAlignItems::FlexStart,
            FlexAlignItems::FlexEnd
        ])
        .unwrap(),
        json!(["center", "flex-start", "flex-end"])
    );
    assert_eq!(
        serde_json::to_value([
            FlexButtonStyle::Primary,
            FlexButtonStyle::Secondary,
            FlexButtonStyle::Link
        ])
        .unwrap(),
        json!(["primary", "secondary", "link"])
    );
    assert_eq!(
        serde_json::to_value([FlexButtonHeight::Md, FlexButtonHeight::Sm]).unwrap(),
        json!(["md", "sm"])
    );
    assert_eq!(
        serde_json::to_value([FlexGravity::Top, FlexGravity::Bottom, FlexGravity::Center]).unwrap(),
        json!(["top", "bottom", "center"])
    );
    assert_eq!(
        serde_json::to_value([FlexAlign::Start, FlexAlign::End, FlexAlign::Center]).unwrap(),
        json!(["start", "end", "center"])
    );
    assert_eq!(
        serde_json::to_value([FlexAspectMode::Fit, FlexAspectMode::Cover]).unwrap(),
        json!(["fit", "cover"])
    );
    assert_eq!(
        serde_json::to_value([
            FlexTextDecoration::None,
            FlexTextDecoration::Underline,
            FlexTextDecoration::LineThrough
        ])
        .unwrap(),
        json!(["none", "underline", "line-through"])
    );
}

#[test]
fn enums_tolerate_unknown_values() {
    let decode = |s: &str| json!(s);
    assert_eq!(
        serde_json::from_value::<FlexDirection>(decode("ttb")).unwrap(),
        FlexDirection::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexBubbleSize>(decode("tera")).unwrap(),
        FlexBubbleSize::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexBoxLayout>(decode("grid")).unwrap(),
        FlexBoxLayout::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexPosition>(decode("fixed")).unwrap(),
        FlexPosition::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexJustifyContent>(decode("x")).unwrap(),
        FlexJustifyContent::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexAlignItems>(decode("stretch")).unwrap(),
        FlexAlignItems::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexButtonStyle>(decode("ghost")).unwrap(),
        FlexButtonStyle::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexButtonHeight>(decode("lg")).unwrap(),
        FlexButtonHeight::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexGravity>(decode("middle")).unwrap(),
        FlexGravity::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexAlign>(decode("justify")).unwrap(),
        FlexAlign::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexAdjustMode>(decode("grow")).unwrap(),
        FlexAdjustMode::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexAspectMode>(decode("fill")).unwrap(),
        FlexAspectMode::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexFontWeight>(decode("light")).unwrap(),
        FlexFontWeight::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexFontStyle>(decode("oblique")).unwrap(),
        FlexFontStyle::Unknown
    );
    assert_eq!(
        serde_json::from_value::<FlexTextDecoration>(decode("blink")).unwrap(),
        FlexTextDecoration::Unknown
    );
}

#[test]
fn unknown_container_round_trips() {
    let raw = json!({"type": "grid", "cells": [{"type": "bubble"}]});
    let container: FlexContainer = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(container, FlexContainer::Unknown(raw.clone()));
    assert_eq!(serde_json::to_value(&container).unwrap(), raw);
}

#[test]
fn malformed_known_component_is_error() {
    let err = serde_json::from_value::<FlexComponent>(json!({"type": "box", "layout": "vertical"}))
        .unwrap_err();
    assert!(
        err.to_string().contains("missing field `contents`"),
        "{err}"
    );
    let nested = json!({"type": "bubble", "body": {"type": "box", "layout": "vertical", "contents": [{"type": "image"}]}});
    let err = serde_json::from_value::<FlexContainer>(nested).unwrap_err();
    assert!(err.to_string().contains("missing field `url`"), "{err}");
    let err = serde_json::from_value::<FlexComponent>(json!({"layout": "vertical"})).unwrap_err();
    assert!(err.to_string().contains("missing field `type`"), "{err}");
}
