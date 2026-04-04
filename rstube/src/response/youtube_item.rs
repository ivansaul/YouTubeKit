use crate::response::{ChannelBadge, NavigationEndpoint};
use crate::serializer::MapResult;
use crate::{response::Thumbnails, serializer::text::Text};

use serde::Deserialize;
use serde_with::{
    rust::deserialize_ignore_any, serde_as, DefaultOnError, DisplayFromStr, VecSkipError,
};

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

    /// Corrected search query
    #[serde(rename_all = "camelCase")]
    #[serde(alias = "showingResultsForRenderer")]
    DidYouMeanRenderer {
        #[serde_as(as = "Text")]
        corrected_query: String,
    },

    /// No video list item (e.g. ad) or unimplemented item
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    None,
}

//
// VideoRenderer
//

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnails,
    #[serde_as(as = "Text")]
    pub title: String,
    #[serde_as(as = "Option<Text>")]
    pub length_text: Option<String>,
    /// Contains live tag for recommended videos
    #[serde(default)]
    #[serde_as(as = "VecSkipError<_>")]
    pub badges: Vec<VideoBadge>,
    /// Contains Short/Live tag
    #[serde(default)]
    #[serde_as(as = "VecSkipError<_>")]
    pub thumbnail_overlays: Vec<TimeOverlay>,
    #[serde_as(as = "Option<Text>")]
    pub view_count_text: Option<String>,
    #[serde_as(as = "Option<Text>")]
    pub published_time_text: Option<String>,
    /// Abbreviated video description (on startpage)
    #[serde_as(as = "Option<Text>")]
    pub description_snippet: Option<String>,
    /// Contains abbreviated video description (on search page)
    #[serde_as(as = "Option<VecSkipError<_>>")]
    pub detailed_metadata_snippets: Option<Vec<DetailedMetadataSnippet>>,
    /// Release date for upcoming videos
    pub upcoming_event_data: Option<UpcomingEventData>,
    #[serde(rename = "shortBylineText")]
    pub channel: Option<ShortBylineText>,
    pub channel_thumbnail_supported_renderers: Option<ChannelThumbnailSupportedRenderers>,
    #[serde(default)]
    #[serde_as(as = "VecSkipError<_>")]
    pub owner_badges: Vec<ChannelBadge>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TimeOverlay {
    pub thumbnail_overlay_time_status_renderer: TimeOverlayRenderer,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TimeOverlayRenderer {
    /// `29:54`
    ///
    /// Is `LIVE` in case of a livestream and `SHORTS` in case of a short video
    #[serde_as(as = "Text")]
    pub text: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub style: TimeOverlayStyle,
}

#[derive(Default, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TimeOverlayStyle {
    #[default]
    Default,
    Live,
    Shorts,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DetailedMetadataSnippet {
    #[serde_as(as = "Text")]
    pub snippet_text: String,
}

/// Badges are displayed on the video thumbnail and
/// show certain video properties (e.g. active livestream)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoBadge {
    pub metadata_badge_renderer: VideoBadgeRenderer,
}

/// Badges are displayed on the video thumbnail and
/// show certain video properties (e.g. active livestream)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoBadgeRenderer {
    pub style: VideoBadgeStyle,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum VideoBadgeStyle {
    /// Active livestream
    BadgeStyleTypeLiveNow,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpcomingEventData {
    /// Unixtime in seconds
    #[serde_as(as = "DisplayFromStr")]
    pub start_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChannelThumbnailSupportedRenderers {
    pub channel_thumbnail_with_link_renderer: ChannelThumbnailWithLinkRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChannelThumbnailWithLinkRenderer {
    pub thumbnail: Thumbnails,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShortBylineText {
    #[serde(default)]
    #[serde_as(as = "VecSkipError<_>")]
    pub runs: Vec<ShortBylineTextRun>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShortBylineTextRun {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint,
}

pub(crate) trait IsLive {
    fn is_live(&self) -> bool;
}

pub(crate) trait IsShort {
    fn is_short(&self) -> bool;
}

impl IsLive for Vec<TimeOverlay> {
    fn is_live(&self) -> bool {
        self.iter().any(|overlay| {
            overlay.thumbnail_overlay_time_status_renderer.style == TimeOverlayStyle::Live
        })
    }
}

impl IsShort for Vec<TimeOverlay> {
    fn is_short(&self) -> bool {
        self.iter().any(|overlay| {
            overlay.thumbnail_overlay_time_status_renderer.style == TimeOverlayStyle::Shorts
        })
    }
}

impl IsLive for Vec<VideoBadge> {
    fn is_live(&self) -> bool {
        self.iter().any(|badge| {
            badge.metadata_badge_renderer.style == VideoBadgeStyle::BadgeStyleTypeLiveNow
        })
    }
}

//
// ContinuationItemRenderer
//

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
