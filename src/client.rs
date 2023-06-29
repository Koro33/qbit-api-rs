use crate::error::ClientError;
use crate::{
    api::{self, Endpoint},
    types,
};
use log::debug;
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::{error::Error, io::prelude::*, path::Path, sync::Arc};
use url::Url;

#[derive(Debug)]
pub struct QbitClient {
    pub host: Url,
    pub auth: types::AuthLoginForm,
    pub client: Client,
    pub cookie_store: Arc<CookieStoreMutex>,
}

impl QbitClient {
    pub fn try_new(host: String, username: String, password: String) -> Result<Self, ClientError> {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()?;
        Ok(Self {
            host: Url::parse(&host)?,
            auth: types::AuthLoginForm { username, password },
            client,
            cookie_store,
        })
    }

    pub fn new(host: String, username: String, password: String) -> Self {
        Self::try_new(host, username, password).unwrap()
    }

    pub async fn _resp<E>(&self, endpoint: &E) -> Result<E::Response, ClientError>
    where
        E: Endpoint,
    {
        let url = self.host.join(&endpoint.relative_path())?;
        let mut request = self.client.request(endpoint.method(), url);

        // build Headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", self.host.to_string().parse()?);
        request = request.headers(headers);

        if let Some(query) = endpoint.query() {
            request = request.query(query);
        }
        if let Some(form) = endpoint.form() {
            request = request.form(form);
        }
        if let Some(multipart) = endpoint.multipart() {
            request = request.multipart(multipart);
        }
        debug!("request: {:?}", request);

        // send request
        let resp = request.send().await?;
        debug!("response: {:?}", resp);

        // check status code, return errors that defined in api
        if let Some(error) = endpoint.check_status(resp.status()) {
            return Err(error);
        }
        // deserialize response as string or type defined in api
        let de_resp = endpoint.de_response(resp).await?;
        Ok(de_resp)
    }

    pub async fn auth_login(&self) -> Result<String, ClientError> {
        let api_auth_login = api::AuthLogin {
            f: self.auth.clone(),
        };

        {
            let mut store = self.cookie_store.lock().unwrap();
            store.clear();
        }

        let s = self._resp(&api_auth_login).await?;
        Ok(s)
    }

    pub async fn auth_logout(&self) -> Result<String, ClientError> {
        let api_auth_logout = api::AuthLogout {};
        let s = self._resp(&api_auth_logout).await?;
        Ok(s)
    }

    pub async fn app_version(&self) -> Result<String, ClientError> {
        let api_app_version = api::AppVersion {};
        let s = self._resp(&api_app_version).await?;
        Ok(s)
    }

    pub async fn app_webapi_version(&self) -> Result<String, ClientError> {
        let api_app_webapi_version = api::AppWebApiVersion {};
        let s = self._resp(&api_app_webapi_version).await?;
        Ok(s)
    }

    pub async fn app_build_info(&self) -> Result<types::AppBuildInfoResponse, ClientError> {
        let api_build_info = api::AppBuildInfo {};
        let de_resp = self._resp(&api_build_info).await?;
        Ok(de_resp)
    }

    pub async fn app_preferences(&self) -> Result<types::AppPreferences, ClientError> {
        let api_app_preferences = api::AppPreferences {};
        let de_resp = self._resp(&api_app_preferences).await?;
        Ok(de_resp)
    }

    pub async fn app_set_preferences(
        &self,
        f: types::AppSetPreferencesForm,
    ) -> Result<String, ClientError> {
        let api_set_preferences = api::AppSetPreferences { f };
        let s = self._resp(&api_set_preferences).await?;
        Ok(s)
    }

    pub async fn app_default_save_path(&self) -> Result<String, ClientError> {
        let api_default_save_path = api::AppDefaultSavePath {};
        let s = self._resp(&api_default_save_path).await?;
        Ok(s)
    }

    pub async fn log_main(
        &self,
        q: types::LogMainQuery,
    ) -> Result<Vec<types::LogMainResponseItem>, ClientError> {
        let api_logmain = api::LogMain { q };
        let de_resp = self._resp(&api_logmain).await?;
        Ok(de_resp.data)
    }

    pub async fn log_peers(
        &self,
        q: types::LogPeersQuery,
    ) -> Result<Vec<types::LogPeersResponseItem>, ClientError> {
        let api_logpeers = api::LogPeers { q };
        let de_resp = self._resp(&api_logpeers).await?;
        Ok(de_resp.data)
    }

