use crate::error::TypesError;
use serde::{self, Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashMap;

/// # `/api/v2/auth/login`
#[derive(Debug, Clone, Serialize)]
pub struct AuthLoginForm {
    pub username: String,
    pub password: String,
}

/// # `/api/v2/app/buildInfo`
#[derive(Debug, Deserialize)]
pub struct AppBuildInfoResponse {
    pub qt: String,
    pub libtorrent: String,
    pub boost: String,
    pub openssl: String,
    pub bitness: u32,
}

/// # `/api/v2/app/preferences`
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppPreferences {
    pub locale: Option<String>,
    // pub create_subfolder_enabled: bool, // described in docs but not appear
    pub start_paused_enabled: Option<bool>,
    pub auto_delete_mode: Option<i64>,
    pub preallocate_all: Option<bool>,
    pub incomplete_files_ext: Option<bool>,
    pub auto_tmm_enabled: Option<bool>,
    pub torrent_changed_tmm_enabled: Option<bool>,
    pub save_path_changed_tmm_enabled: Option<bool>,
    pub category_changed_tmm_enabled: Option<bool>,
    pub save_path: Option<String>,
    pub temp_path_enabled: Option<bool>,
    pub temp_path: Option<String>,
    pub scan_dirs: Option<HashMap<String, i64>>,
    pub export_dir: Option<String>,
    pub export_dir_fin: Option<String>,
    pub mail_notification_enabled: Option<bool>,
    pub mail_notification_sender: Option<String>,
    pub mail_notification_email: Option<String>,
    pub mail_notification_smtp: Option<String>,
    pub mail_notification_ssl_enabled: Option<bool>,
    pub mail_notification_auth_enabled: Option<bool>,
    pub mail_notification_username: Option<String>,
    pub mail_notification_password: Option<String>,
    pub autorun_enabled: Option<bool>,
    pub autorun_program: Option<String>,
    pub queueing_enabled: Option<bool>,
    pub max_active_downloads: Option<i64>,
    pub max_active_torrents: Option<i64>,
    pub max_active_uploads: Option<i64>,
    pub dont_count_slow_torrents: Option<bool>,
    pub slow_torrent_dl_rate_threshold: Option<i64>,
    pub slow_torrent_ul_rate_threshold: Option<i64>,
    pub slow_torrent_inactive_timer: Option<i64>,
    pub max_ratio_enabled: Option<bool>,
    pub max_ratio: Option<f64>,
    pub max_ratio_act: Option<MaxRatioAct>,
    pub listen_port: Option<i64>,
    pub upnp: Option<bool>,
    pub random_port: Option<bool>,
    pub dl_limit: Option<i64>,
    pub up_limit: Option<i64>,
    pub max_connec: Option<i64>,
    pub max_connec_per_torrent: Option<i64>,
    pub max_uploads: Option<i64>,
    pub max_uploads_per_torrent: Option<i64>,
    pub stop_tracker_timeout: Option<i64>,
    pub enable_piece_extent_affinity: Option<bool>,
    pub bittorrent_protocol: Option<BittorrentProtocol>,
    pub limit_utp_rate: Option<bool>,
    pub limit_tcp_overhead: Option<bool>,
    pub limit_lan_peers: Option<bool>,
    pub alt_dl_limit: Option<i64>,
    pub alt_up_limit: Option<i64>,
    pub scheduler_enabled: Option<bool>,
    pub schedule_from_hour: Option<i64>,
    pub schedule_from_min: Option<i64>,
    pub schedule_to_hour: Option<i64>,
    pub schedule_to_min: Option<i64>,
    pub scheduler_days: Option<SchedulerDays>,
    pub dht: Option<bool>,
    pub pex: Option<bool>,
    pub lsd: Option<bool>,
    pub encryption: Option<Encryption>,
    pub anonymous_mode: Option<bool>,
    pub proxy_type: Option<ProxyType>,
    pub proxy_ip: Option<String>,
    pub proxy_port: Option<i64>,
    pub proxy_peer_connections: Option<bool>,
    pub proxy_auth_enabled: Option<bool>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_torrents_only: Option<bool>,
    pub ip_filter_enabled: Option<bool>,
    pub ip_filter_path: Option<String>,
    pub ip_filter_trackers: Option<bool>,
    pub web_ui_domain_list: Option<String>,
    pub web_ui_address: Option<String>,
    pub web_ui_port: Option<i64>,
    pub web_ui_upnp: Option<bool>,
    pub web_ui_username: Option<String>,
    #[serde(skip_deserializing)]
    pub web_ui_password: Option<String>,
    pub web_ui_csrf_protection_enabled: Option<bool>,
    pub web_ui_clickjacking_protection_enabled: Option<bool>,
    pub web_ui_secure_cookie_enabled: Option<bool>,
    pub web_ui_max_auth_fail_count: Option<i64>,
    pub web_ui_ban_duration: Option<i64>,
    pub web_ui_session_timeout: Option<i64>,
    pub web_ui_host_header_validation_enabled: Option<bool>,
    pub bypass_local_auth: Option<bool>,
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,
    pub bypass_auth_subnet_whitelist: Option<String>,
    pub alternative_webui_enabled: Option<bool>,
    pub alternative_webui_path: Option<String>,
    pub use_https: Option<bool>,
    pub web_ui_https_key_path: Option<String>,
    pub web_ui_https_cert_path: Option<String>,
    pub dyndns_enabled: Option<bool>,
    pub dyndns_service: Option<DynDnsService>,
    pub dyndns_username: Option<String>,
    pub dyndns_password: Option<String>,
    pub dyndns_domain: Option<String>,
    pub rss_refresh_interval: Option<i64>,
    pub rss_max_articles_per_feed: Option<i64>,
    pub rss_processing_enabled: Option<bool>,
    pub rss_auto_downloading_enabled: Option<bool>,
    pub rss_download_repack_proper_episodes: Option<bool>,
    pub rss_smart_episode_filters: Option<String>,
    pub add_trackers_enabled: Option<bool>,
    pub add_trackers: Option<String>,
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,
    pub web_ui_custom_http_headers: Option<String>,
    pub max_seeding_time_enabled: Option<bool>,
    pub max_seeding_time: Option<i64>,
    pub announce_ip: Option<String>,
    pub announce_to_all_tiers: Option<bool>,
    pub announce_to_all_trackers: Option<bool>,
    pub async_io_threads: Option<i64>,
    #[serde(rename = "banned_IPs")]
    pub banned_ips: Option<String>,
    pub checking_memory_use: Option<i64>,
    pub current_interface_address: Option<String>,
    pub current_network_interface: Option<String>,
    pub disk_cache: Option<i64>,
    pub disk_cache_ttl: Option<i64>,
    pub embedded_tracker_port: Option<i64>,
    pub enable_coalesce_read_write: Option<bool>,
    pub enable_embedded_tracker: Option<bool>,
    pub enable_multi_connections_from_same_ip: Option<bool>,
    // pub enable_os_cache: bool, // described in docs but not appear
    pub enable_upload_suggestions: Option<bool>,
    pub file_pool_size: Option<i64>,
    pub outgoing_ports_max: Option<i64>,
    pub outgoing_ports_min: Option<i64>,
    pub recheck_completed_torrents: Option<bool>,
    pub resolve_peer_countries: Option<bool>,
    pub save_resume_data_interval: Option<i64>,
    pub send_buffer_low_watermark: Option<i64>,
    pub send_buffer_watermark: Option<i64>,
    pub send_buffer_watermark_factor: Option<i64>,
    pub socket_backlog_size: Option<i64>,
    pub upload_choking_algorithm: Option<UploadChokingAlgorithm>,
    pub upload_slots_behavior: Option<UploadSlotsBehavior>,
    pub upnp_lease_duration: Option<i64>,
    pub utp_tcp_mixed_mode: Option<UtpTcpMixedMode>,
}

/// # `/api/v2/app/setPreferences`
/// `Warning`: some fields should be set together.
/// For example: Setting `max_ratio` separatly has no effect unless with `max_ratio_enabled` together.
///
/// A better methed is to get the current preferences from `/api/v2/app/preferences` and change the fields within,
/// then send the changed preferences back to `/api/v2/app/setPreferences`
#[derive(Debug, Serialize)]
pub struct AppSetPreferencesForm {
    #[serde(with = "preferences_serialize")]
    pub json: AppPreferences,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::scheduler_days`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SchedulerDays {
    EveryDay = 0,
    EveryWeekday = 1,
    EveryWeekend = 2,
    EveryMonday = 3,
    EveryTuesday = 4,
    EveryWednesday = 5,
    EveryThursday = 6,
    EveryFriday = 7,
    EverySaturday = 8,
    EverySunday = 9,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::encryption`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Encryption {
    PreferEncryption = 0,
    ForceEncryptionOn = 1,
    ForceEncryptionOff = 2,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::proxy_type`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ProxyType {
    Disabled = 0,
    Http = 1,
    Socks5 = 2,
    HttpWithAuth = 3,
    Socks5WithAuth = 4,
    Socks4WithAuth = 5,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::dyndns_service`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum DynDnsService {
    DyDNS = 0,
    NOIP = 1,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::bittorrent_protocol`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MaxRatioAct {
    Pause = 0,
    Remove = 1,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::limit_utp_rate`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum BittorrentProtocol {
    TCPUTP = 0,
    TCP = 1,
    UTP = 2,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::upload_choking_algorithm`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::upload_slots_behavior`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    FixedSlots = 0,
    RateBased = 1,
}

/// # `/api/v2/app/preferences`
/// [`AppPreferences::utp_tcp_mixed_mode`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTCP = 0,
    PeerProportional = 1,
}

/// # `/api/v2/log/main`
#[derive(Debug, Serialize)]
pub struct LogMainQuery {
    pub normal: bool,
    pub info: bool,
    pub warning: bool,
    pub critical: bool,
    pub last_known_id: i64,
}

impl Default for LogMainQuery {
    fn default() -> Self {
        Self {
            normal: true,
            info: true,
            warning: true,
            critical: true,
            last_known_id: -1,
        }
    }
}

/// # `/api/v2/log/main`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LogMainResponse {
    pub data: Vec<LogMainResponseItem>,
}

/// # `/api/v2/log/main`
/// [`LogMainResponse::data`]
#[derive(Debug, Deserialize)]
pub struct LogMainResponseItem {
    pub id: u64,
    pub message: String,
    pub timestamp: u64,
    pub r#type: LogMainType,
}

/// # `/api/v2/log/main`
/// [`LogMainResponseItem::type`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum LogMainType {
    NORMAL = 1,
    INFO = 2,
    WARNING = 4,
    CRITICAL = 8,
}

/// # `/api/v2/log/peers`
#[derive(Debug, Serialize)]
pub struct LogPeersQuery {
    pub last_known_id: i64,
}

impl Default for LogPeersQuery {
    fn default() -> Self {
        Self { last_known_id: -1 }
    }
}

/// # `/api/v2/log/peers`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LogPeersResponse {
    pub data: Vec<LogPeersResponseItem>,
}

