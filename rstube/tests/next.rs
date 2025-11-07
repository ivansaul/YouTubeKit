use rstube::client::RusTube;

#[tokio::test]
async fn next_test() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client.fetch_recommended_videos("TbAsRmHYpKc").await?;
    assert!(!res.is_empty());
    assert!(res.first().map(|v| v.id.clone()).is_some());
    Ok(())
}
