use common::error::Result;
use youtube::{client::YouTube, models::VideoTag};

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

    pub async fn token(&self, query: String) -> Result<String> {
        let value = self.inner.token(query).await?;
        Ok(value)
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

    pub async fn videos(&self, query: String) -> Result<Vec<VideoTag>> {
        let data = self.inner.search(query).await?;
        Ok(data)
    }
}
