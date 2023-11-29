use serde::{self, Deserialize, Serialize};

/// # `api/v2/search/start`
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartForm {
    pub pattern: String,
    pub plugins: String,
    pub category: String,
}

/// # `api/v2/search/start`
#[derive(Debug, Clone, Deserialize)]
pub struct StartResponse {
    pub id: u64,
}

/// # `/api/v2/search/stop`
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopForm {
    pub id: u64,
}

/// # `/api/v2/search/status`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct StatusQuery {
    pub id: Option<u64>,
}

/// # `/api/v2/search/status`
pub type StatusResponse = Vec<StatusResponseItem>;

/// # `/api/v2/search/status`
/// [`StatusResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct StatusResponseItem {
    pub id: u64,
    pub status: String,
    pub total: u64,
}

/// # `/api/v2/search/results`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResultsQuery {
    pub id: u64,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// # `/api/v2/search/results`
#[derive(Debug, Clone, Deserialize)]
pub struct ResultsResponse {
    pub results: Vec<ResultsResponseItem>,
    pub status: String,
    pub total: u64,
}

/// # `/api/v2/search/results`
/// [`ResultsResponse`]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsResponseItem {
    pub descr_link: String,
    pub file_name: String,
    pub file_size: f64,
    pub file_url: String,
    pub nb_leechers: u64,
    pub nb_seeders: u64,
    pub site_url: String,
}

/// # `/api/v2/search/delete`
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteForm {
    pub id: u64,
}
