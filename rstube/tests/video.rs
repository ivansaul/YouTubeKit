use rstest::rstest;
use rstube::{
    client::RusTube,
    locale::Language,
};

/// Tests that standard video details are fetched correctly.
#[rstest]
#[case("TbAsRmHYpKc")]
#[tokio::test]
async fn video_details_test(#[case] id: &str) -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client.query().video(id).send().await?;

    assert_eq!(res.id, id);
    assert!(!res.title.is_empty());
    assert!(res.duration.is_some());
    assert!(res.description.is_some());
    assert!(!res.thumbnail.is_empty());
    assert!(res.view_count.is_some());
    assert!(res.publish_date.is_some());
    Ok(())
}

/// Tests that requesting an unavailable video returns an extraction error.
#[rstest]
#[case("unavailable")]
#[tokio::test]
async fn video_details_unavailable_test(#[case] id: &str) {
    let client = RusTube::new();
    let res = client.query().lang(Language::EnglishUS).video(id).send().await;
    assert!(res.is_err());
}
