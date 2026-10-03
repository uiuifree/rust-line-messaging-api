# Changelog

このプロジェクトの注目すべき変更はこのファイルに記録する。
形式は [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/) に、
バージョニングは [Semantic Versioning](https://semver.org/lang/ja/) に準拠する。

## [Unreleased]

## [0.2.0]

全面的に作り直した。0.1 とはソースレベルの互換性がない。

### Added

- LINE 公式 OpenAPI 仕様（line/line-openapi）の Messaging API・Insight・Audience・Channel access token の操作 102 件のうち 95 件（LINE 通知メッセージ（PNP）と membership の 7 件を除く全部）。
  メソッド名は `operationId` の snake_case
- 型付きのメッセージ（text / textV2 / sticker / image / video / audio / location / imagemap / template / flex / coupon）、
  全アクション、クイックリプライ、sender
- 全 Webhook イベントの型と署名検証（`webhook::verify_signature` / `webhook::parse_request`）
- 知らない `type` を `Unknown(serde_json::Value)` として受け取り、そのまま送り返せる仕組み
- リトライキー（`X-Line-Retry-Key`）と `x-line-request-id` / `x-line-accepted-request-id` の取り出し。
  リッチメニューの一括操作（`rich_menu_batch`）も、進捗確認に使う request ID を返す
- Webhook の `replyToken` は、仕様で必須のイベントでも省略可として受け取る（付いていないイベントがあってもリクエスト全体の解析を失敗させない）
- `LineClient::builder()`（`reqwest::Client`・接続先 URL の差し替え）。既定で接続 5 秒・無応答 10 秒のタイムアウト
- トークン発行系のエラー（`error` / `error_description`）を `ErrorResponse` で型付きで扱える
- `Error::Decode` に `x-line-request-id` を保持。エラー応答の本文を読めなかったときもステータスと request ID を残す
- 最低 Rust バージョン 1.88 を宣言し、CI で確認
- 認証情報をログに出さない: `Error::Http` の表示から URL を除く（トークン検証などはクエリに認証情報を載せるため）。チャネルアクセストークン・トークン発行のリクエストとレスポンスの `Debug` で秘密値を伏せる
- `reqwest` と `serde_json` の再公開
- 日本語の `README.md` と英語の `README.en.md`、`llms.txt`、`LICENSE`、CI
- 全公開 API の rustdoc（説明は LINE 公式 OpenAPI 仕様から。`#![warn(missing_docs)]` で抜けを CI で検出）
- サンプル `examples/echo_bot.rs`（axum で Webhook を受けるオウム返し Bot）と `examples/push_message.rs`

### Changed

- エラーを `LineError` から `Error`（`std::error::Error` 実装）に変更
- reqwest 0.13 に更新（TLS は既定の rustls）。`serde_derive` を外し `serde` の `derive` feature に統合

### Fixed

- GET のパラメータを JSON ボディで送っていたため、`followers/ids` の `limit` / `start` が LINE に届いていなかった
- URL 末尾の空白で `quota/consumption` と `aggregation/list` が失敗していた
- パスに埋め込む ID をエスケープしていなかった
- 空のボディを返すエンドポイントが失敗扱いになっていた
- 画像のダウンロードで失敗時に panic していた。アップロードの Content-Type が `image/jpeg` 固定だった

### Removed

- `blocking` feature（使われていなかった）
