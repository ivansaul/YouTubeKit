use serde::Deserialize;

use crate::{
    client::{
        innertube::{config::ClientType, endpoint::YTEndpoint},
        QContinuation, RequestOpts, RusTube,
    },
    error::Error,
    models::convert::FromYtItem,
    response::continuation::ContinuationResponse,
};

/// A page of items fetched from the YouTube API.
/// Wraps a list of items and the continuation token for progressive loading.
#[derive(Debug, Deserialize)]
pub struct Page<T> {
    /// Content of the page
    pub(crate) items: Vec<T>,
    /// Continuation token to fetch the next page.
    ///
    /// `None` means there are no more pages available.
    pub(crate) ctoken: Option<String>,
    /// YouTube API endpoint to fetch continuations from.
    pub(crate) endpoint: YTEndpoint,
    /// Estimated total number of results (pagination hint only, not exact).
    ///
    /// Intended to be shown to the user (e.g. "1,261 results").
    /// Do not use for iterating or checking completion.
    pub estimated_results: Option<u64>,
    /// Query corrected by YouTube (e.g. typo correction).
    pub corrected_query: Option<String>,
}

/// A stateful paginator for fetching items from the YouTube API in pages.
///
/// Created from an initial [`Page`], and allows fetching subsequent pages
/// via [`Paginator::next_page`].
#[derive(Debug)]
pub struct Paginator<T> {
    pub(crate) page: Page<T>,
    pub(crate) client: RusTube,
    pub(crate) opts: RequestOpts,
}

impl<T: FromYtItem> Paginator<T> {
    pub(crate) fn new(page: Page<T>, client: RusTube, opts: RequestOpts) -> Self {
        Self { page, client, opts }
    }

    /// Returns the current page's items.
    pub fn items(&self) -> &[T] {
        &self.page.items
    }

    /// Fetches the next page of items.
    ///
    /// Returns `Ok(Some(&[T]))` with the new items, or `Ok(None)` if there
    /// are no more pages.
    pub async fn next_page(&mut self) -> Result<Option<&[T]>, Error> {
        let token = match &self.page.ctoken {
            Some(t) => t,
            None => return Ok(None),
        };

        let request_body = QContinuation {
            continuation: token,
        };

        let next_page: Page<T> = self
            .client
            .inner
            .execute_request::<ContinuationResponse, _, _>(
                &ClientType::WEB,
                self.page.endpoint,
                "continuation",
                &request_body,
                &self.opts,
            )
            .await?;

        self.page = next_page;
        Ok(Some(&self.page.items))
    }
}
