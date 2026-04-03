use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    error::ExtractionError,
    models::{convert::FromYtItem, paginator::Page, YouTubeItem},
    response::mapper::{YouTubeListItem, YouTubeListMapper},
    serializer::{MapRespCtx, MapResponse, MapResult},
};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub estimated_results: Option<u64>,
    pub contents: Contents,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Contents {
    pub two_column_search_results_renderer: TwoColumnSearchResultsRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TwoColumnSearchResultsRenderer {
    pub primary_contents: YouTubeListRendererWrap,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct YouTubeListRendererWrap {
    pub section_list_renderer: YouTubeListRenderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct YouTubeListRenderer {
    pub contents: MapResult<Vec<YouTubeListItem>>,
}

impl<T: FromYtItem> MapResponse<Page<T>> for SearchResponse {
    fn map_response(self, ctx: &MapRespCtx<'_>) -> Result<MapResult<Page<T>>, ExtractionError> {
        let items = self
            .contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents;

        let mut mapper = YouTubeListMapper::<YouTubeItem>::new(ctx.lang);
        mapper.map_response(items);

        Ok(MapResult {
            content: Page {
                items: mapper
                    .items
                    .into_iter()
                    .filter_map(T::from_yt_item)
                    .collect(),
                ctoken: mapper.ctoken,
                endpoint: ctx.endpoint,
                estimated_results: self.estimated_results,
                corrected_query: mapper.corrected_query,
            },
            warnings: mapper.warnings,
        })
    }
}