/// # `/api/v2/log/peers`
/// [`LogPeersResponse::data`]
#[derive(Debug, Deserialize)]
pub struct LogPeersResponseItem {
    pub id: u64,
    pub ip: String,
    pub timestamp: u64,
    pub blocked: bool,
    pub reason: String,
}

/// # `/api/v2/sync/maindata`
#[derive(Debug, Default, Serialize)]
pub struct SyncMaindataQuery {
    pub rid: u64,
}

/// # `/api/v2/sync/maindata`
#[derive(Debug, Deserialize)]
pub struct SyncMaindataResponse {
    pub rid: u64,
    pub full_update: Option<bool>,
    pub torrents: Option<SyncMaindataTorrentsResponse>,
    pub torrents_removed: Option<Vec<String>>,
    pub categories: Option<TorrentsCategoriesResponse>, // Note: catagories are not mentioned in docs
    pub categories_removed: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub tags_removed: Option<Vec<String>>,
    pub queueing: Option<bool>,
    pub server_state: Option<ServerState>, // Note: server_state are not mentioned in docs
}

/// # `/api/v2/sync/maindata`
/// [`SyncMaindataResponse::torrents`]
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SyncMaindataTorrentsResponse {
    pub data: HashMap<String, SyncMaindataTorrentsResponseItem>,
}

