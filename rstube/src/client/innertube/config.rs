use crate::locale::{Country, Language};
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;

pub struct ClientConfig {
    pub context: ClientContext,
    pub api_key: Cow<'static, str>,
    pub headers: ClientHeaders,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClientContext {
    pub client: ClientInfo,
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_name: Cow<'static, str>,
    pub client_version: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_screen: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_sdk_version: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_make: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<Cow<'static, str>>,
    #[serde(rename = "gl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(rename = "hl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClientHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: Cow<'static, str>,
    #[serde(rename = "X-Youtube-Client-Name")]
    pub x_youtube_client_name: Cow<'static, str>,
    #[serde(rename = "X-Youtube-Client-Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_youtube_client_version: Option<Cow<'static, str>>,
}

impl TryFrom<&ClientHeaders> for HeaderMap {
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(headers: &ClientHeaders) -> Result<Self, Self::Error> {
        let mut map = HeaderMap::new();

        map.insert("User-Agent", headers.user_agent.parse()?);
        map.insert(
            "X-Youtube-Client-Name",
            headers.x_youtube_client_name.parse()?,
        );

        if let Some(ref v) = headers.x_youtube_client_version {
            map.insert("X-Youtube-Client-Version", v.parse()?);
        }

        Ok(map)
    }
}

#[derive(Debug, Default)]
pub enum ClientType {
    #[default]
    WEB,
    // IOS,
}

impl ClientType {
    pub fn config(&self) -> ClientConfig {
        match self {
            ClientType::WEB => ClientConfig {
                context: ClientContext {
                    client: ClientInfo {
                        client_name: Cow::Borrowed("WEB"),
                        client_version: Cow::Borrowed("2.20250523.01.00"),
                        client_screen: None,
                        platform: Some(Cow::Borrowed("DESKTOP")),
                        android_sdk_version: None,
                        os_name: Some(Cow::Borrowed("Windows")),
                        os_version: Some(Cow::Borrowed("10.0")),
                        ..Default::default()
                    },
                },
                headers: ClientHeaders {
                    user_agent: Cow::Borrowed("Mozilla/5.0"),
                    x_youtube_client_name: Cow::Borrowed("1"),
                    x_youtube_client_version: Some(Cow::Borrowed("2.20250523.01.00")),
                },
                api_key: Cow::Borrowed("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"),
            },
        }
    }
}
