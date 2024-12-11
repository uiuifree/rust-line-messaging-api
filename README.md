# LINE Bot Messaging API for Rust

`line-bot-messaging-api` は、LINE Messaging API を Rust 言語で簡単に利用するためのライブラリです。このライブラリを使用することで、LINE Bot の作成や操作を簡単に行うことができます。

## 特徴

- LINE Messaging API の主要な機能をサポート
- シンプルで直感的な API 設計
- 非同期処理をサポート
- Rust のエコシステムに完全対応

## インストール

`Cargo.toml` に以下を追加してください。

```toml
[dependencies]
line-bot-messaging-api = "0.1"
```

その後、以下のコマンドを実行して依存関係をインストールします。

```bash
cargo build
```

## クイックスタート

以下の例は、LINE Bot にテキストメッセージを送信するシンプルなコードです。

```rust
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::message::{LineMessageText, LineMessagesBuilder};
use tokio;

#[tokio::main]
async fn main() {
    let client = LineClient::new("YOUR_CHANNEL_ACCESS_TOKEN");

    let message = LineMessageText::new("テスト".to_string());
    let mut builder = LineMessagesBuilder::new();
    builder.append(message);

    match client
        .message_send_push(&builder.to_push_request("USER_ID"))
        .await  {
        Ok(_) => println!("Message sent successfully!"),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

```

## 主な機能

### メッセージ送信

- テキストメッセージ
- 画像メッセージ
- スタンプメッセージ
- その他のメッセージタイプ

### リッチメニュー

- リッチメニューの作成、取得、削除
- リッチメニューとユーザーの関連付け

### ユーザー情報

- ユーザープロファイルの取得

### Webhook

- Webhook イベントの処理

### インサイト

- メッセージ配信や応答の統計データ取得

## API リファレンス

各 API の詳細については、以下のドキュメントをご覧ください。

- [lib.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/lib.rs)
- [bot.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/api/bot.rs)
- [message.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/api/message.rs)
- [profile.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/api/profile.rs)
- [richmenu.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/api/richmenu.rs)
- [webhook.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/src/api/webhook.rs)

## テスト

プロジェクトにはいくつかのテストが含まれています。テストを実行するには、以下のコマンドを使用します。

```bash
cargo test
```

テストコードの例:

- [test\_chat.rs](https://github.com/uiuifree/rust-line-messaging-api/blob/main/tests/test_chat.rs)

## 貢献

このプロジェクトへの貢献を歓迎します。バグ報告や機能提案は [GitHub Issues](https://github.com/uiuifree/rust-line-messaging-api/issues) でお知らせください。


