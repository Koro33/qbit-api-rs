use serde::{self, Deserialize, Serialize};
use serde_repr::*;
use super::string_saperated_with_vertical_bar;

/// # `/api/v2/transfer/info`
#[derive(Debug, Clone, Deserialize)]
pub struct InfoResponse {
    pub dl_info_speed: u64,
    pub dl_info_data: u64,
    pub up_info_speed: u64,
    pub up_info_data: u64,
    pub dl_rate_limit: u64,
    pub up_rate_limit: u64,
    pub dht_nodes: u64,
    pub connection_status: ConnectionStatus,
}

/// # `/api/v2/transfer/info`
///
/// [`crate::types::sync::ServerState::connection_status`]
///
/// [`InfoResponse::connection_status`]
#[derive(Debug, Clone, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}

/// # `/api/v2/transfer/speedLimitsMode`
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum SpeedLimitsModeResponse {
    Normal = 0,
    Alternative = 1,
}

/// # `/api/v2/transfer/setDownloadLimit`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDownloadLimitForm {
    pub limit: u64,
}

/// # `/api/v2/transfer/setUploadLimit`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetUploadLimitForm {
    pub limit: u64,
}

/// # `/api/v2/transfer/banPeers`
///
/// `peers` format `Vec<host:port>`
#[derive(Debug, Clone, Default, Serialize)]
pub struct BanPeersForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub peers: Vec<String>,
}