/// # `/api/v2/sync/maindata`
/// [`SyncMaindataTorrentsResponse::data`]
///
/// `Note`: similar to `TorrentsInfoResponseItem` but no `hash` field
#[derive(Debug, Deserialize)]
pub struct SyncMaindataTorrentsResponseItem {
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
    pub state: Option<TorrentsInfoState>,
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
/// [`SyncMaindataResponse::server_state`]
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Default, Serialize)]
pub struct SyncTorrentPeersQuery {
    pub hash: String,
    pub rid: u64,
}

/// # `/api/v2/sync/torrentPeers`
///
/// `Note`: The response's format remains TODO status in docs
#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct SyncTorrentPeersResponse {
    pub full_update: bool,
    pub peers: HashMap<String, SyncTorrentPeer>,
}

/// # `/api/v2/sync/torrentPeers`
/// [`SyncTorrentPeersResponse::peers`]
#[derive(Debug, PartialEq, Deserialize)]
pub struct SyncTorrentPeer {
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

/// # `/api/v2/transfer/info`
#[derive(Debug, Deserialize)]
pub struct TransferInfoResponse {
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
/// [`ServerState::connection_status`]
///
/// [`TransferInfoResponse::connection_status`]
#[derive(Debug, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}

/// # `/api/v2/transfer/speedLimitsMode`
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SpeedLimitsModeResponse {
    Normal = 0,
    Alternative = 1,
}

/// # `/api/v2/transfer/setDownloadLimit`
#[derive(Debug, Serialize)]
pub struct SetDownloadLimitForm {
    pub limit: u64,
}

/// # `/api/v2/transfer/setUploadLimit`
#[derive(Debug, Serialize)]
pub struct SetUploadLimitForm {
    pub limit: u64,
}

/// # `/api/v2/transfer/banPeers`
///
/// `peers` format `Vec<host:port>`
#[derive(Debug, Serialize)]
pub struct BanPeersForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub peers: Vec<String>,
}

/// # `/api/v2/torrents/info`
#[derive(Debug, Serialize, Default)]
pub struct TorrentsInfoQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TorrentsInfoFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Hashes>,
}

