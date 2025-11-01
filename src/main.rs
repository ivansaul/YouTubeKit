use anyhow::Result;
use youtube::client::YouTube;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

#[allow(dead_code)]
async fn call_next() -> Result<()> {
    let client = YouTube::new();
    let data = client.fetch_recommended_videos("TbAsRmHYpKc").await?;
    dbg!(data);
    Ok(())
}

#[allow(dead_code)]
async fn call_search() -> Result<()> {
    let client = YouTube::new();
    let data = client.search("YouTube videos").await?;
    dbg!(data);
    Ok(())
}
