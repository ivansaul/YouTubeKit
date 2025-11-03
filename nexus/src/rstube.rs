use common::error::Result;
use rstube::client::RusTube as NativeRusTube;

#[derive(uniffi::Object)]
pub struct RusTube {
    client: NativeRusTube,
}

#[uniffi::export(async_runtime = "tokio")]
impl RusTube {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            client: NativeRusTube::new(),
        }
    }

    pub async fn search(&self, query: String) -> Result<String> {
        let res = self.client.search(query).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }

    pub async fn fetch_video_info(&self, video_id: String) -> Result<String> {
        let res = self.client.fetch_video_info(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }

    pub async fn fetch_recommended_videos(&self, video_id: String) -> Result<String> {
        let res = self.client.fetch_recommended_videos(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }
}

impl Default for RusTube {
    fn default() -> Self {
        Self::new()
    }
}
