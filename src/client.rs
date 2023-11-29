use crate::error::ClientError;
use crate::{
    api::{self, Endpoint},
    types,
};
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
        log::debug!("request: {:?}", request);

        // send request
        let resp = request.send().await?;
        log::debug!("response: {:?}", resp);

        // check status code, return errors that defined in api
        if let Some(error) = endpoint.check_status(resp.status()) {
            return Err(error);
        }
        // deserialize response as string or type defined in api
        let de_resp = endpoint.de_response(resp).await?;
        Ok(de_resp)
    }

    /// # `/api/v2/auth/login`
    pub async fn auth_login(&self) -> Result<(), ClientError> {
        let auth_form = types::auth::LoginForm {
            username: self.auth.username.clone(),
            password: self.auth.password.clone(),
        };
        let api_auth_login = api::auth::Login { f: auth_form };

        {
            let mut store = self.cookie_store.lock().unwrap();
            store.clear();
        }

        let _s = self._resp(&api_auth_login).await?;

        Ok(())
    }

    /// # `/api/v2/auth/logout`
    pub async fn auth_logout(&self) -> Result<(), ClientError> {
        let api_auth_logout = api::auth::Logout {};
        let _s = self._resp(&api_auth_logout).await?;

        Ok(())
    }

    /// # `/api/v2/app/version`
    pub async fn app_version(&self) -> Result<String, ClientError> {
        let api_app_version = api::app::Version {};
        let s = self._resp(&api_app_version).await?;

        Ok(s)
    }

    /// # `/api/v2/app/webapiVersion`
    pub async fn app_webapi_version(&self) -> Result<String, ClientError> {
        let api_app_webapi_version = api::app::WebApiVersion {};
        let s = self._resp(&api_app_webapi_version).await?;

        Ok(s)
    }

    /// # `/api/v2/app/buildInfo`
    pub async fn app_build_info(&self) -> Result<types::app::BuildInfoResponse, ClientError> {
        let api_build_info = api::app::BuildInfo {};
        let de_resp = self._resp(&api_build_info).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/app/shutdown`
    pub async fn app_shutdown(&self) -> Result<(), ClientError> {
        let api_app_shutdown = api::app::Shutdown {};
        let _s = self._resp(&api_app_shutdown).await?;

        Ok(())
    }

    /// # `/api/v2/app/preferences`
    pub async fn app_preferences(&self) -> Result<types::app::Preferences, ClientError> {
        let api_app_preferences = api::app::Preferences {};
        let de_resp = self._resp(&api_app_preferences).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/app/setPreferences`
    pub async fn app_set_preferences(
        &self,
        f: &types::app::SetPreferencesForm,
    ) -> Result<(), ClientError> {
        let api_set_preferences = api::app::SetPreferences { f: f.to_owned() };
        let _s = self._resp(&api_set_preferences).await?;

        Ok(())
    }

    /// # `/api/v2/app/defaultSavePath`
    pub async fn app_default_save_path(&self) -> Result<(), ClientError> {
        let api_default_save_path = api::app::DefaultSavePath {};
        let _s = self._resp(&api_default_save_path).await?;

        Ok(())
    }

    /// # `/api/v2/log/main`
    pub async fn log_main(
        &self,
        q: &types::log::MainQuery,
    ) -> Result<Vec<types::log::MainResponseItem>, ClientError> {
        let api_logmain = api::log::Main { q: q.to_owned() };
        let de_resp = self._resp(&api_logmain).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/log/peers`
    pub async fn log_peers(
        &self,
        q: &types::log::PeersQuery,
    ) -> Result<Vec<types::log::PeersResponseItem>, ClientError> {
        let api_logpeers = api::log::Peers { q: q.to_owned() };
        let de_resp = self._resp(&api_logpeers).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/sync/maindata`
    pub async fn sync_maindata(
        &self,
        q: &types::sync::MaindataQuery,
    ) -> Result<types::sync::MaindataResponse, ClientError> {
        let api_maindata = api::sync::Maindata { q: q.to_owned() };
        let de_resp = self._resp(&api_maindata).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/sync/torrentPeers`
    pub async fn sync_torrent_peers(
        &self,
        q: &types::sync::TorrentPeersQuery,
    ) -> Result<types::sync::TorrentPeersResponse, ClientError> {
        let api_torrent_peers = api::sync::TorrentPeers { q: q.to_owned() };
        let de_resp = self._resp(&api_torrent_peers).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/transfer/info`
    pub async fn transfer_info(&self) -> Result<types::transfer::InfoResponse, ClientError> {
        let api_transfer_info = api::transfer::Info {};
        let de_resp = self._resp(&api_transfer_info).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/transfer/speedLimitsMode`
    pub async fn speed_limits_mode(
        &self,
    ) -> Result<types::transfer::SpeedLimitsModeResponse, ClientError> {
        let api_speed_limits_mode = api::transfer::SpeedLimitsMode {};
        let de_resp = self._resp(&api_speed_limits_mode).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/transfer/toggleSpeedLimitsMode`
    pub async fn toggle_speed_limits_mode(&self) -> Result<(), ClientError> {
        let api_toggle_speed_limits_mode = api::transfer::ToggleSpeedLimitsMode {};
        let _s = self._resp(&api_toggle_speed_limits_mode).await?;

        Ok(())
    }

    /// # `/api/v2/transfer/downloadLimit`
    pub async fn download_limit(&self) -> Result<u64, ClientError> {
        let api_download_limit = api::transfer::DownloadLimit {};
        let s = self._resp(&api_download_limit).await?;
        let dl_speed: u64 = s.parse().map_err(|_e| ClientError::ParseError)?;
        Ok(dl_speed)
    }

    /// # `/api/v2/transfer/setDownloadLimit`
    pub async fn set_download_limit(&self, limit: u64) -> Result<(), ClientError> {
        let api_set_download_limit = api::transfer::SetDownloadLimit {
            f: types::transfer::SetDownloadLimitForm { limit },
        };
        let _s = self._resp(&api_set_download_limit).await?;

        Ok(())
    }

    /// # `/api/v2/transfer/uploadLimit`
    pub async fn upload_limit(&self) -> Result<u64, ClientError> {
        let api_upload_limit = api::transfer::UploadLimit {};
        let s = self._resp(&api_upload_limit).await?;
        let ul_speed: u64 = s.parse().map_err(|_e| ClientError::ParseError)?;
        Ok(ul_speed)
    }

    /// # `/api/v2/transfer/setUploadLimit`
    pub async fn set_upload_limit(&self, limit: u64) -> Result<(), ClientError> {
        let api_set_upload_limit = api::transfer::SetUploadLimit {
            f: types::transfer::SetUploadLimitForm { limit },
        };
        let _s = self._resp(&api_set_upload_limit).await?;

        Ok(())
    }

    /// # `/api/v2/transfer/banPeers`
    pub async fn ban_peers<T>(&self, peers: &[T]) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let peers: Vec<String> = peers.iter().map(|p| p.as_ref().to_owned()).collect();

        let f = types::transfer::BanPeersForm { peers };
        let api_ban_peers = api::transfer::BanPeers { f };
        let _s = self._resp(&api_ban_peers).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/info`
    pub async fn torrents_info(
        &self,
        q: &types::torrents::InfoQuery,
    ) -> Result<types::torrents::InfoResponse, ClientError> {
        let api_torrents_info = api::torrents::Info { q: q.to_owned() };
        let de_resp = self._resp(&api_torrents_info).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/properties`
    pub async fn torrents_properties(
        &self,
        hash: &str,
    ) -> Result<types::torrents::PropertiesResponse, ClientError> {
        let q = types::torrents::PropertiesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_properties = api::torrents::Properties { q };
        let de_resp = self._resp(&api_torrents_properties).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/trackers`
    pub async fn torrents_trackers(
        &self,
        hash: &str,
    ) -> Result<types::torrents::TrackersResponse, ClientError> {
        let q = types::torrents::TrackersQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_trackers = api::torrents::Trackers { q };
        let de_resp = self._resp(&api_torrents_trackers).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/webseeds`
    pub async fn torrents_webseeds(
        &self,
        hash: &str,
    ) -> Result<types::torrents::WebseedsResponse, ClientError> {
        let q = types::torrents::WebseedsQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_webseeds = api::torrents::Webseeds { q };
        let de_resp = self._resp(&api_torrents_webseeds).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/files`
    pub async fn torrents_files(
        &self,
        hash: &str,
        indexes: Option<&[u64]>,
    ) -> Result<types::torrents::FilesResponse, ClientError> {
        let indexes = indexes.map(|v| {
            v.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("|")
        });

        let q = types::torrents::FilesQuery {
            hash: hash.to_owned(),
            indexes,
        };
        let api_torrents_files = api::torrents::Files { q };
        let de_resp = self._resp(&api_torrents_files).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/pieceStates`
    pub async fn torrents_piece_states(
        &self,
        hash: &str,
    ) -> Result<types::torrents::PieceStatesResponse, ClientError> {
        let q = types::torrents::PieceStatesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_piece_states = api::torrents::PieceStates { q };
        let de_resp = self._resp(&api_torrents_piece_states).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/pieceHashes`
    pub async fn torrents_piece_hashes(
        &self,
        hash: &str,
    ) -> Result<types::torrents::PieceHashesResponse, ClientError> {
        let q = types::torrents::PieceHashesQuery {
            hash: hash.to_owned(),
        };
        let api_torrents_piece_hashes = api::torrents::PieceHashes { q };
        let de_resp = self._resp(&api_torrents_piece_hashes).await?;

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/pause`
    pub async fn torrents_pause<T>(&self, hashes: &[T]) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::PauseForm { hashes };
        let api_torrents_pause = api::torrents::Pause { f };
        let _s = self._resp(&api_torrents_pause).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/resume`
    pub async fn torrents_resume<T>(&self, hashes: &[T]) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::ResumeForm { hashes };
        let api_torrents_resume = api::torrents::Resume { f };
        let _s = self._resp(&api_torrents_resume).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/delete`
    pub async fn torrents_delete<T>(
        &self,
        hashes: &[T],
        delete_files: bool,
    ) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::DeleteForm {
            hashes,
            delete_files,
        };
        let api_torrents_delete = api::torrents::Delete { f };
        let _s = self._resp(&api_torrents_delete).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/recheck`
    pub async fn torrents_recheck<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::RecheckForm { hashes };
        let api_torrents_recheck = api::torrents::Recheck { f };
        let _s = self._resp(&api_torrents_recheck).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/reannounce`
    pub async fn torrents_reannounce<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::ReannounceForm { hashes };
        let api_torrents_reannounce = api::torrents::Reannounce { f };
        let _s = self._resp(&api_torrents_reannounce).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/add`
    pub async fn torrents_add_by_url<U>(&self, urls: &[U]) -> Result<(), ClientError>
    where
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();
        let ta = types::torrents::AddMultipart {
            urls,
            torrents: vec![],
            ..Default::default()
        };
        self.torrents_add(ta).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/add`
    pub async fn torrents_add_by_file<F>(&self, files: &[F]) -> Result<(), ClientError>
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
        let ta = types::torrents::AddMultipart {
            urls: vec![],
            torrents: files,
            ..Default::default()
        };
        self.torrents_add(ta).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/add`
    async fn torrents_add(&self, ta: types::torrents::AddMultipart) -> Result<(), ClientError> {
        let api_torrents_add = api::torrents::Add { mp: ta };
        if api_torrents_add.multipart().is_none() {
            return Err(ClientError::InvalidMultipart("no valid multipart".into()));
        }
        let _s = self._resp(&api_torrents_add).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/addTrackers`
    pub async fn torrents_add_trackers<H, U>(&self, hash: H, urls: &[U]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();

        let f = types::torrents::AddTrackersForm {
            hash: hash.as_ref().to_owned(),
            urls,
        };
        let api_torrents_add_trackers = api::torrents::AddTrackers { f };
        let _s = self._resp(&api_torrents_add_trackers).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/editTracker`
    pub async fn torrents_edit_tracker<H, U>(
        &self,
        hash: H,
        orig_url: U,
        new_url: U,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        U: AsRef<str>,
    {
        let f = types::torrents::EditTrackerForm {
            hash: hash.as_ref().to_owned(),
            orig_url: orig_url.as_ref().to_owned(),
            new_url: new_url.as_ref().to_owned(),
        };
        let api_torrents_edit_tracker = api::torrents::EditTracker { f };
        let _s = self._resp(&api_torrents_edit_tracker).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/removeTrackers`
    pub async fn torrents_remove_trackers<H, U>(
        &self,
        hash: H,
        urls: &[U],
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        U: AsRef<str>,
    {
        let urls: Vec<String> = urls.iter().map(|u| u.as_ref().to_owned()).collect();

        let f = types::torrents::RemoveTrackersForm {
            hash: hash.as_ref().to_owned(),
            urls,
        };
        let api_torrents_remove_trackers = api::torrents::RemoveTrackers { f };
        let _s = self._resp(&api_torrents_remove_trackers).await?;

        Ok(())
    }

    /// # `/api/v2/torrents/addPeers`
    pub async fn torrents_add_peers<H, P>(
        &self,
        hashes: &[H],
        peers: &[P],
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        P: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let peers: Vec<String> = peers.iter().map(|p| p.as_ref().to_owned()).collect();

        let f = types::torrents::AddPeersForm { hashes, peers };
        let api_torrents_add_peers = api::torrents::AddPeers { f };
        let _s = self._resp(&api_torrents_add_peers).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/increasePrio`
    pub async fn torrents_increase_prio<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::IncreasePrioForm { hashes };
        let api_torrents_increase_prio = api::torrents::IncreasePrio { f };
        let _s = self._resp(&api_torrents_increase_prio).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/decreasePrio`
    pub async fn torrents_decrease_prio<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::DecreasePrioForm { hashes };
        let api_torrents_decrease_prio = api::torrents::DecreasePrio { f };
        let _s = self._resp(&api_torrents_decrease_prio).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/topPrio`
    pub async fn torrents_top_prio<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::TopPrioForm { hashes };
        let api_torrents_top_prio = api::torrents::TopPrio { f };
        let _s = self._resp(&api_torrents_top_prio).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/bottomPrio`
    pub async fn torrents_bottom_prio<H>(&self, hashes: &[H]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::BottomPrioForm { hashes };
        let api_torrents_bottom_prio = api::torrents::BottomPrio { f };
        let _s = self._resp(&api_torrents_bottom_prio).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/filePrio`
    // TODO: Implement

    /// # `/api/v2/torrents/downloadLimit`
    pub async fn torrents_download_limit<H>(
        &self,
        hashes: &[H],
    ) -> Result<types::torrents::DownloadLimitResponse, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::DownloadLimitForm { hashes };
        let api_torrents_download_limit = api::torrents::DownloadLimit { f };
        let de_resp = self._resp(&api_torrents_download_limit).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/setDownloadLimit`
    pub async fn torrents_set_download_limit<H>(
        &self,
        hashes: &[H],
        limit: u64,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetDownloadLimitForm { hashes, limit };
        let api_torrents_set_download_limit = api::torrents::SetDownloadLimit { f };
        let _s = self._resp(&api_torrents_set_download_limit).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setShareLimits`
    pub async fn torrents_set_share_limits<H>(
        &self,
        hashes: &[H],
        ratio_limit: types::torrents::RatioLimit,
        seeding_time_limit: i64,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetShareLimitsForm {
            hashes,
            ratio_limit,
            seeding_time_limit,
        };
        let api_torrents_set_share_limits = api::torrents::SetShareLimits { f };
        let _s = self._resp(&api_torrents_set_share_limits).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/uploadLimit`
    pub async fn torrents_upload_limit<H>(
        &self,
        hashes: &[H],
    ) -> Result<types::torrents::UploadLimitResponse, ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::UploadLimitForm { hashes };
        let api_torrents_upload_limit = api::torrents::UploadLimit { f };
        let de_resp = self._resp(&api_torrents_upload_limit).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/setUploadLimit`
    pub async fn torrents_set_upload_limit<H>(
        &self,
        hashes: &[H],
        limit: u64,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetUploadLimitForm { hashes, limit };
        let api_torrents_set_upload_limit = api::torrents::SetUploadLimit { f };
        let _s = self._resp(&api_torrents_set_upload_limit).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setLocation`
    pub async fn torrents_set_location<H, L>(
        &self,
        hashes: &[H],
        location: L,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        L: AsRef<Path>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetLocationForm {
            hashes,
            location: location.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_set_location = api::torrents::SetLocation { f };
        let _s = self._resp(&api_torrents_set_location).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/rename`
    pub async fn torernts_rename<H, N>(&self, hash: H, name: N) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        N: AsRef<str>,
    {
        let f = types::torrents::RenameForm {
            hash: hash.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
        };
        let api_torrents_rename = api::torrents::Rename { f };
        let _s = self._resp(&api_torrents_rename).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setCategory`
    pub async fn torernts_set_category<H, C>(
        &self,
        hashes: &[H],
        category: C,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        C: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetCategoryForm {
            hashes,
            category: category.as_ref().to_owned(),
        };
        let api_torrents_set_category = api::torrents::SetCategory { f };
        let _s = self._resp(&api_torrents_set_category).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/categories`
    pub async fn torrents_categories(
        &self,
    ) -> Result<types::torrents::CategoriesResponse, ClientError> {
        let api_torrents_categories = api::torrents::Categories {};
        let de_resp = self._resp(&api_torrents_categories).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/createCategory`
    pub async fn torrents_create_category<C, P>(
        &self,
        category: C,
        save_path: P,
    ) -> Result<(), ClientError>
    where
        C: AsRef<str>,
        P: AsRef<Path>,
    {
        let f = types::torrents::CreateCategoryForm {
            category: category.as_ref().to_owned(),
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_create_category = api::torrents::CreateCategory { f };
        let _s = self._resp(&api_torrents_create_category).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/editCategory`
    pub async fn torrents_edit_category<C, P>(
        &self,
        category: C,
        save_path: P,
    ) -> Result<(), ClientError>
    where
        C: AsRef<str>,
        P: AsRef<Path>,
    {
        let f = types::torrents::EditCategoryForm {
            category: category.as_ref().to_owned(),
            save_path: save_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_edit_category = api::torrents::EditCategory { f };
        let _s = self._resp(&api_torrents_edit_category).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/removeCategories`
    pub async fn torrents_remove_categories<C>(&self, categories: &[C]) -> Result<(), ClientError>
    where
        C: AsRef<str>,
    {
        let categories: Vec<String> = categories.iter().map(|c| c.as_ref().to_owned()).collect();

        let f = types::torrents::RemoveCategoriesForm { categories };
        let api_torrents_remove_categories = api::torrents::RemoveCategories { f };
        let _s = self._resp(&api_torrents_remove_categories).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/addTags`
    pub async fn torrents_add_tags<H, T>(&self, hashes: &[H], tags: &[T]) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::torrents::AddTagsForm { hashes, tags };
        let api_torrents_add_tags = api::torrents::AddTags { f };
        let _s = self._resp(&api_torrents_add_tags).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/removeTags`
    pub async fn torrents_remove_tags<H, T>(
        &self,
        hashes: &[H],
        tags: &[T],
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        T: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::torrents::RemoveTagsForm { hashes, tags };
        let api_torrents_remove_tags = api::torrents::RemoveTags { f };
        let _s = self._resp(&api_torrents_remove_tags).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/tags`
    pub async fn torrents_tags(&self) -> Result<types::torrents::TagsResponse, ClientError> {
        let api_torrents_tags = api::torrents::Tags {};
        let de_resp = self._resp(&api_torrents_tags).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/torrents/createTags`
    pub async fn torrens_create_tags<T>(&self, tags: &[T]) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::torrents::CreateTagsForm { tags };
        let api_torrents_create_tags = api::torrents::CreateTags { f };
        let _s = self._resp(&api_torrents_create_tags).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/deleteTags`
    pub async fn torrents_delete_tags<T>(&self, tags: &[T]) -> Result<(), ClientError>
    where
        T: AsRef<str>,
    {
        let tags: Vec<String> = tags.iter().map(|t| t.as_ref().to_owned()).collect();

        let f = types::torrents::DeleteTagsForm { tags };
        let api_torrents_delete_tags = api::torrents::DeleteTags { f };
        let _s = self._resp(&api_torrents_delete_tags).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setAutoManagement`
    pub async fn torrents_set_auto_management<H>(
        &self,
        hashes: &[H],
        enable: bool,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetAutoManagementForm { hashes, enable };
        let api_torrents_set_automanagement = api::torrents::SetAutoManagement { f };
        let _s = self._resp(&api_torrents_set_automanagement).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/toggleSequentialDownload`
    pub async fn torrents_toggle_sequential_download<H>(
        &self,
        hashes: &[H],
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::ToggleSequentialDownloadForm { hashes };
        let api_torrents_toggle_sequential_download = api::torrents::ToggleSequentialDownload { f };
        let _s = self
            ._resp(&api_torrents_toggle_sequential_download)
            .await
            .unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/toggleFirstLastPiecePrio`
    pub async fn torrents_toggle_first_last_piece_prio<H>(
        &self,
        hashes: &[H],
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::ToggleFirstLastPiecePrioForm { hashes };
        let api_torrents_toggle_first_last_piece_prio =
            api::torrents::ToggleFirstLastPiecePrio { f };
        let _s = self
            ._resp(&api_torrents_toggle_first_last_piece_prio)
            .await
            .unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setForceStart`
    pub async fn torrents_set_force_start<H>(
        &self,
        hashes: &[H],
        value: bool,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetForceStartForm { hashes, value };
        let api_torrents_set_force_start = api::torrents::SetForceStart { f };
        let _s = self._resp(&api_torrents_set_force_start).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/setSuperSeeding`
    pub async fn torrents_set_super_seeding<H>(
        &self,
        hashes: &[H],
        value: bool,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
    {
        let hashes: Vec<String> = hashes.iter().map(|h| h.as_ref().to_owned()).collect();

        let f = types::torrents::SetSuperSeedingForm { hashes, value };
        let api_torrents_set_super_seeding = api::torrents::SetSuperSeeding { f };
        let _s = self._resp(&api_torrents_set_super_seeding).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/renameFile`
    pub async fn torrents_rename_file<H, P>(
        &self,
        hash: H,
        old_path: P,
        new_path: P,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        let f = types::torrents::RenameFileForm {
            hash: hash.as_ref().to_owned(),
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_file = api::torrents::RenameFile { f };
        let _s = self._resp(&api_torrents_rename_file).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/torrents/renameFolder`
    pub async fn torrents_rename_folder<H, P>(
        &self,
        hash: H,
        old_path: P,
        new_path: P,
    ) -> Result<(), ClientError>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        let f = types::torrents::RenameFolderForm {
            hash: hash.as_ref().to_owned(),
            old_path: old_path.as_ref().to_string_lossy().to_string(),
            new_path: new_path.as_ref().to_string_lossy().to_string(),
        };
        let api_torrents_rename_folder = api::torrents::RenameFolder { f };
        let _s = self._resp(&api_torrents_rename_folder).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/search/start`
    pub async fn search_start<T>(
        &self,
        pattern: T,
        plugins: T,
        category: T,
    ) -> Result<types::search::StartResponse, ClientError>
    where
        T: AsRef<str>,
    {
        let f = types::search::StartForm {
            pattern: pattern.as_ref().to_owned(),
            plugins: plugins.as_ref().to_owned(),
            category: category.as_ref().to_owned(),
        };
        let api_search_start = api::search::Start { f };
        let de_resp = self._resp(&api_search_start).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/search/stop`
    pub async fn search_stop(&self, id: u64) -> Result<(), ClientError> {
        let f = types::search::StopForm { id };
        let api_search_stop = api::search::Stop { f };
        let _s = self._resp(&api_search_stop).await.unwrap();

        Ok(())
    }

    /// # `/api/v2/search/status`
    pub async fn search_status(
        &self,
        id: Option<u64>,
    ) -> Result<types::search::StatusResponse, ClientError> {
        let q = types::search::StatusQuery { id };
        let api_search_status = api::search::Status { q };
        let de_resp = self._resp(&api_search_status).await.unwrap();

        Ok(de_resp)
    }

    /// # `/api/v2/search/results`
    pub async fn search_results(
        &self,
        id: u64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<types::search::ResultsResponse, ClientError> {
        let q = types::search::ResultsQuery { id, limit, offset };
        let api_search_results = api::search::Results { q };
        let de_resp = self._resp(&api_search_results).await.unwrap();

        Ok(de_resp)
    }
}
