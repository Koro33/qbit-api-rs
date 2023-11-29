use serde::{self, Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashMap;
use super::preferences_serialize;

/// # `/api/v2/app/buildInfo`
#[derive(Debug, Clone, Deserialize)]
pub struct BuildInfoResponse {
    pub qt: String,
    pub libtorrent: String,
    pub boost: String,
    pub openssl: String,
    pub bitness: u32,
}

/// # `/api/v2/app/preferences`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Preferences {
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetPreferencesForm {
    #[serde(with = "preferences_serialize")]
    pub json: Preferences,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::scheduler_days`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
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
/// [`Preferences::encryption`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Encryption {
    PreferEncryption = 0,
    ForceEncryptionOn = 1,
    ForceEncryptionOff = 2,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::proxy_type`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
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
/// [`Preferences::dyndns_service`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum DynDnsService {
    DyDNS = 0,
    NOIP = 1,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::bittorrent_protocol`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MaxRatioAct {
    Pause = 0,
    Remove = 1,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::limit_utp_rate`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum BittorrentProtocol {
    TCPUTP = 0,
    TCP = 1,
    UTP = 2,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::upload_choking_algorithm`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::upload_slots_behavior`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    FixedSlots = 0,
    RateBased = 1,
}

/// # `/api/v2/app/preferences`
/// [`Preferences::utp_tcp_mixed_mode`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTCP = 0,
    PeerProportional = 1,
}