    pub async fn sync_maindata(
        &self,
        q: types::SyncMaindataQuery,
    ) -> Result<types::SyncMaindataResponse, ClientError> {
        let api_maindata = api::Maindata { q };
        let de_resp = self._resp(&api_maindata).await?;
        Ok(de_resp)
    }

    pub async fn sync_torrent_peers(
        &self,
        q: types::SyncTorrentPeersQuery,
    ) -> Result<types::SyncTorrentPeersResponse, ClientError> {
        let api_torrent_peers = api::TorrentPeers { q };
        let de_resp = self._resp(&api_torrent_peers).await?;
        Ok(de_resp)
    }

    pub async fn transfer_info(&self) -> Result<types::TransferInfoResponse, ClientError> {
        let api_transfer_info = api::TransferInfo {};
        let de_resp = self._resp(&api_transfer_info).await?;
        Ok(de_resp)
    }

    pub async fn speed_limits_mode(&self) -> Result<types::SpeedLimitsModeResponse, ClientError> {
        let api_speed_limits_mode = api::SpeedLimitsMode {};
        let de_resp = self._resp(&api_speed_limits_mode).await?;
        Ok(de_resp)
    }

    pub async fn toggle_speed_limits_mode(&self) -> Result<String, ClientError> {
        let api_toggle_speed_limits_mode = api::ToggleSpeedLimitsMode {};
        let s = self._resp(&api_toggle_speed_limits_mode).await?;
        Ok(s)
    }

    pub async fn download_limit(&self) -> Result<String, ClientError> {
        let api_download_limit = api::DownloadLimit {};
        let s = self._resp(&api_download_limit).await?;
        Ok(s)
    }

    pub async fn set_download_limit(&self, limit: u64) -> Result<String, ClientError> {
        let api_set_download_limit = api::SetDownloadLimit {
            f: types::SetDownloadLimitForm { limit },
        };
        let s = self._resp(&api_set_download_limit).await?;
        Ok(s)
    }

    pub async fn upload_limit(&self) -> Result<String, ClientError> {
        let api_upload_limit = api::UploadLimit {};
        let s = self._resp(&api_upload_limit).await?;
        Ok(s)
    }

    pub async fn set_upload_limit(&self, limit: u64) -> Result<String, ClientError> {
        let api_set_upload_limit = api::SetUploadLimit {
            f: types::SetUploadLimitForm { limit },
        };
        let s = self._resp(&api_set_upload_limit).await?;
        Ok(s)
    }

    pub async fn ban_peers(&self, peers: Vec<String>) -> Result<String, ClientError> {
        let f = types::BanPeersForm { peers };
        let api_ban_peers = api::BanPeers { f };
        let s = self._resp(&api_ban_peers).await?;
        Ok(s)
    }

    pub async fn torrents_info(
        &self,
        q: types::TorrentsInfoQuery,
    ) -> Result<types::TorrentsInfoResponse, ClientError> {
        let api_torrents_info = api::TorrentsInfo { q };
        let de_resp = self._resp(&api_torrents_info).await?;
        Ok(de_resp)
    }

    pub async fn torrents_properties(
        &self,
        hash: String,
    ) -> Result<types::TorrentsPropertiesResponse, ClientError> {
        let q = types::TorrentsPropertiesQuery { hash };
        let api_torrents_properties = api::TorrentsProperties { q };
        let de_resp = self._resp(&api_torrents_properties).await?;
        Ok(de_resp)
    }

    pub async fn torrents_trackers(
        &self,
        hash: String,
    ) -> Result<types::TorrentsTrackersResponse, ClientError> {
        let q = types::TorrentsTrackersQuery { hash };
        let api_torrents_trackers = api::TorrentsTrackers { q };
        let de_resp = self._resp(&api_torrents_trackers).await?;
        Ok(de_resp)
    }

    pub async fn torrents_webseeds(
        &self,
        hash: String,
    ) -> Result<types::TorrentsWebseedsResponse, ClientError> {
        let q = types::TorrentsWebseedsQuery { hash };
        let api_torrents_webseeds = api::TorrentsWebseeds { q };
        let de_resp = self._resp(&api_torrents_webseeds).await?;
        Ok(de_resp)
    }

