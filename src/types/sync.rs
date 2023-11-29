use super::torrents::{CategoriesResponse, InfoState};
use super::transfer::ConnectionStatus;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

/// # `/api/v2/sync/maindata`
#[derive(Debug, Clone, Default, Serialize)]
pub struct MaindataQuery {
    pub rid: u64,
}

/// # `/api/v2/sync/maindata`
#[derive(Debug, Clone, Deserialize)]
pub struct MaindataResponse {
    pub rid: u64,
    pub full_update: Option<bool>,
    pub torrents: Option<MaindataTorrentsResponse>,
    pub torrents_removed: Option<Vec<String>>,
    pub categories: Option<CategoriesResponse>, // Note: catagories are not mentioned in docs
    pub categories_removed: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub tags_removed: Option<Vec<String>>,
    pub queueing: Option<bool>,
    pub server_state: Option<ServerState>, // Note: server_state are not mentioned in docs
}

/// # `/api/v2/sync/maindata`
/// [`MaindataResponse::torrents`]
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct MaindataTorrentsResponse {
    pub data: HashMap<String, MaindataTorrentsResponseItem>,
}

/// # `/api/v2/sync/maindata`
/// [`MaindataTorrentsResponse::data`]
///
/// `Note`: similar to `TorrentsInfoResponseItem` but no `hash` field
#[derive(Debug, Clone, Deserialize)]
pub struct MaindataTorrentsResponseItem {
    pub added_on: Option<u64>,
    pub amount_left: Option<u64>,
    pub auto_tmm: Option<bool>,
    pub category: Option<String>,
    pub completed: Option<i64>,
    pub completion_on: Option<u64>,
    pub dl_limit: Option<i64>,
    pub dlspeed: Option<i64>,
    pub downloaded: Option<i64>,
    pub downloaded_session: Option<i64>,
    pub eta: Option<i64>,
    pub f_l_piece_prio: Option<bool>,
    pub force_start: Option<bool>,
    // pub hash: String,
    pub last_activity: Option<u64>,
    pub magnet_uri: Option<String>,
    pub max_ratio: Option<f64>,
    pub max_seeding_time: Option<i64>,
    pub name: Option<String>,
    pub num_complete: Option<i64>,
    pub num_incomplete: Option<i64>,
    pub num_leechs: Option<i64>,
    pub num_seeds: Option<i64>,
    pub priority: Option<i64>,
    pub progress: Option<f64>,
    pub ratio: Option<f64>,
    pub ratio_limit: Option<f64>,
    pub save_path: Option<String>,
    pub seeding_time_limit: Option<i64>,
    pub seen_complete: Option<i64>,
    pub seq_dl: Option<bool>,
    pub size: Option<i64>,
    pub state: Option<InfoState>,
    pub super_seeding: Option<bool>,
    pub tags: Option<String>,
    pub time_active: Option<i64>,
    pub total_size: Option<i64>,
    pub tracker: Option<String>,
    pub up_limit: Option<i64>,
    pub uploaded: Option<i64>,
    pub uploaded_session: Option<i64>,
    pub upspeed: Option<i64>,
}

/// # `/api/v2/sync/maindata`
/// [`MaindataResponse::server_state`]
#[derive(Debug, Clone, Deserialize)]
pub struct ServerState {
    pub alltime_dl: Option<u64>,
    pub alltime_ul: Option<u64>,
    pub average_time_queue: Option<u64>,
    pub free_space_on_disk: Option<u64>,
    pub global_ratio: Option<String>,
    pub queued_io_jobs: Option<u64>,
    pub queueing: Option<bool>,
    pub read_cache_hits: Option<String>,
    pub read_cache_overload: Option<String>,
    pub refresh_interval: Option<u64>,
    pub total_buffers_size: Option<u64>,
    pub total_peer_connections: Option<u64>,
    pub total_queued_size: Option<u64>,
    pub total_wasted_session: Option<u64>,
    pub use_alt_speed_limits: Option<bool>,
    pub write_cache_overload: Option<String>,

    pub dl_info_speed: Option<u64>,
    pub dl_info_data: Option<u64>,
    pub up_info_speed: Option<u64>,
    pub up_info_data: Option<u64>,
    pub dl_rate_limit: Option<u64>,
    pub up_rate_limit: Option<u64>,
    pub dht_nodes: Option<u64>,
    pub connection_status: Option<ConnectionStatus>,
}

/// # `/api/v2/sync/torrentPeers`
#[derive(Debug, Clone, Default, Serialize)]
pub struct TorrentPeersQuery {
    pub hash: String,
    pub rid: u64,
}

/// # `/api/v2/sync/torrentPeers`
///
/// `Note`: The response's format remains TODO status in docs
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct TorrentPeersResponse {
    pub full_update: bool,
    pub peers: HashMap<String, TorrentPeer>,
}

/// # `/api/v2/sync/torrentPeers`
/// [`TorrentPeersResponse::peers`]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TorrentPeer {
    pub client: String,
    pub connection: String,
    pub country: String,
    pub country_code: String,
    pub dl_speed: u64,
    pub downloaded: u64,
    pub files: String,
    pub flags: String,
    pub flags_desc: String,
    pub ip: String,
    pub peer_id_client: String,
    pub port: u16,
    pub progress: f64,
    pub relevance: f64,
    pub up_speed: u64,
    pub uploaded: u64,
}
