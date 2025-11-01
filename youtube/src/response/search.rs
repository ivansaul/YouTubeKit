use crate::{
    models::{ChannelTag, VideoTag},
    response::player::Thumbnails,
};
use serde::Deserialize;
use serde_with::rust::deserialize_ignore_any;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub contents: Contents,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_search_results_renderer: TwoColumnSearchResultsRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnSearchResultsRenderer {
    pub primary_contents: PrimaryContents,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryContents {
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRenderer {
    pub contents: Vec<SectionListRendererItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionListRendererItem {
    #[serde(rename_all = "camelCase")]
    ItemSectionRenderer(ItemSectionRenderer),

    #[serde(rename_all = "camelCase")]
    ContinuationItemRenderer(ContinuationItemRenderer),

    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer {
    pub contents: Vec<ItemSectionRendererItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemSectionRendererItem {
    #[serde(rename_all = "camelCase")]
    VideoRenderer(VideoRenderer),

    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnails,
    pub title: Title,
    pub published_time_text: Option<SimpleText>,
    pub length_text: Option<SimpleText>,
    pub short_view_count_text: Option<ShortViewCountText>,
    pub owner_text: Option<OwnerText>,
    pub channel_thumbnail_supported_renderers: ChannelThumbnailSupportedRenderers,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub runs: Vec<SimpleText>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    #[serde(alias = "simpleText")]
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortViewCountText {
    pub runs: Option<Vec<SimpleText>>,
    pub simple_text: Option<String>,
}

impl ShortViewCountText {
    pub fn get_text(&self) -> Option<String> {
        if let Some(s) = self.simple_text.as_ref() {
            return Some(s.clone());
        }
        if let Some(runs) = self.runs.as_ref() {
            return Some(
                runs.iter()
                    .map(|r| r.text.clone())
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }
        None
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerText {
    runs: Vec<OwnerTextRun>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerTextRun {
    pub text: String,
    // TO-DO: check if this is always present
    // Sometimes don't have this field, for example
    // https://www.youtube.com/watch?v=nOSxuaDgl3s
    // the owners are a multiple channels "Collaborators"
    // pub navigation_endpoint: NavigationEndpoint,
    // now collect data from this field ChannelThumbnailSupportedRenderers
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnailSupportedRenderers {
    pub channel_thumbnail_with_link_renderer: ChannelThumbnailWithLinkRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnailWithLinkRenderer {
    pub thumbnail: Thumbnails,
    pub navigation_endpoint: NavigationEndpoint,
}

// =========================
// ContinuationItemRenderer
// =========================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemRenderer {
    pub continuation_endpoint: ContinuationEndpoint,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationEndpoint {
    pub continuation_command: ContinuationCommand,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationCommand {
    pub token: String,
}

// =======================
// Map Response
// =======================

fn map_video_render(item: &VideoRenderer) -> VideoTag {
    let title = item
        .title
        .runs
        .iter()
        .map(|i| i.text.clone())
        .collect::<Vec<_>>()
        .join("");

    let avatar = item
        .channel_thumbnail_supported_renderers
        .channel_thumbnail_with_link_renderer
        .thumbnail
        .thumbnails
        .clone();

    let channel_id = item
        .channel_thumbnail_supported_renderers
        .channel_thumbnail_with_link_renderer
        .navigation_endpoint
        .browse_endpoint
        .browse_id
        .clone();

    let channel_label = item
        .channel_thumbnail_supported_renderers
        .channel_thumbnail_with_link_renderer
        .navigation_endpoint
        .browse_endpoint
        .canonical_base_url
        .clone();

    let channel = item
        .owner_text
        .as_ref()
        .and_then(|owner_text| owner_text.runs.first())
        .map(|owner| ChannelTag {
            id: channel_id,
            name: owner.text.clone(),
            label: channel_label.unwrap_or_default(),
            avatar,
            subscriber_count: None,
        });

    VideoTag {
        id: item.video_id.clone(),
        name: title,
        thumbnail: item.thumbnail.thumbnails.clone(),
        publish_date: item.published_time_text.as_ref().map(|i| i.text.clone()),
        length_text: item.length_text.as_ref().map(|i| i.text.clone()),
        view_count: item
            .short_view_count_text
            .as_ref()
            .and_then(|i| i.get_text()),
        channel,
    }
}

impl SearchResponse {
    pub fn map_response(&self) -> Vec<VideoTag> {
        self.contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents
            .iter()
            .filter_map(|item| match item {
                SectionListRendererItem::ItemSectionRenderer(item) => Some(item),
                _ => None,
            })
            .flat_map(|item| item.contents.iter())
            .filter_map(|item| match item {
                ItemSectionRendererItem::VideoRenderer(item) => Some(item),
                _ => None,
            })
            .map(map_video_render)
            .collect::<Vec<_>>()
    }
}