    pub async fn torrents_files(
        &self,
        hash: String,
    ) -> Result<types::TorrentsFilesResponse, ClientError> {
        let q = types::TorrentsFilesQuery {
            hash,
            ..Default::default()
        };
        let api_torrents_files = api::TorrentsFiles { q };
        let de_resp = self._resp(&api_torrents_files).await?;
        Ok(de_resp)
    }

    pub async fn torrents_piece_states(
        &self,
        hash: String,
    ) -> Result<types::TorrentsPieceStatesResponse, ClientError> {
        let q = types::TorrentsPieceStatesQuery { hash };
        let api_torrents_piece_states = api::TorrentsPieceStates { q };
        let de_resp = self._resp(&api_torrents_piece_states).await?;
        Ok(de_resp)
    }

    pub async fn torrents_piece_hashes(
        &self,
        hash: String,
    ) -> Result<types::TorrentsPieceHashesResponse, ClientError> {
        let q = types::TorrentsPieceHashesQuery { hash };
        let api_torrents_piece_hashes = api::TorrentsPieceHashes { q };
        let de_resp = self._resp(&api_torrents_piece_hashes).await?;
        Ok(de_resp)
    }

    pub async fn torrents_pause(&self, hashes: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsPauseForm { hashes };
        let api_torrents_pause = api::TorrentsPause { f };
        let s = self._resp(&api_torrents_pause).await?;
        Ok(s)
    }

