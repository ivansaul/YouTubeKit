pub(crate) mod config;
pub(crate) mod endpoint;

use crate::{
    client::{
        innertube::{config::ClientType, endpoint::YTEndpoint},
        QBody, RequestOpts,
    },
    serializer::{MapRespCtx, MapResponse},
};
use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[derive(Debug)]
pub struct InnerTube {
    client: reqwest::Client,
}

impl InnerTube {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl InnerTube {
    fn base_url(&self) -> &str {
        "https://www.youtube.com/youtubei/v1"
    }

    pub(crate) async fn execute_request<R, M, B>(
        &self,
        ctype: &ClientType,
        endpoint: YTEndpoint,
        id: &str,
        body: &B,
        opts: &RequestOpts,
    ) -> Result<M, crate::error::Error>
    where
        R: DeserializeOwned + MapResponse<M> + Debug,
        B: Serialize + ?Sized,
    {
        self.execute_request_ctx::<R, M, B>(ctype, endpoint, id, body, opts)
            .await
    }

    async fn execute_request_ctx<R, M, B>(
        &self,
        ctype: &ClientType,
        endpoint: YTEndpoint,
        id: &str,
        body: &B,
        opts: &RequestOpts,
    ) -> Result<M, crate::error::Error>
    where
        R: DeserializeOwned + MapResponse<M> + Debug,
        B: Serialize + ?Sized,
    {
        let config = ctype.config();
        let headers: HeaderMap = (&config.headers).try_into()?;
        let mut client_context = config.context;
        client_context.client.language = Some(opts.language);
        client_context.client.country = Some(opts.country);

        let request_body = QBody {
            context: client_context,
            body,
        };

        let url = format!(
            "{}/{}?key={}",
            self.base_url(),
            endpoint.as_str(),
            config.api_key
        );

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        let map_ctx = MapRespCtx {
            id,
            lang: opts.language,
            endpoint,
        };

        let text = response.text().await?;
        let parsed = serde_json::from_str::<R>(&text)?;
        let mapped = parsed.map_response(&map_ctx)?;

        for warning in &mapped.warnings {
            tracing::warn!("yt extraction warning: {warning}");
        }

        Ok(mapped.content)
    }
}