/// # `/api/v2/torrents/info`
/// [`TorrentsInfoQuery::filter`]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TorrentsInfoFilter {
    All,
    Downloading,
    Seeding,
    Completed,
    Paused,
    Active,
    Inactive,
    Resumed,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}

/// # `/api/v2/torrents/info`
/// [`TorrentsInfoQuery::hashes`]
#[derive(Debug, Serialize, Deserialize)]
pub struct Hashes(#[serde(with = "string_saperated_with_vertical_bar")] pub Vec<String>);

/// # `/api/v2/torrents/info`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsInfoResponse {
    pub data: Vec<TorrentsInfoResponseItem>,
}

/// # `/api/v2/torrents/info`
/// [`TorrentsInfoResponse::data`]
#[derive(Debug, Deserialize)]
pub struct TorrentsInfoResponseItem {
    pub added_on: u64,
    pub amount_left: u64,
    pub auto_tmm: bool,
    pub category: String,
    pub completed: i64,
    pub completion_on: u64,
    pub dl_limit: i64,
    pub dlspeed: i64,
    pub downloaded: i64,
    pub downloaded_session: i64,
    pub eta: i64,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub hash: String,
    pub last_activity: u64,
    pub magnet_uri: String,
    pub max_ratio: f64,
    pub max_seeding_time: i64,
    pub name: String,
    pub num_complete: i64,
    pub num_incomplete: i64,
    pub num_leechs: i64,
    pub num_seeds: i64,
    pub priority: i64,
    pub progress: f64,
    pub ratio: f64,
    pub ratio_limit: f64,
    pub save_path: String,
    pub seeding_time_limit: i64,
    pub seen_complete: i64,
    pub seq_dl: bool,
    pub size: i64,
    pub state: TorrentsInfoState,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: i64,
    pub total_size: i64,
    pub tracker: String,
    pub up_limit: i64,
    pub uploaded: i64,
    pub uploaded_session: i64,
    pub upspeed: i64,
}

/// # `/api/v2/torrents/info`
/// [`TorrentsInfoResponseItem::state`]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TorrentsInfoState {
    Error,
    MissingFiles,
    Uploading,
    PausedUP,
    QueuedUP,
    StalledUP,
    CheckingUP,
    ForcedUP,
    Allocating,
    Downloading,
    MetaDL,
    PausedDL,
    QueuedDL,
    StalledDL,
    CheckingDL,
    ForceDL,
    CheckingResumeData,
    Moving,
    Unknown,
}