    pub async fn torrents_add_by_url<U>(&self, urls: &[U]) -> Result<String, ClientError>
    where
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_string()).collect();
        let ta = types::TorrentsAddMultipart {
            urls,
            torrents: vec![],
            ..Default::default()
        };
        let s = self.torrents_add(ta).await?;
        Ok(s)
    }

    pub async fn torrents_add_by_file<F>(&self, files: &[F]) -> Result<String, ClientError>
    where
        F: AsRef<Path>,
    {
        type VecOfNameAndContent = Vec<(String, Vec<u8>)>;
        let fc = |x: &F| -> Result<(String, Vec<u8>), Box<dyn Error>> {
            let mut f = std::fs::File::open(x.as_ref())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            Ok((
                x.as_ref()
                    .file_name()
                    .ok_or("no file name")?
                    .to_string_lossy()
                    .to_string(),
                buffer,
            ))
        };
        let files: Result<VecOfNameAndContent, Box<dyn Error>> = files.iter().map(fc).collect();
        let files = files.map_err(|_| ClientError::Other("".into()))?;
        let ta = types::TorrentsAddMultipart {
            urls: vec![],
            torrents: files,
            ..Default::default()
        };
        let s = self.torrents_add(ta).await?;
        Ok(s)
    }

    async fn torrents_add(&self, ta: types::TorrentsAddMultipart) -> Result<String, ClientError> {
        let api_torrents_add = api::TorrentsAdd { mp: ta };
        if api_torrents_add.multipart().is_none() {
            return Err(ClientError::InvalidMultipart("no valid multipart".into()));
        }
        let s = self._resp(&api_torrents_add).await?;
        Ok(s)
    }

    pub async fn torrents_add_trackers(
        &self,
        hash: String,
        urls: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsAddTrackersForm { hash, urls };
        let api_torrents_add_trackers = api::TorrentsAddTrackers { f };
        let s = self._resp(&api_torrents_add_trackers).await?;
        Ok(s)
    }

    pub async fn torrents_edit_tracker(
        &self,
        hash: String,
        orig_url: String,
        new_url: String,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsEditTrackerForm {
            hash,
            orig_url,
            new_url,
        };
        let api_torrents_edit_tracker = api::TorrentsEditTracker { f };
        let s = self._resp(&api_torrents_edit_tracker).await?;
        Ok(s)
    }

    pub async fn torrents_remove_trackers(
        &self,
        hash: String,
        urls: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsRemoveTrackersForm { hash, urls };
        let api_torrents_remove_trackers = api::TorrentsRemoveTrackers { f };
        let s = self._resp(&api_torrents_remove_trackers).await?;
        Ok(s)
    }

    pub async fn torrents_add_peers(
        &self,
        hashes: Vec<String>,
        peers: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsAddPeersForm { hashes, peers };
        let api_torrents_add_peers = api::TorrentsAddPeers { f };
        let s = self._resp(&api_torrents_add_peers).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_increase_prio(&self, hashes: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsIncreasePrioForm { hashes };
        let api_torrents_increase_prio = api::TorrentsIncreasePrio { f };
        let s = self._resp(&api_torrents_increase_prio).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_decrease_prio(&self, hashes: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsDecreasePrioForm { hashes };
        let api_torrents_decrease_prio = api::TorrentsDecreasePrio { f };
        let s = self._resp(&api_torrents_decrease_prio).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_top_prio(&self, hashes: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsTopPrioForm { hashes };
        let api_torrents_top_prio = api::TorrentsTopPrio { f };
        let s = self._resp(&api_torrents_top_prio).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_bottom_prio(&self, hashes: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsBottomPrioForm { hashes };
        let api_torrents_bottom_prio = api::TorrentsBottomPrio { f };
        let s = self._resp(&api_torrents_bottom_prio).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_download_limit(
        &self,
        hashes: Vec<String>,
    ) -> Result<types::TorrentsDownloadLimitResponse, ClientError> {
        let f = types::TorrentsDownloadLimitForm { hashes };
        let api_torrents_download_limit = api::TorrentsDownloadLimit { f };
        let de_resp = self._resp(&api_torrents_download_limit).await.unwrap();
        Ok(de_resp)
    }

    pub async fn torrents_set_download_limit(
        &self,
        hashes: Vec<String>,
        limit: u64,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetDownloadLimitForm { hashes, limit };
        let api_torrents_set_download_limit = api::TorrentsSetDownloadLimit { f };
        let s = self._resp(&api_torrents_set_download_limit).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_set_share_limits(
        &self,
        hashes: Vec<String>,
        ratio_limit: types::RatioLimit,
        seeding_time_limit: i64,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetShareLimitsForm {
            hashes,
            ratio_limit,
            seeding_time_limit,
        };
        let api_torrents_set_share_limits = api::TorrentsSetShareLimits { f };
        let s = self._resp(&api_torrents_set_share_limits).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_upload_limit(
        &self,
        hashes: Vec<String>,
    ) -> Result<types::TorrentsUploadLimitResponse, ClientError> {
        let f = types::TorrentsUploadLimitForm { hashes };
        let api_torrents_upload_limit = api::TorrentsUploadLimit { f };
        let de_resp = self._resp(&api_torrents_upload_limit).await.unwrap();
        Ok(de_resp)
    }

    pub async fn torrents_set_upload_limit(
        &self,
        hashes: Vec<String>,
        limit: u64,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetUploadLimitForm { hashes, limit };
        let api_torrents_set_upload_limit = api::TorrentsSetUploadLimit { f };
        let s = self._resp(&api_torrents_set_upload_limit).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_set_location<T>(
        &self,
        hashes: Vec<String>,
        location: T,
    ) -> Result<String, ClientError>
    where
        T: AsRef<Path>,
    {
        let f = types::TorrentsSetLocationForm {
            hashes,
            location: location.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_set_location = api::TorrentsSetLocation { f };
        let s = self._resp(&api_torrents_set_location).await.unwrap();
        Ok(s)
    }

    pub async fn torernts_rename(&self, hash: String, name: String) -> Result<String, ClientError> {
        let f = types::TorrentsRenameForm { hash, name };
        let api_torrents_rename = api::TorrentsRename { f };
        let s = self._resp(&api_torrents_rename).await.unwrap();
        Ok(s)
    }

    pub async fn torernts_set_category(
        &self,
        hashes: Vec<String>,
        category: String,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetCategoryForm { hashes, category };
        let api_torrents_set_category = api::TorrentsSetCategory { f };
        let s = self._resp(&api_torrents_set_category).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_categories(
        &self,
    ) -> Result<types::TorrentsCategoriesResponse, ClientError> {
        let api_torrents_categories = api::TorrentsCategories {};
        let de_resp = self._resp(&api_torrents_categories).await.unwrap();
        Ok(de_resp)
    }

    pub async fn torrents_create_category<T>(
        &self,
        category: String,
        save_path: T,
    ) -> Result<String, ClientError>
    where
        T: AsRef<Path>,
    {
        let f = types::TorrentsCreateCategoryForm {
            category,
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_create_category = api::TorrentsCreateCategory { f };
        let s = self._resp(&api_torrents_create_category).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_edit_category<T>(
        &self,
        category: String,
        save_path: T,
    ) -> Result<String, ClientError>
    where
        T: AsRef<Path>,
    {
        let f = types::TorrentsEditCategoryForm {
            category,
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_edit_category = api::TorrentsEditCategory { f };
        let s = self._resp(&api_torrents_edit_category).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_remove_categories(
        &self,
        categories: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsRemoveCategoriesForm { categories };
        let api_torrents_remove_categories = api::TorrentsRemoveCategories { f };
        let s = self._resp(&api_torrents_remove_categories).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_add_tags(
        &self,
        hashes: Vec<String>,
        tags: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsAddTagsForm { hashes, tags };
        let api_torrents_add_tags = api::TorrentsAddTags { f };
        let s = self._resp(&api_torrents_add_tags).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_remove_tags(
        &self,
        hashes: Vec<String>,
        tags: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsRemoveTagsForm { hashes, tags };
        let api_torrents_remove_tags = api::TorrentsRemoveTags { f };
        let s = self._resp(&api_torrents_remove_tags).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_tags(&self) -> Result<types::TorrentsTagsResponse, ClientError> {
        let api_torrents_tags = api::TorrentsTags {};
        let de_resp = self._resp(&api_torrents_tags).await.unwrap();
        Ok(de_resp)
    }

    pub async fn torrens_create_tags(&self, tags: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsCreateTagsForm { tags };
        let api_torrents_create_tags = api::TorrentsCreateTags { f };
        let s = self._resp(&api_torrents_create_tags).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_delete_tags(&self, tags: Vec<String>) -> Result<String, ClientError> {
        let f = types::TorrentsDeleteTagsForm { tags };
        let api_torrents_delete_tags = api::TorrentsDeleteTags { f };
        let s = self._resp(&api_torrents_delete_tags).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_set_auto_management(
        &self,
        hashes: Vec<String>,
        enable: bool,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetAutoManagementForm { hashes, enable };
        let api_torrents_set_automanagement = api::TorrentsSetAutoManagement { f };
        let s = self._resp(&api_torrents_set_automanagement).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_toggle_sequential_download(
        &self,
        hashes: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsToggleSequentialDownloadForm { hashes };
        let api_torrents_toggle_sequential_download = api::TorrentsToggleSequentialDownload { f };
        let s = self
            ._resp(&api_torrents_toggle_sequential_download)
            .await
            .unwrap();
        Ok(s)
    }

    pub async fn torrents_toggle_first_last_piece_prio(
        &self,
        hashes: Vec<String>,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsToggleFirstLastPiecePrioForm { hashes };
        let api_torrents_toggle_first_last_piece_prio = api::TorrentsToggleFirstLastPiecePrio { f };
        let s = self
            ._resp(&api_torrents_toggle_first_last_piece_prio)
            .await
            .unwrap();
        Ok(s)
    }

    pub async fn torrents_set_force_start(
        &self,
        hashes: Vec<String>,
        value: bool,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetForceStartForm { hashes, value };
        let api_torrents_set_force_start = api::TorrentsSetForceStart { f };
        let s = self._resp(&api_torrents_set_force_start).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_set_super_seeding(
        &self,
        hashes: Vec<String>,
        value: bool,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsSetSuperSeedingForm { hashes, value };
        let api_torrents_set_super_seeding = api::TorrentsSetSuperSeeding { f };
        let s = self._resp(&api_torrents_set_super_seeding).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_rename_file<T>(
        &self,
        hash: String,
        old_path: T,
        new_path: T,
    ) -> Result<String, ClientError>
    where
        T: AsRef<Path>,
    {
        let f = types::TorrentsRenameFileForm {
            hash,
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_file = api::TorrentsRenameFile { f };
        let s = self._resp(&api_torrents_rename_file).await.unwrap();
        Ok(s)
    }

    pub async fn torrents_rename_folder<T>(
        &self,
        hash: String,
        old_path: T,
        new_path: T,
    ) -> Result<String, ClientError>
    where
        T: AsRef<Path>,
    {
        let f = types::TorrentsRenameFolderForm {
            hash,
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_folder = api::TorrentsRenameFolder { f };
        let s = self._resp(&api_torrents_rename_folder).await.unwrap();
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;

    async fn login() -> QbitClient {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let qbit = QbitClient::new(
            "http://192.168.0.11:8080".to_string(),
            "admin".to_string(),
            "adminadmin".to_string(),
        );
        qbit.auth_login().await.unwrap();
        qbit
    }

    static LOGIN: OnceCell<QbitClient> = OnceCell::const_new();

    #[tokio::test]
    pub async fn test_version() {
        let client = LOGIN.get_or_init(login).await;
        let v = client.app_version().await.unwrap();
        debug!("version: {}", v);
    }

    #[tokio::test]
    pub async fn test_webapi_version() {
        let client = LOGIN.get_or_init(login).await;
        let v = client.app_webapi_version().await.unwrap();
        debug!("webapi_version: {}", v);
    }

    #[tokio::test]
    pub async fn test_build_info() {
        let client = LOGIN.get_or_init(login).await;
        let buildinfo = client.app_build_info().await.unwrap();
        debug!("buildinfo: {:?}", buildinfo);
    }
    #[tokio::test]
    pub async fn test_preferences() {
        let client = LOGIN.get_or_init(login).await;
        let p = client.app_preferences().await.unwrap();
        debug!("preferences: {:?}", p);
    }

    #[tokio::test]
    pub async fn test_set_preferences() {
        let client = LOGIN.get_or_init(login).await;
        let p = client
            .app_set_preferences(types::AppSetPreferencesForm {
                json: types::AppPreferences {
                    max_ratio_enabled: Some(true),
                    max_ratio: Some(2.0),
                    ..Default::default()
                },
            })
            .await
            .unwrap();
        debug!("set_preferences: {:?}", p);
    }

    #[tokio::test]
    pub async fn test_default_save_path() {
        let client = LOGIN.get_or_init(login).await;
        let p = client.app_default_save_path().await.unwrap();
        debug!("default_save_path: {}", p);
    }

    #[tokio::test]
    pub async fn test_log_main() {
        let client = LOGIN.get_or_init(login).await;
        let logmain = client
            .log_main(types::LogMainQuery {
                normal: false,
                info: false,
                ..Default::default()
            })
            .await
            .unwrap();
        debug!("logmain: {:?}", logmain);
    }

    #[tokio::test]
    pub async fn test_log_peers() {
        let client = LOGIN.get_or_init(login).await;
        let logpeers = client
            .log_peers(types::LogPeersQuery { last_known_id: -1 })
            .await
            .unwrap();
        debug!("logpeers: {:?}", logpeers);
    }

    #[tokio::test]
    pub async fn test_maindata() {
        let client = LOGIN.get_or_init(login).await;
        let maindata = client
            .sync_maindata(types::SyncMaindataQuery {
                ..Default::default()
            })
            .await
            .unwrap();
        debug!("maindata: {:?}", maindata);
    }

    #[tokio::test]
    pub async fn test_sync_torrent_peers() {
        let client = LOGIN.get_or_init(login).await;
        let peers = client
            .sync_torrent_peers(types::SyncTorrentPeersQuery {
                hash: "4bbdc771d411859f7e507740ed83fe675acac12b".to_string(),
                ..Default::default()
            })
            .await
            .unwrap_or_default();
        assert_eq!(
            peers,
            types::SyncTorrentPeersResponse {
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    pub async fn test_transfer_info() {
        let client = LOGIN.get_or_init(login).await;
        let info = client.transfer_info().await.unwrap();
        debug!("transfer_info: {:?}", info);
    }

    #[tokio::test]
    pub async fn test_speed_limits_mode() {
        let client = LOGIN.get_or_init(login).await;
        let mode = client.speed_limits_mode().await.unwrap();
        debug!("speed_limits_mode: {:?}", mode);
    }

    #[tokio::test]
    pub async fn test_toggle_speed_limits_mode() {
        let client = LOGIN.get_or_init(login).await;
        let status = client.toggle_speed_limits_mode().await.unwrap();
        debug!("toggle_speed_limits_mode: {:?}", status);
    }

    #[tokio::test]
    pub async fn test_download_limit() {
        let client = LOGIN.get_or_init(login).await;
        let speed = client.download_limit().await.unwrap();
        debug!("download_limit: {:?}", speed);
    }

    #[tokio::test]
    pub async fn test_set_download_limit() {
        let client = LOGIN.get_or_init(login).await;
        let status = client.set_download_limit(25 * 1024 * 1024).await.unwrap();
        debug!("set_download_limit: {:?}", status);
    }

    #[tokio::test]
    pub async fn test_upload_limit() {
        let client = LOGIN.get_or_init(login).await;
        let speed = client.upload_limit().await.unwrap();
        debug!("upload_limit: {:?}", speed);
    }

    #[tokio::test]
    pub async fn test_set_upload_limit() {
        let client = LOGIN.get_or_init(login).await;
        let status = client.set_upload_limit(2 * 1024 * 1024).await.unwrap();
        debug!("set_upload_limit: {:?}", status);
    }

    #[tokio::test]
    pub async fn test_ban_peers() {
        let client = LOGIN.get_or_init(login).await;
        let peers = vec![
            "123.123.123.123:123".to_string(),
            "12.12.12.12:12".to_string(),
        ];
        let status = client.ban_peers(peers).await.unwrap();
        debug!("ban_peers: {:?}", status);
    }

    #[tokio::test]
    pub async fn test_torrents_info() {
        let client = LOGIN.get_or_init(login).await;
        let info = client
            .torrents_info(types::TorrentsInfoQuery {
                hashes: Some(types::Hashes(vec![
                    "f3f662af354d8e64a2fe488c532e463be2992153".to_string(),
                    "bbf047dfc085498130780f9e4e5348d897196b85".to_string(),
                ])),
                ..Default::default()
            })
            .await
            .unwrap();
        debug!("torrents_info: {:?}", info);
    }

    #[tokio::test]
    pub async fn test_torrents_properties() {
        let client = LOGIN.get_or_init(login).await;
        let properties = client
            .torrents_properties("f3f662af354d8e64a2fe488c532e463be2992153".to_string())
            .await
            .unwrap();
        debug!("torrents_properties: {:?}", properties);
    }

    #[tokio::test]
    pub async fn test_torrents_trackers() {
        let client = LOGIN.get_or_init(login).await;
        let trackers = client
            .torrents_trackers("f3f662af354d8e64a2fe488c532e463be2992153".to_string())
            .await
            .unwrap();
        debug!("torrents_trackers: {:?}", trackers);
    }

    #[tokio::test]
    pub async fn test_torrents_webseeds() {
        let client = LOGIN.get_or_init(login).await;
        let webseeds = client
            .torrents_webseeds("f3f662af354d8e64a2fe488c532e463be2992153".to_string())
            .await
            .unwrap();
        debug!("torrents_webseeds: {:?}", webseeds);
    }

    #[tokio::test]
    pub async fn test_torrents_files() {
        let client = LOGIN.get_or_init(login).await;
        let files = client
            .torrents_files("e23f36256da9474c99073c98d273461633efc970".to_string())
            .await
            .unwrap();
        debug!("torrents_files: {:?}", files);
    }

    #[tokio::test]
    pub async fn test_torrents_piece_states() {
        let client = LOGIN.get_or_init(login).await;
        let states = client
            .torrents_piece_states("f3f662af354d8e64a2fe488c532e463be2992153".to_string())
            .await
            .unwrap();
        debug!("torrents_piece_states: {:?}", states);
    }

    #[tokio::test]
    pub async fn test_torrents_piece_hashes() {
        let client = LOGIN.get_or_init(login).await;
        let hashes = client
            .torrents_piece_hashes("f3f662af354d8e64a2fe488c532e463be2992153".to_string())
            .await
            .unwrap();
        debug!("torrents_piece_hashes: {:?}", hashes);
    }

    #[tokio::test]
    pub async fn test_torrents_pause() {
        let client = LOGIN.get_or_init(login).await;
        let status = client
            .torrents_pause(vec!["f3f662af354d8e64a2fe488c532e463be2992153".to_string()])
            .await
            .unwrap();
        debug!("torrents_pause: {:?}", status);
    }

    #[tokio::test]
    pub async fn test_torrents_add_trackers() {
        let client = LOGIN.get_or_init(login).await;
        let status = client
            .torrents_add_trackers(
                "2d4d408abca54ad478fc65544ade5849eef703a5".to_string(),
                vec![
                    "123.123.123.123:123".to_string(),
                    "12.12.12.12:12".to_string(),
                ],
            )
            .await
            .unwrap();
        debug!("torrents_add_trackers: {:?}", status);
    }
}
