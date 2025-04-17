use std::env;
use line_bot_messaging_api::LineClient;

#[tokio::test]
async fn test() {
    let token = env::var("TOKEN").unwrap();
    let client = LineClient::new(&token);

    let res = client.bot_info().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
