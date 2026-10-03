# line-bot-messaging-api

[![crates.io](https://img.shields.io/crates/v/line-bot-messaging-api.svg)](https://crates.io/crates/line-bot-messaging-api)
[![downloads](https://img.shields.io/crates/d/line-bot-messaging-api.svg)](https://crates.io/crates/line-bot-messaging-api)
[![docs.rs](https://img.shields.io/docsrs/line-bot-messaging-api)](https://docs.rs/line-bot-messaging-api)
[![CI](https://github.com/uiuifree/rust-line-messaging-api/actions/workflows/ci.yml/badge.svg)](https://github.com/uiuifree/rust-line-messaging-api/actions/workflows/ci.yml)
[![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)](https://blog.rust-lang.org/)
[![license: MIT](https://img.shields.io/crates/l/line-bot-messaging-api.svg)](https://github.com/uiuifree/rust-line-messaging-api/blob/main/LICENSE)

**Typed, async Rust client for the LINE Messaging API.** Send messages, manage rich menus,
read insights and audiences, and receive webhooks with signature verification — every
request and response is a Rust type written by hand from LINE's official OpenAPI
specification. This is an unofficial client, not provided by LINE.

[日本語 README](https://github.com/uiuifree/rust-line-messaging-api/blob/main/README.md) · [API documentation](https://docs.rs/line-bot-messaging-api) · [llms.txt](https://github.com/uiuifree/rust-line-messaging-api/blob/main/llms.txt) · [Changelog](https://github.com/uiuifree/rust-line-messaging-api/blob/main/CHANGELOG.md)

## Why this crate

- **Covers the official spec.** 95 of the 102 operations in LINE's
  [OpenAPI specification](https://github.com/line/line-openapi) for the Messaging API,
  Insight, Audience management and Channel access token are implemented — everything
  except the LINE notification message (PNP) and membership endpoints.
- **Method names match the official reference.** Each method is the `operationId` in
  snake_case (`pushMessage` → `push_message`, `getRichMenuList` → `get_rich_menu_list`),
  so the LINE documentation maps one-to-one onto this crate.
- **Typed messages.** Text, text v2 (mentions/emoji substitution), sticker, image, video,
  audio, location, imagemap, templates, Flex (every container and component), quick
  replies and all action types. Flex JSON from the Flex Message Simulator deserializes as is.
- **Typed webhooks.** All webhook event types with `x-line-signature` verification built in.
- **Forward compatible.** A message, action, Flex component or webhook event whose `type`
  this version does not know becomes `Unknown(serde_json::Value)` instead of failing the
  whole payload, and serializes back unchanged.
- **Safe defaults.** Path parameters are percent-encoded, the channel access token is
  redacted from `Debug`, retry keys (`X-Line-Retry-Key`) and request IDs are first-class.

## Install

```toml
[dependencies]
line-bot-messaging-api = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

`reqwest` and `serde_json` are re-exported (`line_bot_messaging_api::reqwest`,
`line_bot_messaging_api::serde_json`), so you do not need to match their versions.

## Examples

- [`examples/echo_bot.rs`](https://github.com/uiuifree/rust-line-messaging-api/blob/main/examples/echo_bot.rs): an echo bot that replies with the received text (webhook server on axum)
- [`examples/push_message.rs`](https://github.com/uiuifree/rust-line-messaging-api/blob/main/examples/push_message.rs): push a message with quick reply buttons

```sh
LINE_CHANNEL_SECRET=... LINE_CHANNEL_ACCESS_TOKEN=... cargo run --example echo_bot
```

## Send a message

```rust,no_run
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::PushMessageRequest;
use line_bot_messaging_api::message::{PostbackAction, QuickReply, QuickReplyItem, TextMessage};

# async fn run() -> line_bot_messaging_api::Result<()> {
let client = LineClient::new("CHANNEL_ACCESS_TOKEN");

let text = TextMessage::new("Which one?").quick_reply(QuickReply::new([
    QuickReplyItem::new(PostbackAction::new("choice=a").label("A")),
    QuickReplyItem::new(PostbackAction::new("choice=b").label("B")),
]));

let request = PushMessageRequest::new("U0123456789abcdef", [text])
    // A retry key makes resending the same request safe (generate your own UUID).
    .retry_key("123e4567-e89b-12d3-a456-426614174000");
let response = client.push_message(&request).await?;
println!("sent: {:?}, request id: {:?}", response.sent_messages, response.request_id);
# Ok(())
# }
```

## Receive webhooks

Verify the signature and parse the events in one call. Pass the raw request body bytes,
before any JSON parsing.

```rust,no_run
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::ReplyMessageRequest;
use line_bot_messaging_api::message::TextMessage;
use line_bot_messaging_api::webhook::{self, Event, MessageContent};

# async fn handle(client: &LineClient, body: &[u8], signature: &str) -> line_bot_messaging_api::Result<()> {
let request = webhook::parse_request("CHANNEL_SECRET", body, signature)?;
for event in request.events {
    match event {
        Event::Message(event) => {
            if let (Some(token), MessageContent::Text(text)) = (&event.reply_token, &event.message) {
                let reply = ReplyMessageRequest::new(token, [TextMessage::new(&text.text)]);
                client.reply_message(&reply).await?;
            }
        }
        Event::Follow(event) => println!("followed by {:?}", event.common.source),
        Event::Unknown(raw) => println!("event type not supported yet: {raw}"),
        _ => {}
    }
}
# Ok(())
# }
```

## Flex Message

Build it with types, or load JSON designed in the
[Flex Message Simulator](https://developers.line.biz/flex-simulator/):

```rust
use line_bot_messaging_api::message::{FlexContainer, FlexMessage};
use line_bot_messaging_api::serde_json;

let container: FlexContainer = serde_json::from_str(r#"{
  "type": "bubble",
  "body": {
    "type": "box",
    "layout": "vertical",
    "contents": [{ "type": "text", "text": "Hello, Flex!" }]
  }
}"#).unwrap();
let message = FlexMessage::new("Hello", container);
```

## Rich menus

```rust,no_run
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::{RichMenuArea, RichMenuBounds, RichMenuRequest, RichMenuSize};
use line_bot_messaging_api::message::UriAction;

# async fn run(client: &LineClient, image: Vec<u8>) -> line_bot_messaging_api::Result<()> {
let menu = RichMenuRequest::new(
    RichMenuSize::new(2500, 843),
    true,
    "main",
    "Menu",
    [RichMenuArea::new(
        RichMenuBounds::new(0, 0, 2500, 843),
        UriAction::new("https://example.com").label("Web"),
    )],
);
let created = client.create_rich_menu(&menu).await?;
client.set_rich_menu_image(&created.rich_menu_id, image, "image/png").await?;
client.set_default_rich_menu(&created.rich_menu_id).await?;
# Ok(())
# }
```

## Errors

A non-2xx response becomes `Error::Api`, carrying the status, the `x-line-request-id`
header and LINE's error details.

```rust,no_run
# async fn run(client: &line_bot_messaging_api::LineClient) {
match client.get_profile("U0123456789abcdef").await {
    Ok(profile) => println!("{}", profile.display_name),
    Err(err) => match err.api_error() {
        Some(api) => eprintln!("{} {} {:?}", api.status, api.body.message, api.body.details),
        None => eprintln!("{err}"),
    },
}
# }
```

## Configuration

By default the client gives up when a connection takes more than 5 seconds or the server
sends nothing for 10 seconds (there is no limit on the total time, so large content
downloads are not cut off). Pass your own `reqwest::Client` to change timeouts or use a
proxy. Base URLs can be overridden,
e.g. to point the client at a mock server in tests.

```rust,no_run
use std::time::Duration;
use line_bot_messaging_api::{LineClient, reqwest};

# fn run() -> line_bot_messaging_api::Result<()> {
let client = LineClient::builder()
    .channel_access_token("CHANNEL_ACCESS_TOKEN")
    .http_client(reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?)
    .build()?;
# Ok(())
# }
```

The channel access token endpoints (`issue_channel_token` etc.) do not send an
`Authorization` header, so they work from a client built without a token
(`LineClient::builder().build()?`).

## Coverage

| Area | Endpoints |
|---|---|
| Messages | reply / push / multicast / narrowcast (+ progress) / broadcast, validation, delivery counts, quota, aggregation units, mark as read, loading animation |
| Content | message content, preview, transcoding status |
| Users, groups, rooms | profile, follower IDs, bot info, member IDs/count/profile, leave, group summary |
| Rich menus | create, validate, image upload/download, get, delete, list, default, aliases, per-user and bulk link/unlink, batch |
| Account link | issue link token |
| Webhook | endpoint get/set/test, signature verification, all event types |
| Insight | demographics, followers, deliveries, message events, unit statistics, rich menu statistics |
| Audience | create (user IDs, file, click, impression), add, update description, delete, get, shared audiences |
| Coupons | create, list, detail, close |
| Channel access token | v2.1 (JWT), stateless and short-lived tokens: issue, verify, revoke |

## Development

```sh
cargo test                                  # no network access; LINE is mocked with wiremock
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo llvm-cov --summary-only
```

## License

MIT — see [LICENSE](https://github.com/uiuifree/rust-line-messaging-api/blob/main/LICENSE).
