use super::{
    string_saperated_with_backslash_n, string_saperated_with_comma,
    string_saperated_with_vertical_bar,
};
use crate::error::TypesError;
use serde::{self, Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashMap;

/// # `/api/v2/torrents/info`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct InfoQuery {
    pub filter: Option<InfoFilter>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<InfoSort>,
    pub reverse: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<i64>,
    #[serde(serialize_with = "ser_option_hashes")]
    pub hashes: Option<Vec<String>>,
}

pub fn ser_option_hashes<S>(option_hashes: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match option_hashes {
        Some(hashes) => s.serialize_str(&hashes.join("|")),
        None => s.serialize_none(),
    }
}

/// # `/api/v2/torrents/info`
/// [`InfoQuery::filter`]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InfoFilter {
    #[default]
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
/// [`InfoQuery::sort`]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InfoSort {
    AddedOn,
    AmountLeft,
    AutoTmm,
    Category,
    Completed,
    CompletionOn,
    DlLimit,
    Dlspeed,
    Downloaded,
    DownloadedSession,
    Eta,
    FLPiecePrio,
    ForceStart,
    Hash,
    LastActivity,
    MagnetUri,
    MaxRatio,
    MaxSeedingTime,
    #[default]
    Name,
    NumComplete,
    NumIncomplete,
    NumLeechs,
    NumSeeds,
    Priority,
    Progress,
    Ratio,
    SavePath,
    SeedingTimeLimit,
    SeenComplete,
    SeqDl,
    Size,
    State,
    SuperSeeding,
    Tags,
    TimeActive,
    TotalSize,
    Tracker,
    UpLimit,
    Uploaded,
    UploadedSession,
    Upspeed,
}

/// # `/api/v2/torrents/info`
pub type InfoResponse = Vec<InfoResponseItem>;

/// # `/api/v2/torrents/info`
/// [`InfoResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct InfoResponseItem {
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
    pub state: InfoState,
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
/// [`InfoResponseItem::state`]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InfoState {
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct PropertiesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/properties`
#[derive(Debug, Clone, Deserialize)]
pub struct PropertiesResponse {
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct TrackersQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/trackers`
pub type TrackersResponse = Vec<TrackersResponseItem>;

/// # `/api/v2/torrents/trackers`
/// [`TrackersResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct TrackersResponseItem {
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
/// [`TrackersResponseItem::status`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TrackerStatus {
    Disabled = 0,
    NotContacted = 1,
    Working = 2,
    Updating = 3,
    NotWorking = 4,
}

/// # `/api/v2/torrents/webseeds`
#[derive(Debug, Clone, Default, Serialize)]
pub struct WebseedsQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/webseeds`
pub type WebseedsResponse = Vec<WebseedsResponseItem>;

/// # `/api/v2/torrents/webseeds`
/// [`WebseedsResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct WebseedsResponseItem {
    pub url: String,
}

/// # `/api/v2/torrents/files`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct FilesQuery {
    pub hash: String,
    // like 0|1|2|3|2|2|2|2...
    pub indexes: Option<String>,
}

/// # `/api/v2/torrents/files`
pub type FilesResponse = Vec<FilesResponseItem>;

/// # `/api/v2/torrents/files`
/// [`FilesResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct FilesResponseItem {
    pub index: u64,
    pub name: String,
    pub size: u64,
    pub progress: f64,
    pub priority: FilesPriority,
    pub is_seed: Option<bool>,
    pub piece_range: Vec<u64>,
    pub availability: f64,
}

/// # `/api/v2/torrents/files`
/// [`FilesResponseItem::priority`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum FilesPriority {
    NotDownload = 0,
    Normal = 1,
    High = 6,
    Maximal = 7,
}

/// # `/api/v2/torrents/pieceStates`
#[derive(Debug, Clone, Default, Serialize)]
pub struct PieceStatesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/pieceStates`
pub type PieceStatesResponse = Vec<PieceStates>;

/// # `/api/v2/torrents/pieceStates`
/// [`PieceStatesResponse`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum PieceStates {
    NotDownloaded = 0,
    Downloading = 1,
    Downloaded = 2,
}

/// # `/api/v2/torrents/pieceHashes`
#[derive(Debug, Clone, Default, Serialize)]
pub struct PieceHashesQuery {
    pub hash: String,
}

/// # `/api/v2/torrents/pieceHashes`
pub type PieceHashesResponse = Vec<String>;

/// # `/api/v2/torrents/pause`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct PauseForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl PauseForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/resume`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResumeForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl ResumeForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/delete`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(rename = "deleteFiles")]
    pub delete_files: bool,
}

impl DeleteForm {
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct RecheckForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl RecheckForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/reannounce`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReannounceForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl ReannounceForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/add`
#[derive(Debug, Clone, Default)]
pub struct AddMultipart {
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

impl AddMultipart {
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddTrackersForm {
    pub hash: String,
    #[serde(serialize_with = "string_saperated_with_backslash_n::serialize")]
    pub urls: Vec<String>,
}

/// # `/api/v2/torrents/editTracker`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTrackerForm {
    pub hash: String,
    pub orig_url: String,
    pub new_url: String,
}

/// # `/api/v2/torrents/removeTrackers`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveTrackersForm {
    pub hash: String,
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub urls: Vec<String>,
}

/// # `/api/v2/torrents/addPeers`
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddPeersForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub peers: Vec<String>,
}

/// # `/api/v2/torrents/increasePrio`
///
/// `warning`: it was described in the docs that parameter should be a query.
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct IncreasePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl IncreasePrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/decreasePrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct DecreasePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl DecreasePrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/topPrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct TopPrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl TopPrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/bottomPrio`
///
/// `warning`: it was described in the docs that parameter should be a query .
/// but it actually should be a form
#[derive(Debug, Clone, Default, Serialize)]
pub struct BottomPrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

