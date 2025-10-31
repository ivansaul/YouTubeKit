use youtube::client::YouTube;

#[tokio::test]
async fn next_test() {
    let client = YouTube::new();
    let res = client.fetch_recommended_videos("TbAsRmHYpKc").await;
    assert!(res.is_ok());
}
