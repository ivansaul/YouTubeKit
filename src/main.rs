use anyhow::Result;
use youtube::client::YouTube;

#[tokio::main]
async fn main() -> Result<()> {
    // let yt = YouTube::new();
    // // let yt = YouTubeBridge::new();

    // let data = yt.get_video_details("e0RvAwqU2uw").await?;
    // // let dt = yt.search("archlinux hyperland").await?;
    // // let data = yt.fetch_video_details("e0RvAwqU2uw".into()).await?;
    // dbg!(data);
    //
    call_next().await?;

    Ok(())
}

async fn call_next() -> Result<()> {
    let client = YouTube::new();
    let data = client.fetch_recommended_videos("TbAsRmHYpKc").await?;
    dbg!(data);
    Ok(())
}
