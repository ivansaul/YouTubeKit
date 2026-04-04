use time::OffsetDateTime;

use crate::{
    locale::Language,
    models::{channel::ChannelTag, VideoItem, YouTubeItem},
    response::youtube_item::{IsLive, IsShort, TimeOverlayStyle, VideoRenderer, YouTubeListItem},
    serializer::MapResult,
    utils,
};

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
    pub corrected_query: Option<String>,
}

impl<T> YouTubeListMapper<T> {
    pub fn new(lang: Language) -> Self {
        Self {
            _lang: lang,
            items: Vec::new(),
            warnings: Vec::new(),
            ctoken: None,
            corrected_query: None,
        }
    }

    fn map_video(&mut self, video: VideoRenderer) -> VideoItem {
        let is_live = video.thumbnail_overlays.is_live() || video.badges.is_live();
        let is_short = video.thumbnail_overlays.is_short();
        let is_upcoming = video.upcoming_event_data.is_some();

        let length_text = video.length_text.or_else(|| {
            video
                .thumbnail_overlays
                .into_iter()
                .find(|ol| {
                    ol.thumbnail_overlay_time_status_renderer.style == TimeOverlayStyle::Default
                })
                .map(|ol| ol.thumbnail_overlay_time_status_renderer.text)
        });

        let short_description = video
            .detailed_metadata_snippets
            .and_then(|snippets| snippets.into_iter().next().map(|d| d.snippet_text))
            .or(video.description_snippet);

        let view_count = video
            .view_count_text
            .and_then(|txt| utils::numeric::parse_numeric(&txt).ok());

        let duration = length_text.and_then(|text| utils::numeric::parse_video_length(&text));

        let publish_date = video
            .upcoming_event_data
            .as_ref()
            .and_then(|upc| OffsetDateTime::from_unix_timestamp(upc.start_time).ok())
            // or_else(|| {
            // TODO: parse from video.published_time_text
            // })
        ;

        let channel_avatar = video
            .channel_thumbnail_supported_renderers
            .map(|c| c.channel_thumbnail_with_link_renderer.thumbnail.thumbnails)
            .unwrap_or_default();

        let channel_badge = video.owner_badges.into();

        let channel = video
            .channel
            .and_then(|s| s.runs.into_iter().next())
            .map(|r| ChannelTag {
                id: r.navigation_endpoint.browse_endpoint.browse_id,
                name: r.text,
                handle: r
                    .navigation_endpoint
                    .browse_endpoint
                    .canonical_base_url
                    .map(|h| h.replace("/@", "")),
                avatar: channel_avatar,
                verification: channel_badge,
                subscriber_count: None,
            });

        VideoItem {
            id: video.video_id,
            name: video.title,
            thumbnails: video.thumbnail.thumbnails,
            duration,
            channel,
            view_count,
            is_live,
            is_short,
            is_upcoming,
            publish_date,
            publish_date_txt: video.published_time_text,
            short_description,
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
            YouTubeListItem::DidYouMeanRenderer { corrected_query } => {
                self.corrected_query = Some(corrected_query);
            }
            YouTubeListItem::None => {}
        }
    }

    pub(crate) fn map_response(&mut self, mut res: MapResult<Vec<YouTubeListItem>>) {
        self.warnings.append(&mut res.warnings);
        res.content.into_iter().for_each(|item| self.map_item(item));
    }
}
