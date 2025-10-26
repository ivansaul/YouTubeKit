use anyhow::Result;
use ytkit::youtube::client::YouTube;

#[tokio::main]
async fn main() -> Result<()> {
    let yt = YouTube::new();

    // let dt = yt.get_video_details("e4s37VcWCj0").await?;
    let dt = yt.search("archlinux hyperland").await?;

    dbg!(dt);

    Ok(())
}

// async fn main2() -> Result<()> {
//     let client = Innertube::new(ClientType::IOS);
//     let data: serde_json::Value = client.player("SzhWy7cUpsM").await?;
//     // dbg!(data);
//     let streams = &data["streamingData"]["adaptiveFormats"];
//     dbg!(streams);
//     println!("ok");
//     Ok(())
// }