/// # `/api/v2/torrents/properties`
#[derive(Debug, Serialize)]
pub struct TorrentsPropertiesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/properties`
#[derive(Debug, Deserialize)]
pub struct TorrentsPropertiesResponse {
    pub save_path: String,
    pub creation_date: u64,
    pub piece_size: i64,
    pub comment: String,
    pub total_wasted: i64,
    pub total_uploaded: i64,
    pub total_uploaded_session: i64,
    pub total_downloaded: i64,
    pub total_downloaded_session: i64,
    pub up_limit: i64,
    pub dl_limit: i64,
    pub time_elapsed: i64,
    pub seeding_time: i64,
    pub nb_connections: i64,
    pub nb_connections_limit: i64,
    pub share_ratio: f64,
    pub addition_date: i64,
    pub completion_date: i64,
    pub created_by: String,
    pub dl_speed_avg: i64,
    pub dl_speed: i64,
    pub eta: i64,
    pub last_seen: i64,
    pub peers: i64,
    pub peers_total: i64,
    pub pieces_have: u64,
    pub pieces_num: i64,
    pub reannounce: i64,
    pub seeds: i64,
    pub seeds_total: i64,
    pub total_size: u64,
    pub up_speed_avg: i64,
    pub up_speed: i64,
}

/// # `/api/v2/torrents/trackers`
#[derive(Debug, Serialize)]
pub struct TorrentsTrackersQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/trackers`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsTrackersResponse {
    pub data: Vec<TorrentsTrackersResponseItem>,
}

/// # `/api/v2/torrents/trackers`
/// [`TorrentsTrackersResponse::data`]
#[derive(Debug, Deserialize)]
pub struct TorrentsTrackersResponseItem {
    pub url: String,
    pub status: TrackerStatus,
    pub tier: i64,
    pub num_peers: i64,
    pub num_seeds: i64,
    pub num_leeches: i64,
    pub num_downloaded: i64,
    pub msg: String,
}

/// # `/api/v2/torrents/trackers`
/// [`TorrentsTrackersResponseItem::status`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TrackerStatus {
    Disabled = 0,
    NotContacted = 1,
    Working = 2,
    Updating = 3,
    NotWorking = 4,
}

/// # `/api/v2/torrents/webseeds`
#[derive(Debug, Serialize)]
pub struct TorrentsWebseedsQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/webseeds`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsWebseedsResponse {
    pub data: Vec<TorrentsWebseedsResponseItem>,
}

/// # `/api/v2/torrents/webseeds`
/// [`TorrentsWebseedsResponse::data`]
#[derive(Debug, Deserialize)]
pub struct TorrentsWebseedsResponseItem {
    pub url: String,
}

/// # `/api/v2/torrents/files`
#[derive(Debug, Default, Serialize)]
pub struct TorrentsFilesQuery {
    pub hash: String,
    pub indexes: Option<FileIndexes>,
}

/// # `/api/v2/torrents/files`
/// [`TorrentsFilesQuery::indexes`]
#[derive(Debug, Serialize)]
pub struct FileIndexes(
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")] pub Vec<String>,
);

/// # `/api/v2/torrents/files`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsFilesResponse {
    pub data: Vec<TorrentsFilesResponseItem>,
}

/// # `/api/v2/torrents/files`
/// [`TorrentsFilesResponse::data`]
#[derive(Debug, Deserialize)]
pub struct TorrentsFilesResponseItem {
    pub index: u64,
    pub name: String,
    pub size: u64,
    pub progress: f64,
    pub priority: TorrentsFilesPriority,
    pub is_seed: Option<bool>,
    pub piece_range: Vec<u64>,
    pub availability: f64,
}

/// # `/api/v2/torrents/files`
/// [`TorrentsFilesResponseItem::priority`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TorrentsFilesPriority {
    NotDownload = 0,
    Normal = 1,
    High = 6,
    Maximal = 7,
}

/// # `/api/v2/torrents/pieceStates`
#[derive(Debug, Serialize)]
pub struct TorrentsPieceStatesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/pieceStates`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsPieceStatesResponse {
    pub data: Vec<TorrentsPieceStates>,
}

/// # `/api/v2/torrents/pieceStates`
/// [`TorrentsPieceStatesResponse::data`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TorrentsPieceStates {
    NotDownloaded = 0,
    Downloading = 1,
    Downloaded = 2,
}

