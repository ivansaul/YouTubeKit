use serde::Serialize;

use crate::{
    client::{
        innertube::{config::ClientType, endpoint::YTEndpoint},
        RequestOpts, RusTube,
    },
    error::Error,
    models::{
        convert::FromYtItem,
        paginator::{Page, Paginator},
    },
    response::search::SearchResponse,
};

pub enum SearchType {
    Video,
    Channel,
    Playlist,
    Movie,
    Live,
}

impl SearchType {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Video => "EgIQAQ==",
            Self::Channel => "EgIQAg==",
            Self::Playlist => "EgIQAw==",
            Self::Movie => "EgIQBA==",
            Self::Live => "EgJAAQ==",
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QSearch<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<&'a str>,
}

#[derive(Debug)]
pub struct SearchResult<T> {
    pub paginator: Paginator<T>,
    pub estimated_results: Option<u64>,
    pub corrected_query: Option<String>,
}

pub struct SearchBuilder {
    client: RusTube,
    query: String,
    opts: RequestOpts,
    search_type: Option<SearchType>,
}

impl SearchBuilder {
    pub(crate) fn new(client: RusTube, query: String, opts: RequestOpts) -> Self {
        Self {
            client,
            query,
            opts,
            search_type: None,
        }
    }
}

impl SearchBuilder {
    pub fn only(mut self, stype: SearchType) -> Self {
        self.search_type = Some(stype);
        self
    }

    pub async fn send<T>(self) -> Result<SearchResult<T>, Error>
    where
        T: FromYtItem,
    {
        let ctype = ClientType::WEB;

        let request_body = QSearch {
            query: self.query.as_ref(),
            params: self.search_type.map(|t| t.code()),
        };

        let page: Page<T> = self
            .client
            .inner
            .execute_request::<SearchResponse, _, _>(
                &ctype,
                YTEndpoint::Search,
                &self.query,
                &request_body,
                &self.opts,
            )
            .await?;

        let estimated_results = page.estimated_results;
        let corrected_query = page.corrected_query.clone();

        Ok(SearchResult {
            paginator: Paginator::new(page, self.client, self.opts),
            estimated_results,
            corrected_query,
        })
    }
}
