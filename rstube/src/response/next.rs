use crate::models::{ChannelPreview, Thumbnail, VideoPreview};
use common::error::{ExtractionError, Result};
use jsonpath_rust::JsonPath;
use serde_json::Value;

pub struct NextResponse {
    value: Value,
}

impl NextResponse {
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    pub fn map_recommended_videos(&self) -> Result<Vec<VideoPreview>> {
        let lockup_vm_path = concat!(
            "$.contents",
            ".twoColumnWatchNextResults",
            ".secondaryResults",
            ".secondaryResults",
            // filter only video
            ".results[?(@.lockupViewModel.contentType == 'LOCKUP_CONTENT_TYPE_VIDEO')]",
            ".lockupViewModel",
        );

        let video_id_path = "$.contentId";

        let video_title_path = concat!(
            "$.metadata",
            ".lockupMetadataViewModel",
            ".title",
            ".content",
        );

        let video_thumbnail_path = concat!(
            "$.contentImage",
            ".thumbnailViewModel",
            ".image",
            ".sources",
        );

        let publish_date_path = concat!(
            "$.metadata",
            ".lockupMetadataViewModel",
            ".metadata",
            ".contentMetadataViewModel",
            ".metadataRows[1]",
            ".metadataParts[1]",
            ".text.content",
        );

        let view_count_path = concat!(
            "$.metadata",
            ".lockupMetadataViewModel",
            ".metadata",
            ".contentMetadataViewModel",
            ".metadataRows[1]",
            ".metadataParts[0]",
            ".text.content",
        );

        let lenght_text_path = concat!(
            "$.contentImage",
            ".thumbnailViewModel",
            ".overlays[0]",
            ".thumbnailOverlayBadgeViewModel",
            ".thumbnailBadges[0]",
            ".thumbnailBadgeViewModel.text",
        );

        let lockup_vm_list = self.value.query(lockup_vm_path).map_err(|_| {
            ExtractionError::InvalidData("[NextResponse] Could not collect recomended videos")
        })?;

        let mut recomended_videos: Vec<VideoPreview> = Vec::new();

        for lockup_vm in lockup_vm_list.iter() {
            let video_id = lockup_vm
                .query(video_id_path)
                .first_one()
                .and_then(|v| v.as_str())
                .ok_or(ExtractionError::InvalidData(
                    "[NextResponse] Could not find video id ",
                ))?;

            let video_title = lockup_vm
                .query(video_title_path)
                .first_one()
                .and_then(|v| v.as_str())
                .ok_or(ExtractionError::InvalidData(
                    "[NextResponse] Could not find video title",
                ))?;

            let video_thumbnail = lockup_vm
                .query(video_thumbnail_path)
                .first_one()
                .and_then(|f| serde_json::from_value::<Vec<Thumbnail>>(f.clone()).ok())
                .ok_or(ExtractionError::InvalidData(
                    "[NextResponse] Could not find video thumbnails",
                ))?;

            let publish_date = lockup_vm
                .query(publish_date_path)
                .first_one()
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let lenght_text = lockup_vm
                .query(lenght_text_path)
                .first_one()
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let view_count = lockup_vm
                .query(view_count_path)
                .first_one()
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let channel = map_lockup_vm_to_channel_tag(lockup_vm);

            recomended_videos.push(VideoPreview {
                id: video_id.into(),
                title: video_title.into(),
                thumbnail: video_thumbnail,
                publish_date,
                length_text: lenght_text,
                view_count,
                channel,
            });
        }
        Ok(recomended_videos)
    }
}

fn map_lockup_vm_to_channel_tag(lockup_vm: &Value) -> Option<ChannelPreview> {
    let channel_id_path = concat!(
        "$.metadata",
        ".lockupMetadataViewModel",
        ".image",
        ".decoratedAvatarViewModel",
        ".rendererContext",
        ".commandContext",
        ".onTap",
        ".innertubeCommand",
        ".browseEndpoint",
        ".browseId"
    );

    let channel_label_path = concat!(
        "$.metadata",
        ".lockupMetadataViewModel",
        ".image",
        ".decoratedAvatarViewModel",
        ".rendererContext",
        ".commandContext",
        ".onTap",
        ".innertubeCommand",
        ".browseEndpoint",
        ".canonicalBaseUrl"
    );

    let channel_name_path = concat!(
        "$.metadata",
        ".lockupMetadataViewModel",
        ".metadata",
        ".contentMetadataViewModel",
        ".metadataRows[0]",
        ".metadataParts[0]",
        ".text.content",
    );

    let channel_avatar_path = concat!(
        "$.metadata",
        ".lockupMetadataViewModel",
        ".image",
        ".decoratedAvatarViewModel",
        ".avatar",
        ".avatarViewModel",
        ".image",
        ".sources",
    );

    let channel_id = lockup_vm
        .query(channel_id_path)
        .first_one()
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())?;
    let channel_label = lockup_vm
        .query(channel_label_path)
        .first_one()
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())?;
    let channel_name = lockup_vm
        .query(channel_name_path)
        .first_one()
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())?;
    let chanel_avatar = lockup_vm
        .query(channel_avatar_path)
        .first_one()
        .and_then(|f| serde_json::from_value::<Vec<Thumbnail>>(f.clone()).ok())?;

    Some(ChannelPreview {
        id: channel_id,
        name: channel_name,
        label: channel_label,
        avatar: chanel_avatar,
        subscriber_count: None,
    })
}

pub trait QueryResult<T: Copy> {
    fn first_one(self) -> Option<T>;
}

impl<T: Copy, E> QueryResult<T> for std::result::Result<Vec<T>, E> {
    fn first_one(self) -> Option<T> {
        self.ok()?.first().copied()
    }
}
