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

    pub async fn titles(&self, query: String) -> Result<Vec<String>> {
        let data = self
            .inner
            .search(query)
            .await?
            .iter()
            .map(|video| video.name.clone())
            .collect();
        Ok(data)
    }

    pub async fn fetch_video_details(&self, video_id: String) -> Result<String> {
        let res = self.inner.get_video_details(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }

    pub async fn next(&self, video_id: String) -> Result<String> {
        let res = self.inner.next(video_id).await?;
        let json = serde_json::to_string(&res)?;
        Ok(json)
    }
}
