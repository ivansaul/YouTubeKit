use youtube::client::YouTube;

#[tokio::test]
async fn next_test() {
    let client = YouTube::new();
    let res = client.next("TbAsRmHYpKc").await;
    assert!(res.is_ok());
}
