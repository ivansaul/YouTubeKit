use common::error::Result;
use youtube::client::YouTube;

#[derive(uniffi::Object)]
pub struct YouTubeBridge {
    inner: YouTube,
}

#[uniffi::export(async_runtime = "tokio")]
impl YouTubeBridge {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: YouTube::new(),
        }
    }

    pub async fn search(&self, query: String) -> Result<String> {
        let res = self.inner.search(query).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }

    pub async fn fetch_video_details(&self, video_id: String) -> Result<String> {
        let res = self.inner.fetch_video_details(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }

    pub async fn fetch_recommended_videos(&self, video_id: String) -> Result<String> {
        let res = self.inner.fetch_recommended_videos(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }
}

impl Default for YouTubeBridge {
    fn default() -> Self {
        Self::new()
    }
}
