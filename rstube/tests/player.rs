use rstest::rstest;
use rstube::client::RusTube;

#[rstest]
#[case("TbAsRmHYpKc")]
#[case("wiRRsHPTSC8")] // age restricted video
#[case("mKCieTImjvU")] // live stream
#[case("tbpfMbwDlpg")] // members only video
#[tokio::test]
async fn video_details_test(#[case] id: &str) -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client.fetch_video_info(id).await?;

    assert_eq!(res.id, id);
    assert!(!res.title.is_empty());
    assert!(res.duration.is_some());
    assert!(res.description.is_some());
    assert!(!res.thumbnail.is_empty());
    assert!(res.view_count.is_some());
    assert!(res.like_count.is_some());
    assert!(res.publish_date.is_some());
    Ok(())
}

#[rstest]
#[case("unavailable")] // unavailable video
#[tokio::test]
async fn video_details_error_test(#[case] id: &str) {
    let client = RusTube::new();
    let res = client.fetch_video_info(id).await;
    assert!(res.is_err());
}