/// # `/api/v2/torrents/pieceHashes`
#[derive(Debug, Serialize)]
pub struct TorrentsPieceHashesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/pieceHashes`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsPieceHashesResponse {
    pub data: Vec<String>,
}

/// # `/api/v2/torrents/pause`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsPauseForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsPauseForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/resume`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsResumeForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsResumeForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/delete`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsDeleteForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(rename = "deleteFiles")]
    pub delete_files: bool,
}

impl TorrentsDeleteForm {
    pub fn all(&mut self, delete_files: bool) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
        self.delete_files = delete_files;
    }
}

/// # `/api/v2/torrents/recheck`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsRecheckForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsRecheckForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/reannounce`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsReannounceForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsReannounceForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/add`
#[derive(Debug, Default)]
pub struct TorrentsAddMultipart {
    pub urls: Vec<String>,
    pub torrents: Vec<(String, Vec<u8>)>,
    pub savepath: Option<String>,
    pub cookie: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub skip_hash_check: Option<bool>,
    pub paused: Option<bool>,
    pub root_folder: Option<bool>,
    pub rename: Option<String>,
    pub upload_limit: Option<i64>,
    pub download_limit: Option<i64>,
    pub ratio_limit: Option<f32>,
    pub seeding_time_limit: Option<u64>,
    pub auto_tmm: Option<bool>,
    pub sequential_download: Option<bool>,
    pub first_last_piece_prio: Option<bool>,
}

impl TorrentsAddMultipart {
    pub fn to_multipart_form(&self) -> Result<reqwest::multipart::Form, TypesError> {
        if self.urls.is_empty() && self.torrents.is_empty() {
            return Err(TypesError::Other(
                "Either `urls` or `torrents` must be set!".into(),
            ));
        }

        let mut form = reqwest::multipart::Form::new();

        // Add urls separated by `\n`
        if !self.urls.is_empty() {
            let urls = self.urls.join("\n");

            form = form.text("urls", urls);
        }

        // Add torrent files
        if !self.torrents.is_empty() {
            for torrent in self.torrents.iter() {
                form = form.part(
                    "torrents",
                    reqwest::multipart::Part::bytes(torrent.1.clone())
                        .file_name(torrent.0.clone())
                        .mime_str("application/x-bittorrent")
                        .unwrap(), // can't fail
                );
            }
        }

        if let Some(savepath) = &self.savepath {
            form = form.text("savepath", savepath.to_owned());
        }

        if let Some(cookie) = &self.cookie {
            form = form.text("cookie", cookie.to_owned());
        }

        if let Some(category) = &self.category {
            form = form.text("category", category.to_owned());
        }

        if let Some(tags) = &self.tags {
            let tags = tags.join(",");
            form = form.text("tags", tags);
        }

        if let Some(skip_hash_check) = &self.skip_hash_check {
            form = form.text("skip_checking", skip_hash_check.to_string());
        }

        if let Some(paused) = &self.paused {
            form = form.text("paused", paused.to_string());
        }

        if let Some(root_folder) = &self.root_folder {
            form = form.text("root_folder", root_folder.to_string());
        }

        if let Some(rename) = &self.rename {
            form = form.text("rename", rename.to_owned());
        }

        if let Some(upload_limit) = &self.upload_limit {
            form = form.text("upLimit", upload_limit.to_string());
        }

        if let Some(download_limit) = &self.download_limit {
            form = form.text("dlLimit", download_limit.to_string());
        }

        if let Some(ratio_limit) = &self.ratio_limit {
            form = form.text("ratioLimit", ratio_limit.to_string());
        }

        if let Some(seeding_time_limit) = &self.seeding_time_limit {
            form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        }

        if let Some(auto_tmm) = &self.auto_tmm {
            form = form.text("autoTMM", auto_tmm.to_string());
        }

        if let Some(sequential_download) = &self.sequential_download {
            form = form.text("sequentialDownload", sequential_download.to_string());
        }

        if let Some(first_last_piece_prio) = &self.first_last_piece_prio {
            form = form.text("firstLastPiecePrio", first_last_piece_prio.to_string());
        }

        Ok(form)
    }
}

/// # `/api/v2/torrents/addTrackers`
#[derive(Debug, Serialize)]
pub struct TorrentsAddTrackersForm {
    pub hash: String,
    #[serde(serialize_with = "string_saperated_with_backslash_n::serialize")]
    pub urls: Vec<String>,
}

/// # `/api/v2/torrents/editTracker`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsEditTrackerForm {
    pub hash: String,
    pub orig_url: String,
    pub new_url: String,
}

/// # `/api/v2/torrents/removeTrackers`
#[derive(Debug, Serialize)]
pub struct TorrentsRemoveTrackersForm {
    pub hash: String,
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub urls: Vec<String>,
}

/// # `/api/v2/torrents/addPeers`
#[derive(Debug, Serialize)]
pub struct TorrentsAddPeersForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub peers: Vec<String>,
}

/// # `/api/v2/torrents/increasePrio`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsIncreasePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsIncreasePrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/decreasePrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsDecreasePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsDecreasePrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/topPrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsTopPrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsTopPrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/bottomPrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Serialize)]
pub struct TorrentsBottomPrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TorrentsBottomPrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/filePrio`
// TODO: Implement

