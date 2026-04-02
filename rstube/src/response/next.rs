use jsonpath_rust::JsonPath;
use serde::Deserialize;
use serde_with::{rust::deserialize_ignore_any, serde_as, DefaultOnError, VecSkipError};

use crate::{
    error::ExtractionError,
    models::{self, channel::ChannelTag},
    response::{ChannelBadge, NavigationEndpoint, Thumbnails},
    serializer::{text::Text, MapRespCtx, MapResponse, MapResult},
    utils,
};

/// Video details response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoDetails {
    /// Video metadata + recommended videos
    pub contents: Option<Contents>,
    /// Video ID
    pub current_video_endpoint: Option<CurrentVideoEndpoint>,
}

/// Video details main object, contains video metadata and recommended videos
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Contents {
    pub two_column_watch_next_results: TwoColumnWatchNextResults,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TwoColumnWatchNextResults {
    /// Metadata about the video
    pub results: VideoResultsWrap,
}

/// Metadata about the video
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoResultsWrap {
    pub results: VideoResults,
}

/// Video metadata items
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoResults {
    pub contents: Option<MapResult<Vec<VideoResultsItem>>>,
}

/// Video metadata item
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum VideoResultsItem {
    #[serde(rename_all = "camelCase")]
    VideoPrimaryInfoRenderer {
        #[serde_as(as = "Text")]
        title: String,
        view_count: Option<ViewCount>,
        /// Like/Dislike button
        video_actions: VideoActions,
        /// Absolute textual date (e.g. `Dec 29, 2019`)
        #[serde_as(as = "Option<Text>")]
        date_text: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    VideoSecondaryInfoRenderer {
        owner: VideoOwner,
        #[serde_as(as = "Option<Text>")]
        description: Option<String>,
        #[serde_as(as = "Option<Text>")]
        attributed_description: Option<String>,
    },
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewCount {
    pub video_view_count_renderer: ViewCountRenderer,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewCountRenderer {
    /// View count (`232,975,196 views`)
    #[serde_as(deserialize_as = "DefaultOnError<Text>")]
    pub view_count: String,
    #[serde(default)]
    pub is_live: bool,
}

/// Like/Dislike buttons
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoActions {
    pub menu_renderer: serde_json::Value,
}

impl VideoActions {
    fn get_like_count(&self) -> Option<u32> {
        self.menu_renderer
            .query("$..segmentedLikeDislikeButtonViewModel..accessibilityText")
            .ok()?
            .iter()
            .filter_map(|v| v.as_str())
            .find_map(|s| utils::numeric::parse_numeric::<u32>(s).ok())
    }
}

/// Video channel information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoOwner {
    pub video_owner_renderer: VideoOwnerRenderer,
}

/// Video channel information
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoOwnerRenderer {
    #[serde_as(as = "Text")]
    pub title: String,
    pub thumbnail: Thumbnails,
    pub navigation_endpoint: NavigationEndpoint,
    #[serde_as(as = "Option<Text>")]
    pub subscriber_count_text: Option<String>,
    #[serde(default)]
    #[serde_as(as = "VecSkipError<_>")]
    pub badges: Vec<ChannelBadge>,
}

//
// currentVideoEndpoint
//

/// Contains current video ID
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrentVideoEndpoint {
    pub watch_endpoint: CurrentVideoWatchEndpoint,
}
/// Contains current video ID
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrentVideoWatchEndpoint {
    pub video_id: String,
}

//
// MapResponse
//

impl MapResponse<models::VideoDetails> for VideoDetails {
    fn map_response(
        self,
        ctx: &MapRespCtx<'_>,
    ) -> Result<MapResult<models::VideoDetails>, ExtractionError> {
        let mut warnings = Vec::new();

        let contents = self.contents.ok_or_else(|| ExtractionError::NotFound {
            id: ctx.id.to_owned(),
            msg: "no content".into(),
        })?;
        let current_video_endpoint =
            self.current_video_endpoint
                .ok_or_else(|| ExtractionError::NotFound {
                    id: ctx.id.to_owned(),
                    msg: "no current_video_endpoint".into(),
                })?;

        let video_id = current_video_endpoint.watch_endpoint.video_id;
        if ctx.id != video_id {
            return Err(ExtractionError::WrongResult(format!(
                "got wrong video id {}, expected {}",
                video_id, ctx.id
            )));
        }

        let mut primary_results = contents
            .two_column_watch_next_results
            .results
            .results
            .contents
            .ok_or_else(|| ExtractionError::NotFound {
                id: ctx.id.into(),
                msg: "no primary_results".into(),
            })?;

        warnings.append(&mut primary_results.warnings);

        let mut primary_info = None;
        let mut secondary_info = None;

        primary_results.content.into_iter().for_each(|r| match r {
            VideoResultsItem::VideoPrimaryInfoRenderer { .. } => {
                primary_info = Some(r);
            }
            VideoResultsItem::VideoSecondaryInfoRenderer { .. } => {
                secondary_info = Some(r);
            }
            VideoResultsItem::None => {}
        });

        let (title, is_live, view_count, publish_date_text, like_count) = match primary_info {
            Some(VideoResultsItem::VideoPrimaryInfoRenderer {
                title,
                view_count,
                video_actions,
                date_text,
            }) => (
                title,
                view_count
                    .as_ref()
                    .map(|v| v.video_view_count_renderer.is_live)
                    .unwrap_or_default(),
                view_count.as_ref().and_then(|vc| {
                    utils::numeric::parse_numeric(&vc.video_view_count_renderer.view_count).ok()
                }),
                date_text,
                video_actions.get_like_count(),
            ),
            _ => {
                return Err(ExtractionError::InvalidData(
                    "could not find primary_info".into(),
                ));
            }
        };

        let (owner, description) = match secondary_info {
            Some(VideoResultsItem::VideoSecondaryInfoRenderer {
                owner,
                description,
                attributed_description,
            }) => (
                owner.video_owner_renderer,
                attributed_description.or(description),
            ),
            _ => {
                return Err(ExtractionError::InvalidData(
                    "could not find secondary_info".into(),
                ))
            }
        };

        let channel_id = owner.navigation_endpoint.browse_endpoint.browse_id;
        let channel_alias = owner
            .navigation_endpoint
            .browse_endpoint
            .canonical_base_url
            .map(|s| s.replace("/@", ""));
        let channel_name = owner.title;
        let channel_avatar = owner.thumbnail.thumbnails;
        let channel_subscribers = owner.subscriber_count_text;
        let channel_badge = owner.badges;

        Ok(MapResult {
            content: models::VideoDetails {
                id: video_id,
                title,
                duration: None,
                keywords: Vec::new(),
                description,
                thumbnail: Vec::new(),
                view_count,
                is_live,
                like_count,
                category: None,
                is_short: false,
                publish_date: None,
                publish_date_text,
                channel: ChannelTag {
                    id: channel_id,
                    handle: channel_alias,
                    name: channel_name,
                    avatar: channel_avatar,
                    subscriber_count: channel_subscribers,
                    verification: channel_badge.into(),
                },
            },
            warnings,
        })
    }
}
