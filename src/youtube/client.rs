use crate::error::Result;
use crate::youtube::models::VideoTag;
use crate::youtube::response::search::{SearchResponse, SectionListRendererItem};
use crate::{
    innertube::{client::Innertube, models::ClientType},
    youtube::response::player::{PlayerResponse, VideoDetails},
};

pub struct YouTube;

impl YouTube {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_video_details<S: AsRef<str>>(
        &self,
        video_id: S,
    ) -> Result<Option<VideoDetails>> {
        let innertube = Innertube::new(ClientType::IOS);
        let response = innertube
            .player::<PlayerResponse>(video_id.as_ref())
            .await?;
        Ok(response.video_details)
    }

    pub async fn search2<S: AsRef<str>>(&self, query: S) -> Result<serde_json::Value> {
        let innertube = Innertube::new(ClientType::WEB);
        let response = innertube
            .search::<serde_json::Value>(query.as_ref())
            .await?;
        Ok(response)
    }

    pub async fn search<S: AsRef<str>>(&self, query: S) -> Result<Vec<VideoTag>> {
        let innertube = Innertube::new(ClientType::WEB);
        let response = innertube.search::<SearchResponse>(query.as_ref()).await?;
        let data = response.map_response();
        Ok(data)
    }

    pub async fn token<S: AsRef<str>>(&self, query: S) -> Result<String> {
        let innertube = Innertube::new(ClientType::WEB);
        let response = innertube.search::<SearchResponse>(query.as_ref()).await?;
        let contents = response
            .contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents;

        for item in contents {
            match item {
                SectionListRendererItem::ContinuationItemRenderer(continuation) => {
                    return Ok(continuation
                        .continuation_endpoint
                        .continuation_command
                        .token)
                }
                _ => continue,
            }
        }

        Ok("token".to_string())
    }
}

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
