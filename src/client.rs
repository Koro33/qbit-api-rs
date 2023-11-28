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

#[derive(Debug, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct QbitClient {
    pub host: Url,
    pub auth: Credential,
    pub client: Client,
    pub cookie_store: Arc<CookieStoreMutex>,
}

impl QbitClient {
    fn _try_new(host: &str, username: &str, password: &str) -> Result<Self, ClientError> {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .map_err(|e| ClientError::Initialize(e.to_string()))?;
        Ok(Self {
            host: Url::parse(host).map_err(|e| ClientError::Initialize(e.to_string()))?,
            auth: Credential {
                username: username.to_owned(),
                password: password.to_owned(),
            },
            client,
            cookie_store,
        })
    }
    pub fn new_with_user_pwd<U>(host: U, username: U, password: U) -> Result<Self, ClientError>
    where
        U: AsRef<str>,
    {
        Self::_try_new(host.as_ref(), username.as_ref(), password.as_ref())
    }

    pub fn new_from_env() -> Result<Self, ClientError> {
        use std::env::var;

        let host =
            var("QBIT_HOST").map_err(|e| ClientError::Initialize(format!("`QBIT_HOST` {}", e)))?;
        let username = var("QBIT_USERNAME")
            .map_err(|e| ClientError::Initialize(format!("`QBIT_USERNAME` {}", e)))?;
        let password = var("QBIT_PASSWORD")
            .map_err(|e| ClientError::Initialize(format!("`QBIT_PASSWORD` {}", e)))?;
        Self::_try_new(&host, &username, &password)
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
        let auth_form = types::AuthLoginForm {
            username: self.auth.username.clone(),
            password: self.auth.password.clone(),
        };
        let api_auth_login = api::AuthLogin { f: auth_form };

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
        f: &types::AppSetPreferencesForm,
    ) -> Result<String, ClientError> {
        let api_set_preferences = api::AppSetPreferences { f: f.to_owned() };
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
        q: &types::LogMainQuery,
    ) -> Result<Vec<types::LogMainResponseItem>, ClientError> {
        let api_logmain = api::LogMain { q: q.to_owned() };
        let de_resp = self._resp(&api_logmain).await?;

        Ok(de_resp)
    }

    pub async fn log_peers(
        &self,
        q: &types::LogPeersQuery,
    ) -> Result<Vec<types::LogPeersResponseItem>, ClientError> {
        let api_logpeers = api::LogPeers { q: q.to_owned() };
        let de_resp = self._resp(&api_logpeers).await?;

        Ok(de_resp)
    }

    pub async fn sync_maindata(
        &self,
        q: &types::SyncMaindataQuery,
    ) -> Result<types::SyncMaindataResponse, ClientError> {
        let api_maindata = api::Maindata { q: q.to_owned() };
        let de_resp = self._resp(&api_maindata).await?;

        Ok(de_resp)
    }

    pub async fn sync_torrent_peers(
        &self,
        q: &types::SyncTorrentPeersQuery,
    ) -> Result<types::SyncTorrentPeersResponse, ClientError> {
        let api_torrent_peers = api::TorrentPeers { q: q.to_owned() };
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

    pub async fn ban_peers<T>(&self, peers: &[T]) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let peers: Vec<String> = peers.iter().map(|p| p.as_ref().to_owned()).collect();

        let f = types::BanPeersForm { peers };
        let api_ban_peers = api::BanPeers { f };
        let s = self._resp(&api_ban_peers).await?;

        Ok(s)
    }

    pub async fn torrents_info(
        &self,
        q: &types::TorrentsInfoQuery,
    ) -> Result<types::TorrentsInfoResponse, ClientError> {
        let api_torrents_info = api::TorrentsInfo { q: q.to_owned() };
        let de_resp = self._resp(&api_torrents_info).await?;

        Ok(de_resp)
    }

    pub async fn torrents_properties(
        &self,
        hash: &str,
    ) -> Result<types::TorrentsPropertiesResponse, ClientError> {
        let q = types::TorrentsPropertiesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_properties = api::TorrentsProperties { q };
        let de_resp = self._resp(&api_torrents_properties).await?;

        Ok(de_resp)
    }

    pub async fn torrents_trackers(
        &self,
        hash: &str,
    ) -> Result<types::TorrentsTrackersResponse, ClientError> {
        let q = types::TorrentsTrackersQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_trackers = api::TorrentsTrackers { q };
        let de_resp = self._resp(&api_torrents_trackers).await?;

        Ok(de_resp)
    }

    pub async fn torrents_webseeds(
        &self,
        hash: &str,
    ) -> Result<types::TorrentsWebseedsResponse, ClientError> {
        let q = types::TorrentsWebseedsQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_webseeds = api::TorrentsWebseeds { q };
        let de_resp = self._resp(&api_torrents_webseeds).await?;

        Ok(de_resp)
    }

