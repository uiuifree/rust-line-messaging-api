# line-bot-messaging-api

[![crates.io](https://img.shields.io/crates/v/line-bot-messaging-api.svg)](https://crates.io/crates/line-bot-messaging-api)
[![downloads](https://img.shields.io/crates/d/line-bot-messaging-api.svg)](https://crates.io/crates/line-bot-messaging-api)
[![docs.rs](https://img.shields.io/docsrs/line-bot-messaging-api)](https://docs.rs/line-bot-messaging-api)
[![CI](https://github.com/uiuifree/rust-line-messaging-api/actions/workflows/ci.yml/badge.svg)](https://github.com/uiuifree/rust-line-messaging-api/actions/workflows/ci.yml)
[![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)](https://blog.rust-lang.org/)
[![license: MIT](https://img.shields.io/crates/l/line-bot-messaging-api.svg)](https://github.com/uiuifree/rust-line-messaging-api/blob/main/LICENSE)

[English README](https://github.com/uiuifree/rust-line-messaging-api/blob/main/README.en.md) · [API ドキュメント](https://docs.rs/line-bot-messaging-api) · [llms.txt](https://github.com/uiuifree/rust-line-messaging-api/blob/main/llms.txt) · [変更履歴](https://github.com/uiuifree/rust-line-messaging-api/blob/main/CHANGELOG.md)

LINE Bot（LINE 公式アカウント）を Rust で作るための、[LINE Messaging API](https://developers.line.biz/ja/reference/messaging-api/) の非同期クライアントです。リクエストとレスポンスをすべて型で扱えます。LINE 公式のものではない、非公式のクライアントです。

- LINE 公式の OpenAPI 仕様（[line/line-openapi](https://github.com/line/line-openapi)）をもとに、Messaging API・インサイト・オーディエンス管理・チャネルアクセストークンの操作 102 件のうち 95 件を実装しています（除いたのは LINE 通知メッセージ（PNP）と membership の 7 件）
- メソッド名は公式仕様の `operationId` を snake_case にしたものです（`pushMessage` → `push_message`）。公式ドキュメントからそのまま引けます
- メッセージ（テキスト・Flex・テンプレート・イメージマップ・クイックリプライ…）と Webhook イベントはすべて型で表します
- LINE が新しいメッセージ種別やイベントを追加しても、知らない種別は `Unknown` として受け取るので解析全体は失敗しません
- Webhook の署名検証（`x-line-signature`）を内蔵しています

## インストール

```toml
[dependencies]
line-bot-messaging-api = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

`reqwest` と `serde_json` は再公開しているので（`line_bot_messaging_api::reqwest` / `line_bot_messaging_api::serde_json`）、バージョンを合わせて追加する必要はありません。

## サンプル

- [`examples/echo_bot.rs`](https://github.com/uiuifree/rust-line-messaging-api/blob/main/examples/echo_bot.rs): 受け取ったテキストをそのまま返すオウム返し Bot（axum で Webhook を受ける）
- [`examples/push_message.rs`](https://github.com/uiuifree/rust-line-messaging-api/blob/main/examples/push_message.rs): クイックリプライ付きのメッセージを push で送る

```sh
LINE_CHANNEL_SECRET=... LINE_CHANNEL_ACCESS_TOKEN=... cargo run --example echo_bot
```

## メッセージを送る

```rust,no_run
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::PushMessageRequest;
use line_bot_messaging_api::message::{PostbackAction, QuickReply, QuickReplyItem, TextMessage};

# async fn run() -> line_bot_messaging_api::Result<()> {
let client = LineClient::new("CHANNEL_ACCESS_TOKEN");

let text = TextMessage::new("どちらにしますか？").quick_reply(QuickReply::new([
    QuickReplyItem::new(PostbackAction::new("choice=a").label("A")),
    QuickReplyItem::new(PostbackAction::new("choice=b").label("B")),
]));

let request = PushMessageRequest::new("U0123456789abcdef", [text])
    // 同じリクエストを再送しても二重に届かないようにする（UUID を自分で発行する）
    .retry_key("123e4567-e89b-12d3-a456-426614174000");
let response = client.push_message(&request).await?;
println!("sent: {:?}, request id: {:?}", response.sent_messages, response.request_id);
# Ok(())
# }
```

## Webhook を受ける

署名を検証してからイベントを解析します。`body` は受け取ったリクエストボディのバイト列そのもの（解析前）を渡してください。

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
        Event::Unknown(raw) => println!("unsupported event: {raw}"),
        _ => {}
    }
}
# Ok(())
# }
```

## Flex Message

型を組み立てることも、[Flex Message Simulator](https://developers.line.biz/flex-simulator/) で作った JSON をそのまま読み込むこともできます。

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

## リッチメニュー

```rust,no_run
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::{RichMenuArea, RichMenuBounds, RichMenuRequest, RichMenuSize};
use line_bot_messaging_api::message::UriAction;

# async fn run(client: &LineClient, image: Vec<u8>) -> line_bot_messaging_api::Result<()> {
let menu = RichMenuRequest::new(
    RichMenuSize::new(2500, 843),
    true,
    "main",
    "メニュー",
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

## エラー

API が 2xx 以外を返したときは `Error::Api` になり、ステータス・`x-line-request-id`・LINE のエラー詳細を取り出せます。

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

## 設定

既定では、接続に 5 秒、応答が 10 秒途切れたら打ち切ります（大きなコンテンツのダウンロードが途中で切れないよう、全体の時間には上限を設けていません）。タイムアウトやプロキシを変えるときは `reqwest::Client` を渡します（`line_bot_messaging_api::reqwest` として再公開しているので、バージョンを合わせる必要はありません）。接続先の URL も差し替えられます（テストでモックサーバーに向けるときなど）。

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

チャネルアクセストークンの発行系（`issue_channel_token` など）は `Authorization` ヘッダーを使わないので、トークンなしのクライアント（`LineClient::builder().build()?`）から呼べます。

## 対応範囲

| 分類 | 内容 |
|---|---|
| メッセージ送信 | reply / push / multicast / narrowcast（進捗取得）/ broadcast、検証、送信数・上限・集計単位、既読・ローディング表示 |
| コンテンツ | 受信したメッセージのコンテンツ・プレビュー・変換状況の取得 |
| ユーザー・グループ・トークルーム | プロフィール、友だちの ID 一覧、ボット情報、メンバー一覧・人数、退出、グループ概要 |
| リッチメニュー | 作成・検証・画像・取得・削除・一覧、デフォルト設定、エイリアス、ユーザーとの紐付け（一括・バッチ） |
| アカウント連携 | 連携トークンの発行 |
| Webhook | エンドポイントの設定・取得・テスト、署名検証、全イベント型 |
| インサイト | 友だちの属性・数、配信数、メッセージ／集計単位ごとの統計、リッチメニューの統計 |
| オーディエンス管理 | 作成（ID・ファイル・クリック・インプレッション）、追加、説明の変更、削除、取得、共有オーディエンス |
| クーポン | 作成・一覧・詳細・終了 |
| チャネルアクセストークン | v2.1（JWT）・ステートレス・短期トークンの発行・検証・取り消し |

## 開発

```sh
cargo test                                  # ネットワークを使わない（LINE は wiremock で代用）
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo llvm-cov --summary-only
```

## ライセンス

MIT（[LICENSE](https://github.com/uiuifree/rust-line-messaging-api/blob/main/LICENSE)）
