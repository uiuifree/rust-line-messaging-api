use line_bot_messaging_api::LineClient;

#[tokio::test]
async fn test() {
    let token = env!("TOKEN");
    let client = LineClient::new(token);

    let res = client.bot_info().await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