    pub async fn torrents_files(
        &self,
        hash: &str,
        indexes: Option<&[u64]>,
    ) -> Result<types::TorrentsFilesResponse, ClientError> {
        let indexes = indexes.map(|v| {
            v.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("|")
        });

        let q = types::TorrentsFilesQuery {
            hash: hash.to_owned(),
            indexes,
        };
        let api_torrents_files = api::TorrentsFiles { q };
        let de_resp = self._resp(&api_torrents_files).await?;

        Ok(de_resp)
    }

    pub async fn torrents_piece_states(
        &self,
        hash: &str,
    ) -> Result<types::TorrentsPieceStatesResponse, ClientError> {
        let q = types::TorrentsPieceStatesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_piece_states = api::TorrentsPieceStates { q };
        let de_resp = self._resp(&api_torrents_piece_states).await?;

        Ok(de_resp)
    }

    pub async fn torrents_piece_hashes(
        &self,
        hash: &str,
    ) -> Result<types::TorrentsPieceHashesResponse, ClientError> {
        let q = types::TorrentsPieceHashesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_piece_hashes = api::TorrentsPieceHashes { q };
        let de_resp = self._resp(&api_torrents_piece_hashes).await?;

        Ok(de_resp)
    }

    pub async fn torrents_pause<T>(&self, hashes: &[T]) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsPauseForm { hashes };
        let api_torrents_pause = api::TorrentsPause { f };
        let s = self._resp(&api_torrents_pause).await?;

