use rstube::{
    client::RusTube,
    locale::Language,
    models::VideoItem,
};

/// Tests that a basic search returns non-empty results.
#[tokio::test]
async fn search_returns_results() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .lang(Language::EnglishUS)
        .search("Rust programming language")
        .send::<VideoItem>()
        .await?;

    assert!(!res.paginator.items().is_empty());
    Ok(())
}

/// Tests that pagination works correctly across multiple pages.
#[tokio::test]
async fn search_pagination() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .lang(Language::Japanese)
        .search("Rust programming language")
        .send::<VideoItem>()
        .await?;

    let mut paginator = res.paginator;
    assert!(!paginator.items().is_empty());

    let mut pages_fetched = 1;
    while let Some(items) = paginator.next_page().await? {
        assert!(!items.is_empty());
        pages_fetched += 1;
        if pages_fetched >= 3 {
            break;
        }
    }

    assert!(pages_fetched >= 2, "expected at least 2 pages");
    Ok(())
}