impl BottomPrioForm {
    pub fn all(&mut self) {
        self.hashes.clear();
        self.hashes.push("all".to_string());
    }
}

/// # `/api/v2/torrents/filePrio`
// TODO: Implement

/// # `/api/v2/torrents/downloadLimit`
///
/// `Warning`: setting to "all" as docs described doesn't work
#[derive(Debug, Clone, Default, Serialize)]
pub struct DownloadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/downloadLimit`
/// if hash doesn't exist, the return value will be <hash, -1>
pub type DownloadLimitResponse = HashMap<String, i64>;

/// # `/api/v2/torrents/setDownloadLimit`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDownloadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    /// - `["all"]` -> all torrents
    pub hashes: Vec<String>,
    /// - `0` -> no limit
    pub limit: u64,
}

/// # `/api/v2/torrents/setShareLimits`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetShareLimitsForm {
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
/// [`SetShareLimitsForm::ratio_limit`]
///
/// define this type to solve the problem that if ratio_limit is a f64,
/// the value -2 and -1 will be serialized to -2.0 and -1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for RatioLimit {
    fn default() -> Self {
        Self::Special { ratio_limit: -2 }
    }
}

/// # `/api/v2/torrents/uploadLimit`
///
/// `Warning`: setting to "all" as docs described doesn't work
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/uploadLimit`
/// if hash doesn't exist, the return value will be <hash, -1>
pub type UploadLimitResponse = HashMap<String, i64>;

/// # `/api/v2/torrents/setUploadLimit`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetUploadLimitForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    /// - `["all"]` -> all torrents
    pub hashes: Vec<String>,
    /// - `0` -> no limit
    pub limit: u64,
}

/// # `/api/v2/torrents/setLocation`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetLocationForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub location: String,
}

/// # `/api/v2/torrents/rename`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RenameForm {
    pub hash: String,
    pub name: String,
}

/// # `/api/v2/torrents/setCategory`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCategoryForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub category: String,
}

/// # `/api/v2/torrents/categories`
pub type CategoriesResponse = HashMap<String, CategoriesDetails>;

/// # `/api/v2/torrents/categories`
/// [`CategoriesResponse`]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesDetails {
    pub name: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/createCategory`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryForm {
    pub category: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/editCategory`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditCategoryForm {
    pub category: String,
    pub save_path: String,
}

/// # `/api/v2/torrents/removeCategories`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveCategoriesForm {
    #[serde(serialize_with = "string_saperated_with_backslash_n::serialize")]
    pub categories: Vec<String>,
}

/// # `/api/v2/torrents/addTags`
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddTagsForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/removeTags`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveTagsForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/tags`
pub type TagsResponse = Vec<String>;

/// # `/api/v2/torrents/createTags`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTagsForm {
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/deleteTags`
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTagsForm {
    #[serde(serialize_with = "string_saperated_with_comma::serialize")]
    pub tags: Vec<String>,
}

/// # `/api/v2/torrents/setAutoManagement`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetAutoManagementForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub enable: bool,
}

/// # `/api/v2/torrents/toggleSequentialDownload`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ToggleSequentialDownloadForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/toggleFirstLastPiecePrio`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ToggleFirstLastPiecePrioForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
}

/// # `/api/v2/torrents/setForceStart`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetForceStartForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub value: bool,
}

/// # `/api/v2/torrents/setSuperSeeding`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetSuperSeedingForm {
    #[serde(serialize_with = "string_saperated_with_vertical_bar::serialize")]
    pub hashes: Vec<String>,
    pub value: bool,
}

/// # `/api/v2/torrents/renameFile`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFileForm {
    pub hash: String,
    pub old_path: String,
    pub new_path: String,
}

/// # `/api/v2/torrents/renameFolder`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFolderForm {
    pub hash: String,
    pub old_path: String,
    pub new_path: String,
}
