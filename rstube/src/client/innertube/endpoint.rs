use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YTEndpoint {
    Player,
    Search,
    Browse,
    Next,
    ResolveUrl,
}

impl YTEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Player => "player",
            Self::Search => "search",
            Self::Browse => "browse",
            Self::Next => "next",
            Self::ResolveUrl => "resolve_url",
        }
    }
}
