use crate::{
    locale::Language,
    models::{VideoItem, YouTubeItem},
    response::Thumbnails,
    serializer::{text::Text, MapResult},
};
use serde::Deserialize;
use serde_with::{rust::deserialize_ignore_any, serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum YouTubeListItem {
    #[serde(alias = "gridVideoRenderer", alias = "compactVideoRenderer")]
    VideoRenderer(VideoRenderer),

    /// Continuation items are located at the end of a list
    /// and contain the continuation token for progressive loading.
    ContinuationItemRenderer(ContinuationItemRenderer),

    /// Contains search results (e.g. "Upcoming live" or "Community posts")
    ItemSectionRenderer {
        contents: MapResult<Vec<YouTubeListItem>>,
    },

    /// No video list item (e.g. ad) or unimplemented item
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    None,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnails,
    #[serde_as(as = "Text")]
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationItemRenderer {
    pub continuation_endpoint: ContinuationEndpoint,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationEndpoint {
    pub continuation_command: ContinuationCommand,
}

impl ContinuationEndpoint {
    /// Extracts the raw continuation token string.
    pub(crate) fn into_token(self) -> String {
        self.continuation_command.token
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationCommand {
    pub token: String,
}

/// Maps a mixed list of YouTube entities (videos, channels, playlists)
/// into typed model items, collecting warnings for unrecognized items.
#[derive(Debug)]
pub(crate) struct YouTubeListMapper<T> {
    /// Language context (reserved for future localized date parsing)
    _lang: Language,
    pub items: Vec<T>,
    pub warnings: Vec<String>,
    pub ctoken: Option<String>,
    /// Query corrected by YouTube (populated when available in response)
    pub _corrected_query: Option<String>,
}

impl<T> YouTubeListMapper<T> {
    pub fn new(lang: Language) -> Self {
        Self {
            _lang: lang,
            items: Vec::new(),
            warnings: Vec::new(),
            ctoken: None,
            _corrected_query: None,
        }
    }

    fn map_video(&mut self, video: VideoRenderer) -> VideoItem {
        VideoItem {
            id: video.video_id,
            name: video.title,
            thumbnails: video.thumbnail.thumbnails,
        }
    }
}

impl YouTubeListMapper<YouTubeItem> {
    fn map_item(&mut self, item: YouTubeListItem) {
        match item {
            YouTubeListItem::VideoRenderer(video) => {
                let mapped = YouTubeItem::Video(self.map_video(video));
                self.items.push(mapped);
            }
            YouTubeListItem::ItemSectionRenderer { mut contents, .. } => {
                self.warnings.append(&mut contents.warnings);
                contents
                    .content
                    .into_iter()
                    .for_each(|it| self.map_item(it));
            }
            YouTubeListItem::ContinuationItemRenderer(r) => {
                self.ctoken = Some(r.continuation_endpoint.into_token());
            }
            YouTubeListItem::None => {}
        }
    }

    pub(crate) fn map_response(&mut self, mut res: MapResult<Vec<YouTubeListItem>>) {
        self.warnings.append(&mut res.warnings);
        res.content.into_iter().for_each(|item| self.map_item(item));
    }
}
