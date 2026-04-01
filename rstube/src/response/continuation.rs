use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, VecSkipError};

use crate::{
    error::ExtractionError,
    models::{convert::FromYtItem, paginator::Page, YouTubeItem},
    response::mapper::{YouTubeListItem, YouTubeListMapper},
    serializer::{MapRespCtx, MapResponse, MapResult},
};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationResponse {
    /// Number of search results (may be absent in continuation responses)
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub estimated_results: Option<u64>,

    #[serde(
        alias = "onResponseReceivedCommands",
        alias = "onResponseReceivedEndpoints"
    )]
    #[serde_as(as = "Option<VecSkipError<_>>")]
    pub on_response_received_actions: Option<Vec<ContinuationActionWrap<YouTubeListItem>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationActionWrap<T> {
    #[serde(alias = "reloadContinuationItemsCommand")]
    pub append_continuation_items_action: ContinuationAction<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinuationAction<T> {
    pub continuation_items: MapResult<Vec<T>>,
}

impl<T: FromYtItem> MapResponse<Page<T>> for ContinuationResponse {
    fn map_response(self, ctx: &MapRespCtx<'_>) -> Result<MapResult<Page<T>>, ExtractionError> {
        let estimated_results = self.estimated_results;

        let items = self
            .on_response_received_actions
            .and_then(|actions| {
                actions
                    .into_iter()
                    .map(|action| action.append_continuation_items_action.continuation_items)
                    .reduce(|mut acc, mut items| {
                        acc.content.append(&mut items.content);
                        acc.warnings.append(&mut items.warnings);
                        acc
                    })
            })
            .unwrap_or_default();

        let mut mapper = YouTubeListMapper::<YouTubeItem>::new(ctx.lang);
        mapper.map_response(items);

        Ok(MapResult {
            content: Page {
                estimated_results,
                corrected_query: None,
                items: mapper
                    .items
                    .into_iter()
                    .filter_map(T::from_yt_item)
                    .collect(),
                ctoken: mapper.ctoken,
                endpoint: ctx.endpoint,
            },
            warnings: Vec::new(),
        })
    }
}
