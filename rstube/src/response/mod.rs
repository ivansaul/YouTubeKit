pub mod continuation;
pub mod mapper;
pub mod next;
pub mod player;
pub mod search;
pub mod youtube_item;

use crate::models::{self, Thumbnail};
use serde::Deserialize;

/// Wrapper for the YouTube API's thumbnail list format.
#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

//
// NavigationEndpoint
//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
    pub canonical_base_url: Option<String>,
}

//
// Channel Verification Badge
//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChannelBadge {
    pub metadata_badge_renderer: ChannelBadgeRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChannelBadgeRenderer {
    pub style: ChannelBadgeStyle,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ChannelBadgeStyle {
    BadgeStyleTypeVerified,
    BadgeStyleTypeVerifiedArtist,
}

impl From<Vec<ChannelBadge>> for models::channel::Verification {
    fn from(badges: Vec<ChannelBadge>) -> Self {
        badges
            .first()
            .map_or(models::channel::Verification::None, |b| {
                match b.metadata_badge_renderer.style {
                    ChannelBadgeStyle::BadgeStyleTypeVerified => Self::Verified,
                    ChannelBadgeStyle::BadgeStyleTypeVerifiedArtist => Self::Artist,
                }
            })
    }
}

//
// Empty
//

#[derive(Debug, Deserialize)]
pub(crate) struct Empty {}