        Ok(s)
    }

    pub async fn torrents_resume<T>(&self, hashes: &[T]) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsResumeForm { hashes };
        let api_torrents_resume = api::TorrentsResume { f };
        let s = self._resp(&api_torrents_resume).await?;

        Ok(s)
    }

    pub async fn torrents_delete<T>(
        &self,
        hashes: &[T],
        delete_files: bool,
    ) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsDeleteForm {
            hashes,
            delete_files,
        };
        let api_torrents_delete = api::TorrentsDelete { f };
        let s = self._resp(&api_torrents_delete).await?;

        Ok(s)
    }

    pub async fn torrents_recheck<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsRecheckForm { hashes };
        let api_torrents_recheck = api::TorrentsRecheck { f };
        let s = self._resp(&api_torrents_recheck).await?;

        Ok(s)
    }

    pub async fn torrents_reannounce<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsReannounceForm { hashes };
        let api_torrents_reannounce = api::TorrentsReannounce { f };
        let s = self._resp(&api_torrents_reannounce).await?;

        Ok(s)
    }

    pub async fn torrents_add_by_url<U>(&self, urls: &[U]) -> Result<String, ClientError>
    where
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();
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

    pub async fn torrents_add_trackers<U>(
        &self,
        hash: &str,
        urls: &[U],
    ) -> Result<String, ClientError>
    where
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();

        let f = types::TorrentsAddTrackersForm {
            hash: hash.to_owned(),
            urls: urls.to_owned(),
        };
        let api_torrents_add_trackers = api::TorrentsAddTrackers { f };
        let s = self._resp(&api_torrents_add_trackers).await?;

        Ok(s)
    }

    pub async fn torrents_edit_tracker(
        &self,
        hash: &str,
        orig_url: &str,
        new_url: &str,
    ) -> Result<String, ClientError> {
        let f = types::TorrentsEditTrackerForm {
            hash: hash.to_owned(),
            orig_url: orig_url.to_owned(),
            new_url: new_url.to_owned(),
        };
        let api_torrents_edit_tracker = api::TorrentsEditTracker { f };
        let s = self._resp(&api_torrents_edit_tracker).await?;

        Ok(s)
    }

    pub async fn torrents_remove_trackers<U>(
        &self,
        hash: &str,
        urls: &[U],
    ) -> Result<String, ClientError>
    where
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();

        let f = types::TorrentsRemoveTrackersForm {
            hash: hash.to_owned(),
            urls,
        };
        let api_torrents_remove_trackers = api::TorrentsRemoveTrackers { f };
        let s = self._resp(&api_torrents_remove_trackers).await?;

        Ok(s)
    }

    pub async fn torrents_add_peers<H, P>(
        &self,
        hashes: &[H],
        peers: &[P],
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
        P: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let peers: Vec<String> = peers.iter().map(|p| p.as_ref().to_owned()).collect();

        let f = types::TorrentsAddPeersForm { hashes, peers };
        let api_torrents_add_peers = api::TorrentsAddPeers { f };
        let s = self._resp(&api_torrents_add_peers).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_increase_prio<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsIncreasePrioForm { hashes };
        let api_torrents_increase_prio = api::TorrentsIncreasePrio { f };
        let s = self._resp(&api_torrents_increase_prio).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_decrease_prio<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsDecreasePrioForm { hashes };
        let api_torrents_decrease_prio = api::TorrentsDecreasePrio { f };
        let s = self._resp(&api_torrents_decrease_prio).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_top_prio<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsTopPrioForm { hashes };
        let api_torrents_top_prio = api::TorrentsTopPrio { f };
        let s = self._resp(&api_torrents_top_prio).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_bottom_prio<H>(&self, hashes: &[H]) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsBottomPrioForm { hashes };
        let api_torrents_bottom_prio = api::TorrentsBottomPrio { f };
        let s = self._resp(&api_torrents_bottom_prio).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_download_limit<H>(
        &self,
        hashes: &[H],
    ) -> Result<types::TorrentsDownloadLimitResponse, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsDownloadLimitForm { hashes };
        let api_torrents_download_limit = api::TorrentsDownloadLimit { f };
        let de_resp = self._resp(&api_torrents_download_limit).await.unwrap();

        Ok(de_resp)
    }

    pub async fn torrents_set_download_limit<H>(
        &self,
        hashes: &[H],
        limit: u64,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetDownloadLimitForm { hashes, limit };
        let api_torrents_set_download_limit = api::TorrentsSetDownloadLimit { f };
        let s = self._resp(&api_torrents_set_download_limit).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_set_share_limits<H>(
        &self,
        hashes: &[H],
        ratio_limit: types::RatioLimit,
        seeding_time_limit: i64,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetShareLimitsForm {
            hashes,
            ratio_limit,
            seeding_time_limit,
        };
        let api_torrents_set_share_limits = api::TorrentsSetShareLimits { f };
        let s = self._resp(&api_torrents_set_share_limits).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_upload_limit<H>(
        &self,
        hashes: &[H],
    ) -> Result<types::TorrentsUploadLimitResponse, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsUploadLimitForm { hashes };
        let api_torrents_upload_limit = api::TorrentsUploadLimit { f };
        let de_resp = self._resp(&api_torrents_upload_limit).await.unwrap();

        Ok(de_resp)
    }

    pub async fn torrents_set_upload_limit<H>(
        &self,
        hashes: &[H],
        limit: u64,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetUploadLimitForm { hashes, limit };
        let api_torrents_set_upload_limit = api::TorrentsSetUploadLimit { f };
        let s = self._resp(&api_torrents_set_upload_limit).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_set_location<H, L>(
        &self,
        hashes: &[H],
        location: L,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
        L: AsRef<Path>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetLocationForm {
            hashes,
            location: location.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_set_location = api::TorrentsSetLocation { f };
        let s = self._resp(&api_torrents_set_location).await.unwrap();

        Ok(s)
    }

    pub async fn torernts_rename(&self, hash: &str, name: &str) -> Result<String, ClientError> {
        let f = types::TorrentsRenameForm {
            hash: hash.to_owned(),
            name: name.to_owned(),
        };
        let api_torrents_rename = api::TorrentsRename { f };
        let s = self._resp(&api_torrents_rename).await.unwrap();

        Ok(s)
    }

    pub async fn torernts_set_category<H>(
        &self,
        hashes: &[H],
        category: &str,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetCategoryForm {
            hashes,
            category: category.to_owned(),
        };
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

    pub async fn torrents_create_category<P>(
        &self,
        category: &str,
        save_path: P,
    ) -> Result<String, ClientError>
    where
        P: AsRef<Path>,
    {
        let f = types::TorrentsCreateCategoryForm {
            category: category.to_owned(),
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_create_category = api::TorrentsCreateCategory { f };
        let s = self._resp(&api_torrents_create_category).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_edit_category<P>(
        &self,
        category: &str,
        save_path: P,
    ) -> Result<String, ClientError>
    where
        P: AsRef<Path>,
    {
        let f = types::TorrentsEditCategoryForm {
            category: category.to_owned(),
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_edit_category = api::TorrentsEditCategory { f };
        let s = self._resp(&api_torrents_edit_category).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_remove_categories<C>(
        &self,
        categories: &[C],
    ) -> Result<String, ClientError>
    where
        C: AsRef<str>,
    {
        let categories: Vec<String> = categories.iter().map(|c| c.as_ref().to_owned()).collect();

        let f = types::TorrentsRemoveCategoriesForm { categories };
        let api_torrents_remove_categories = api::TorrentsRemoveCategories { f };
        let s = self._resp(&api_torrents_remove_categories).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_add_tags<H, T>(
        &self,
        hashes: &[H],
        tags: &[T],
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::TorrentsAddTagsForm { hashes, tags };
        let api_torrents_add_tags = api::TorrentsAddTags { f };
        let s = self._resp(&api_torrents_add_tags).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_remove_tags<H, T>(
        &self,
        hashes: &[H],
        tags: &[T],
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

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

    pub async fn torrens_create_tags<T>(&self, tags: &[T]) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::TorrentsCreateTagsForm { tags };
        let api_torrents_create_tags = api::TorrentsCreateTags { f };
        let s = self._resp(&api_torrents_create_tags).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_delete_tags<T>(&self, tags: &[T]) -> Result<String, ClientError>
    where
        T: AsRef<str>,
    {
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::TorrentsDeleteTagsForm { tags };
        let api_torrents_delete_tags = api::TorrentsDeleteTags { f };
        let s = self._resp(&api_torrents_delete_tags).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_set_auto_management<H>(
        &self,
        hashes: &[H],
        enable: bool,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetAutoManagementForm { hashes, enable };
        let api_torrents_set_automanagement = api::TorrentsSetAutoManagement { f };
        let s = self._resp(&api_torrents_set_automanagement).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_toggle_sequential_download<H>(
        &self,
        hashes: &[H],
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsToggleSequentialDownloadForm { hashes };
        let api_torrents_toggle_sequential_download = api::TorrentsToggleSequentialDownload { f };
        let s = self
            ._resp(&api_torrents_toggle_sequential_download)
            .await
            .unwrap();

        Ok(s)
    }

    pub async fn torrents_toggle_first_last_piece_prio<H>(
        &self,
        hashes: &[H],
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsToggleFirstLastPiecePrioForm { hashes };
        let api_torrents_toggle_first_last_piece_prio = api::TorrentsToggleFirstLastPiecePrio { f };
        let s = self
            ._resp(&api_torrents_toggle_first_last_piece_prio)
            .await
            .unwrap();

        Ok(s)
    }

    pub async fn torrents_set_force_start<H>(
        &self,
        hashes: &[H],
        value: bool,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetForceStartForm { hashes, value };
        let api_torrents_set_force_start = api::TorrentsSetForceStart { f };
        let s = self._resp(&api_torrents_set_force_start).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_set_super_seeding<H>(
        &self,
        hashes: &[H],
        value: bool,
    ) -> Result<String, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::TorrentsSetSuperSeedingForm { hashes, value };
        let api_torrents_set_super_seeding = api::TorrentsSetSuperSeeding { f };
        let s = self._resp(&api_torrents_set_super_seeding).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_rename_file<P>(
        &self,
        hash: &str,
        old_path: P,
        new_path: P,
    ) -> Result<String, ClientError>
    where
        P: AsRef<Path>,
    {
        let f = types::TorrentsRenameFileForm {
            hash: hash.to_owned(),
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_file = api::TorrentsRenameFile { f };
        let s = self._resp(&api_torrents_rename_file).await.unwrap();

        Ok(s)
    }

    pub async fn torrents_rename_folder<P>(
        &self,
        hash: &str,
        old_path: P,
        new_path: P,
    ) -> Result<String, ClientError>
    where
        P: AsRef<Path>,
    {
        let f = types::TorrentsRenameFolderForm {
            hash: hash.to_owned(),
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_folder = api::TorrentsRenameFolder { f };
        let s = self._resp(&api_torrents_rename_folder).await.unwrap();

        Ok(s)
    }

    pub async fn search_start(
        &self,
        pattern: &str,
        plugins: &str,
        category: &str,
    ) -> Result<types::SearchStartResponse, ClientError> {
        let f = types::SearchStartForm {
            pattern: pattern.to_owned(),
            plugins: plugins.to_owned(),
            category: category.to_owned(),
        };
        let api_search_start = api::SearchStart { f };
        let de_resp = self._resp(&api_search_start).await.unwrap();

        Ok(de_resp)
    }

    pub async fn search_stop(&self, id: u64) -> Result<String, ClientError> {
        let f = types::SearchStopForm { id };
        let api_search_stop = api::SearchStop { f };
        let s = self._resp(&api_search_stop).await.unwrap();

        Ok(s)
    }

    pub async fn search_status(
        &self,
        id: Option<u64>,
    ) -> Result<types::SearchStatusResponse, ClientError> {
        let q = types::SearchStatusQuery { id };
        let api_search_status = api::SearchStatus { q };
        let de_resp = self._resp(&api_search_status).await.unwrap();

        Ok(de_resp)
    }

    pub async fn search_results(
        &self,
        id: u64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<types::SearchResultsResponse, ClientError> {
        let q = types::SearchResultsQuery { id, limit, offset };
        let api_search_results = api::SearchResults { q };
        let de_resp = self._resp(&api_search_results).await.unwrap();

        Ok(de_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;

    async fn login() -> QbitClient {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let qbit = QbitClient::new_with_user_pwd("http://192.168.0.11:8080", "admin", "adminadmin")
            .unwrap();
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
}
