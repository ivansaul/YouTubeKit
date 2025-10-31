use crate::models::video_details::VideoDetails;
use crate::models::VideoTag;
use crate::response::next::NextResponse;
use crate::response::player::PlayerResponse;
use crate::response::search::{SearchResponse, SectionListRendererItem};

use common::error::Result;
use innertube::{client::Innertube, models::ClientType};
use serde_json::Value;

pub struct YouTube;

impl YouTube {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_video_details<S: AsRef<str>>(&self, video_id: S) -> Result<VideoDetails> {
        let innertube = Innertube::new(ClientType::WEB);
        let response = innertube
            .player::<PlayerResponse>(video_id.as_ref())
            .await?;

        Ok(response.map_video_details()?)
    }

    pub async fn search<S: AsRef<str>>(&self, query: S) -> Result<Vec<VideoTag>> {
        let innertube = Innertube::new(ClientType::WEB);
        let response = innertube.search::<SearchResponse>(query.as_ref()).await?;
        let data = response.map_response();
        Ok(data)
    }

    pub async fn fetch_recommended_videos<S: AsRef<str>>(
        &self,
        video_id: S,
    ) -> Result<Vec<VideoTag>> {
        let innertube = Innertube::new(ClientType::WEB);
        let value = innertube.next::<Value>(video_id).await?;
        NextResponse::new(value).map_recommended_videos()
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

impl Default for YouTube {
    fn default() -> Self {
        Self::new()
    }
}