/// # `/api/v2/torrents/downloadLimit`
///
/// `Warning`: it was described in the docs that hashes can be setted to "all",
///  but it doesn't work
#[derive(Debug, Serialize)]
pub struct TorrentsDownloadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/downloadLimit`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsDownloadLimitResponse {
    pub data: HashMap<String, u64>,
}

/// # `/api/v2/torrents/setDownloadLimit`
#[derive(Debug, Serialize)]
pub struct TorrentsSetDownloadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    /// - `0` -> no limit
    pub limit: u64,
}

/// # `/api/v2/torrents/setShareLimits`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsSetShareLimitsForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    /// - `-2` -> global limit used
    /// - `-1` -> no limit.
    #[serde(flatten)]
    pub ratio_limit: RatioLimit,
    /// in minutes
    /// - `-2` -> global limit used
    /// - `-1` -> no limit.
    pub seeding_time_limit: i64,
}

/// # `/api/v2/torrents/setShareLimits`
/// [`TorrentsSetShareLimitsForm::ratio_limit`]
///
/// define this type to solve the problem that if ratio_limit is a f64,
/// the value -2 and -1 will be serialized to -2.0 and -1.0
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RatioLimit {
    Limit {
        #[serde(rename = "ratioLimit")]
        ratio_limit: f64,
    },
    Special {
        #[serde(rename = "ratioLimit")]
        ratio_limit: i8,
    },
}

/// # `/api/v2/torrents/uploadLimit`
///
/// `Warning`: it was described in the docs that hashes can be setted to "all",
///  but it doesn't work
#[derive(Debug, Serialize)]
pub struct TorrentsUploadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/uploadLimit`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsUploadLimitResponse {
    pub data: HashMap<String, u64>,
}

/// # `/api/v2/torrents/setUploadLimit`
#[derive(Debug, Serialize)]
pub struct TorrentsSetUploadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    /// - `0` -> no limit
    pub limit: u64,
}

/// # `/api/v2/torrents/setLocation`
#[derive(Debug, Serialize)]
pub struct TorrentsSetLocationForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub location: String,
}

/// # `/api/v2/torrents/rename`
#[derive(Debug, Serialize)]
pub struct TorrentsRenameForm {
    pub hash: String,
    pub name: String,
}

/// # `/api/v2/torrents/setCategory`
#[derive(Debug, Serialize)]
pub struct TorrentsSetCategoryForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub category: String,
}

/// # `/api/v2/torrents/categories`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsCategoriesResponse {
    pub catagories: HashMap<String, CategoriesDetails>,
}

/// # `/api/v2/torrents/categories`
/// [`TorrentsCategoriesResponse::catagories`]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesDetails {
    pub name: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/createCategory`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsCreateCategoryForm {
    pub category: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/editCategory`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsEditCategoryForm {
    pub category: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/removeCategories`
#[derive(Debug, Serialize)]
pub struct TorrentsRemoveCategoriesForm {
    #[serde(serialize_with = "string_saperated_with_backslash_n::serialize")]
    pub categories: Vec<String>,
}

/// # `/api/v2/torrents/addTags`
#[derive(Debug, Serialize)]
pub struct TorrentsAddTagsForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/removeTags`
#[derive(Debug, Serialize)]
pub struct TorrentsRemoveTagsForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/tags`
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TorrentsTagsResponse {
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/createTags`
#[derive(Debug, Serialize)]
pub struct TorrentsCreateTagsForm {
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/deleteTags`
#[derive(Debug, Serialize)]
pub struct TorrentsDeleteTagsForm {
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/setAutoManagement`
#[derive(Debug, Default, Serialize)]
pub struct TorrentsSetAutoManagementForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub enable: bool,
}

/// # `/api/v2/torrents/toggleSequentialDownload`
#[derive(Debug, Serialize)]
pub struct TorrentsToggleSequentialDownloadForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/toggleFirstLastPiecePrio`
#[derive(Debug, Serialize)]
pub struct TorrentsToggleFirstLastPiecePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/setForceStart`
#[derive(Debug, Default, Serialize)]
pub struct TorrentsSetForceStartForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub value: bool,
}

/// # `/api/v2/torrents/setSuperSeeding`
#[derive(Debug, Default, Serialize)]
pub struct TorrentsSetSuperSeedingForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub value: bool,
}

/// # `/api/v2/torrents/renameFile`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsRenameFileForm {
    pub hash: String,
    pub old_path: String,
    pub new_path: String,
}

