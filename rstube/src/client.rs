use crate::models::VideoInfo;
use crate::models::VideoPreview;
use crate::response::next::NextResponse;
use crate::response::player::PlayerResponse;
use crate::response::search::SearchResponse;

use common::error::Result;
use innertube::{client::InnerTube, models::ClientType};
use serde_json::Value;

pub struct RusTube;

impl RusTube {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_video_info<S: AsRef<str>>(&self, video_id: S) -> Result<VideoInfo> {
        let innertube = InnerTube::new(ClientType::WEB);
        let response = innertube
            .player::<PlayerResponse>(video_id.as_ref())
            .await?;
        let data = response.map_video_details()?;
        Ok(data)
    }

    pub async fn search<S: AsRef<str>>(&self, query: S) -> Result<Vec<VideoPreview>> {
        let innertube = InnerTube::new(ClientType::WEB);
        let response = innertube.search::<SearchResponse>(query.as_ref()).await?;
        let data = response.map_response();
        Ok(data)
    }

    pub async fn fetch_recommended_videos<S: AsRef<str>>(
        &self,
        video_id: S,
    ) -> Result<Vec<VideoPreview>> {
        let innertube = InnerTube::new(ClientType::WEB);
        let response = innertube.next::<Value>(video_id).await?;
        let data = NextResponse::new(response).map_recommended_videos()?;
        Ok(data)
    }
}

impl Default for RusTube {
    fn default() -> Self {
        Self::new()
    }
}
