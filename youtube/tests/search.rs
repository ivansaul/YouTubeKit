use rstest::rstest;
use youtube::client::YouTube;

#[rstest]
#[case("rust programming language")]
#[case("archlinux hyperland")]
#[case("YouTube videos")]
#[tokio::test]
async fn search_test(#[case] query: &str) -> anyhow::Result<()> {
    let client = YouTube::new();
    let res = client.search(query).await?;
    assert!(!res.is_empty());
    Ok(())
}