/// # `/api/v2/torrents/renameFolder`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentsRenameFolderForm {
    pub hash: String,
    pub old_path: String,
    pub new_path: String,
}

/// # `api/v2/search/start`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStartForm {
    pub pattern: String,
    pub plugins: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SearchStartResponse {
    pub id: u64,
}

/// # `/api/v2/search/stop`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStopForm {
    pub id: u64,
}

/// # `/api/v2/search/status`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStatusQuery {
    pub id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct SearchStatusResponseItem {
    pub id: u64,
    pub status: String,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SearchStatusResponse {
    pub jobs: Vec<SearchStatusResponseItem>,
}

/// # `/api/v2/search/results`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsQuery {
    pub id: u64,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultResponseItem {
    pub description_link: String,
    pub file_name: String,
    pub file_size: f64,
    pub file_url: String,
    pub leechers: u64,
    pub seeders: u64,
    pub site_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultsResponse {
    pub results: Vec<SearchResultResponseItem>,
    pub status: String,
    pub total: u64,
}

/// # `/api/v2/search/delete`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchDeleteForm {
    pub id: u64,
}

/// ### NOTE:
/// this custom serializer module is written to solve the problem,
/// that the serde_urlencoded cannot deserialize the nested struct,
/// which is the default dependency of the reqwest
mod preferences_serialize {

    use serde::{self, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, s: S) -> Result<S::Ok, S::Error>
    where
        T: ?Sized + Serialize,
        S: Serializer,
    {
        match serde_json::to_string(value) {
            Ok(json) => s.serialize_str(&json),
            Err(_) => Err(serde::ser::Error::custom("Failed to serialize &T to json")),
        }
    }
}

/// module to serialize & deserialize between `Vec<String>` and String with vertical bar `|`
mod string_saperated_with_vertical_bar {

    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join("|");
        s.serialize_str(&string_line)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_line = String::deserialize(deserializer)?;
        let string_items: Vec<String> = string_line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        Ok(string_items)
    }
}

/// module to serialize & deserialize between `Vec<String>` and String with `\n`
mod string_saperated_with_backslash_n {

    use serde::Serializer;

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join("\n");
        s.serialize_str(&string_line)
    }
}

/// module to serialize & deserialize between `Vec<String>` and String with `,`
mod string_saperated_with_comma {

    use serde::Serializer;

    pub fn serialize<S>(string_items: &[String], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_line = string_items.join(",");
        s.serialize_str(&string_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    fn read_json_file<P: AsRef<Path>>(path: P) -> String {
        let mut file = BufReader::new(File::open(path).unwrap());
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_deserialize_preferences_response() {
        let s = read_json_file("./tests/PreferencesResponse.json");
        let _p: AppPreferences = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_deserialize_maindata_response() {
        let s = read_json_file("./tests/MaindataResponse.json");
        let _p: SyncMaindataResponse = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_serialize_deserialize_hashes() {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct TestHash {
            #[serde(with = "string_saperated_with_vertical_bar")]
            hashes: Vec<String>,
        }

        let serialized = r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6|a585051959d4e06e71da2f4306547a08348e5d34"}"#;

        let deserialized = TestHash {
            hashes: vec![
                "7e2fc0391f2d855affed3b0545927bddd5189bc6".to_string(),
                "a585051959d4e06e71da2f4306547a08348e5d34".to_string(),
            ],
        };
        let de: TestHash = serde_json::from_str(serialized).unwrap();
        assert_eq!(de, deserialized);

        let se = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(se, serialized);
    }

    #[test]
    fn test_set_share_limits_form() {
        let limit = RatioLimit::Limit { ratio_limit: 5.5 };
        let mut limit_form = TorrentsSetShareLimitsForm {
            hashes: vec!["7e2fc0391f2d855affed3b0545927bddd5189bc6".to_string()],
            ratio_limit: limit,
            seeding_time_limit: 0,
        };

        let s = serde_json::to_string(&limit_form).unwrap();
        assert_eq!(
            s,
            r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6","ratioLimit":5.5,"seedingTimeLimit":0}"#
        );

        let special = RatioLimit::Special { ratio_limit: -1 };
        limit_form.ratio_limit = special;
        let s = serde_json::to_string(&limit_form).unwrap();
        assert_eq!(
            s,
            r#"{"hashes":"7e2fc0391f2d855affed3b0545927bddd5189bc6","ratioLimit":-1,"seedingTimeLimit":0}"#
        );
    }
}
