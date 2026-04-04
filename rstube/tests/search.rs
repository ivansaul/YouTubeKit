use rstube::{client::RusTube, models::VideoItem};

#[tokio::test]
async fn search_videos() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .search("archlinux hyperland")
        .send::<VideoItem>()
        .await?;

    assert!(res.estimated_results.is_some());
    assert!(res.corrected_query.is_some());
    assert!(!res.paginator.items().is_empty());

    Ok(())
}

#[tokio::test]
async fn search_pagination() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .search("rust programming language")
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

#[tokio::test]
async fn search_live_streams() -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client
        .query()
        .search("live stream")
        .send::<VideoItem>()
        .await?;

    assert!(res.estimated_results.is_some());

    let items = res.paginator.items();
    assert!(!items.is_empty());
    assert!(items.iter().find(|v| v.is_live).is_some());

    Ok(())
}
