use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

#[async_trait]
pub trait Endpoint {
    /// define type of query, form and response
    type Query: Serialize;
    type Form: Serialize;
    type Response: DeserializeOwned;
    /// The endpoint relative path. Must start with a `/`
    fn relative_path(&self) -> Cow<str>;
    /// The query to be used when calling this endpoint.
    fn query(&self) -> Option<&Self::Query> {
        None
    }
    /// The form body to be used when calling this endpoint.
    fn form(&self) -> Option<&Self::Form> {
        None
    }
    /// The multipart to be used when calling this endpoint.
    fn multipart(&self) -> Option<reqwest::multipart::Form> {
        None
    }
    /// The request method of this endpoint. either POST or GET.
    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }
    /// Check the status code
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError>;
    /// Deserialize the response
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError>;
}

/// # `/api/v2/auth/login`
pub struct AuthLogin {
    pub f: types::AuthLoginForm,
}

#[async_trait]
impl Endpoint for AuthLogin {
    type Query = ();
    type Form = types::AuthLoginForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/auth/login".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::Authentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/auth/logout`
pub struct AuthLogout;

#[async_trait]
impl Endpoint for AuthLogout {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/auth/logout".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/app/version`
pub struct AppVersion;

#[async_trait]
impl Endpoint for AppVersion {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/version".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/app/webapiVersion`
pub struct AppWebApiVersion;

#[async_trait]
impl Endpoint for AppWebApiVersion {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/webapiVersion".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/app/buildInfo`
pub struct AppBuildInfo;

#[async_trait]
impl Endpoint for AppBuildInfo {
    type Query = ();
    type Form = ();
    type Response = types::AppBuildInfoResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/buildInfo".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::AppBuildInfoResponse>().await?)
    }
}

/// # `/api/v2/app/shutdown`
pub struct AppShutdown;

#[async_trait]
impl Endpoint for AppShutdown {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/shutdown".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/app/preferences`
pub struct AppPreferences;

#[async_trait]
impl Endpoint for AppPreferences {
    type Query = ();
    type Form = ();
    type Response = types::AppPreferences;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/preferences".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::AppPreferences>().await?)
    }
}

/// # `/api/v2/app/setPreferences`
pub struct AppSetPreferences {
    pub f: types::AppSetPreferencesForm,
}

#[async_trait]
impl Endpoint for AppSetPreferences {
    type Query = ();
    type Form = types::AppSetPreferencesForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/setPreferences".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/app/defaultSavePath`
pub struct AppDefaultSavePath;

#[async_trait]
impl Endpoint for AppDefaultSavePath {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/app/defaultSavePath".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/log/main`
pub struct LogMain {
    pub q: types::LogMainQuery,
}

#[async_trait]
impl Endpoint for LogMain {
    type Query = types::LogMainQuery;
    type Form = ();
    type Response = types::LogMainResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/log/main".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::LogMainResponse>().await?)
    }
}

/// # `/api/v2/log/peers`
pub struct LogPeers {
    pub q: types::LogPeersQuery,
}

#[async_trait]
impl Endpoint for LogPeers {
    type Query = types::LogPeersQuery;
    type Form = ();
    type Response = types::LogPeersResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/log/peers".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::LogPeersResponse>().await?)
    }
}

/// # `/api/v2/sync/maindata`
pub struct Maindata {
    pub q: types::SyncMaindataQuery,
}

#[async_trait]
impl Endpoint for Maindata {
    type Query = types::SyncMaindataQuery;
    type Form = ();
    type Response = types::SyncMaindataResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/sync/maindata".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SyncMaindataResponse>().await?)
    }
}

/// # `/api/v2/sync/torrentPeers`
pub struct TorrentPeers {
    pub q: types::SyncTorrentPeersQuery,
}

#[async_trait]
impl Endpoint for TorrentPeers {
    type Query = types::SyncTorrentPeersQuery;
    type Form = ();
    type Response = types::SyncTorrentPeersResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/sync/torrentPeers".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SyncTorrentPeersResponse>().await?)
    }
}

/// # `/api/v2/transfer/info`
pub struct TransferInfo;

#[async_trait]
impl Endpoint for TransferInfo {
    type Query = ();
    type Form = ();
    type Response = types::TransferInfoResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/info".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TransferInfoResponse>().await?)
    }
}

/// # `/api/v2/transfer/speedLimitsMode`
pub struct SpeedLimitsMode;

#[async_trait]
impl Endpoint for SpeedLimitsMode {
    type Query = ();
    type Form = ();
    type Response = types::SpeedLimitsModeResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/speedLimitsMode".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SpeedLimitsModeResponse>().await?)
    }
}

/// # `/api/v2/transfer/toggleSpeedLimitsMode`
pub struct ToggleSpeedLimitsMode;

#[async_trait]
impl Endpoint for ToggleSpeedLimitsMode {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/toggleSpeedLimitsMode".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/transfer/downloadLimit`
pub struct DownloadLimit;

#[async_trait]
impl Endpoint for DownloadLimit {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/downloadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/transfer/setDownloadLimit`
pub struct SetDownloadLimit {
    pub f: types::SetDownloadLimitForm,
}

#[async_trait]
impl Endpoint for SetDownloadLimit {
    type Query = ();
    type Form = types::SetDownloadLimitForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/setDownloadLimit".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/transfer/uploadLimit`
pub struct UploadLimit;

#[async_trait]
impl Endpoint for UploadLimit {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/uploadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/transfer/setUploadLimit`
pub struct SetUploadLimit {
    pub f: types::SetUploadLimitForm,
}

#[async_trait]
impl Endpoint for SetUploadLimit {
    type Query = ();
    type Form = types::SetUploadLimitForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/setUploadLimit".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/transfer/banPeers`
pub struct BanPeers {
    pub f: types::BanPeersForm,
}

#[async_trait]
impl Endpoint for BanPeers {
    type Query = ();
    type Form = types::BanPeersForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/banPeers".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/info`
pub struct TorrentsInfo {
    pub q: types::TorrentsInfoQuery,
}

#[async_trait]
impl Endpoint for TorrentsInfo {
    type Query = types::TorrentsInfoQuery;
    type Form = ();
    type Response = types::TorrentsInfoResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/info".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsInfoResponse>().await?)
    }
}

/// # `/api/v2/torrents/properties`
pub struct TorrentsProperties {
    pub q: types::TorrentsPropertiesQuery,
}

#[async_trait]
impl Endpoint for TorrentsProperties {
    type Query = types::TorrentsPropertiesQuery;
    type Form = ();
    type Response = types::TorrentsPropertiesResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/properties".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsPropertiesResponse>().await?)
    }
}

/// # `/api/v2/torrents/trackers`
pub struct TorrentsTrackers {
    pub q: types::TorrentsTrackersQuery,
}

#[async_trait]
impl Endpoint for TorrentsTrackers {
    type Query = types::TorrentsTrackersQuery;
    type Form = ();
    type Response = types::TorrentsTrackersResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/trackers".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsTrackersResponse>().await?)
    }
}

/// # `/api/v2/torrents/webseeds`
pub struct TorrentsWebseeds {
    pub q: types::TorrentsWebseedsQuery,
}

#[async_trait]
impl Endpoint for TorrentsWebseeds {
    type Query = types::TorrentsWebseedsQuery;
    type Form = ();
    type Response = types::TorrentsWebseedsResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/webseeds".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsWebseedsResponse>().await?)
    }
}

/// # `/api/v2/torrents/files`
pub struct TorrentsFiles {
    pub q: types::TorrentsFilesQuery,
}

#[async_trait]
impl Endpoint for TorrentsFiles {
    type Query = types::TorrentsFilesQuery;
    type Form = ();
    type Response = types::TorrentsFilesResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/files".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsFilesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pieceStates`
pub struct TorrentsPieceStates {
    pub q: types::TorrentsPieceStatesQuery,
}

#[async_trait]
impl Endpoint for TorrentsPieceStates {
    type Query = types::TorrentsPieceStatesQuery;
    type Form = ();
    type Response = types::TorrentsPieceStatesResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/pieceStates".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsPieceStatesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pieceHashes`
pub struct TorrentsPieceHashes {
    pub q: types::TorrentsPieceHashesQuery,
}

#[async_trait]
impl Endpoint for TorrentsPieceHashes {
    type Query = types::TorrentsPieceHashesQuery;
    type Form = ();
    type Response = types::TorrentsPieceHashesResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/pieceHashes".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.q.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsPieceHashesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pause`
pub struct TorrentsPause {
    pub f: types::TorrentsPauseForm,
}

#[async_trait]
impl Endpoint for TorrentsPause {
    type Query = ();
    type Form = types::TorrentsPauseForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/pause".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/resume`
pub struct TorrentsResume {
    pub f: types::TorrentsResumeForm,
}

#[async_trait]
impl Endpoint for TorrentsResume {
    type Query = ();
    type Form = types::TorrentsResumeForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/resume".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/delete`
pub struct TorrentsDelete {
    pub f: types::TorrentsDeleteForm,
}

#[async_trait]
impl Endpoint for TorrentsDelete {
    type Query = ();
    type Form = types::TorrentsDeleteForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/delete".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/recheck`
pub struct TorrentsRecheck {
    pub f: types::TorrentsRecheckForm,
}

#[async_trait]
impl Endpoint for TorrentsRecheck {
    type Query = ();
    type Form = types::TorrentsRecheckForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/recheck".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/reannounce`
pub struct TorrentsReannounce {
    pub f: types::TorrentsReannounceForm,
}

#[async_trait]
impl Endpoint for TorrentsReannounce {
    type Query = ();
    type Form = types::TorrentsReannounceForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/reannounce".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/add`
pub struct TorrentsAdd {
    pub mp: types::TorrentsAddMultipart,
}

#[async_trait]
impl Endpoint for TorrentsAdd {
    type Query = ();
    type Form = ();
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/add".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn multipart(&self) -> Option<reqwest::multipart::Form> {
        self.mp.to_multipart_form().ok()
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Some(ClientError::TorrentFileInvalid {
                path: {
                    let paths: Vec<String> = self.mp.torrents.iter().map(|x| x.0.clone()).collect();
                    paths.join(", ")
                },
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/addTrackers`
pub struct TorrentsAddTrackers {
    pub f: types::TorrentsAddTrackersForm,
}

#[async_trait]
impl Endpoint for TorrentsAddTrackers {
    type Query = ();
    type Form = types::TorrentsAddTrackersForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/addTrackers".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hash.clone(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/editTracker`
pub struct TorrentsEditTracker {
    pub f: types::TorrentsEditTrackerForm,
}

#[async_trait]
impl Endpoint for TorrentsEditTracker {
    type Query = ();
    type Form = types::TorrentsEditTrackerForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/editTracker".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::BAD_REQUEST => Some(ClientError::BadRequest(format!(
                "{} is not a valid URL",
                self.f.new_url
            ))),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hash.clone(),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(format!(
                "{} not found or {} already exists.",
                self.f.orig_url, self.f.new_url
            ))),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/removeTrackers`
pub struct TorrentsRemoveTrackers {
    pub f: types::TorrentsRemoveTrackersForm,
}

#[async_trait]
impl Endpoint for TorrentsRemoveTrackers {
    type Query = ();
    type Form = types::TorrentsRemoveTrackersForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/removeTrackers".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hash.clone(),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(format!(
                "all URL ({}) not found.",
                self.f.urls.join(", ")
            ))),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/addPeers`
pub struct TorrentsAddPeers {
    pub f: types::TorrentsAddPeersForm,
}

#[async_trait]
impl Endpoint for TorrentsAddPeers {
    type Query = ();
    type Form = types::TorrentsAddPeersForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/addPeers".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::BAD_REQUEST => Some(ClientError::BadRequest(format!(
                "All peers ({}) are not valid",
                self.f.peers.join(", ")
            ))),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),

            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/increasePrio`
pub struct TorrentsIncreasePrio {
    pub f: types::TorrentsIncreasePrioForm,
}

#[async_trait]
impl Endpoint for TorrentsIncreasePrio {
    type Query = ();
    type Form = types::TorrentsIncreasePrioForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/increasePrio".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Torrent queueing is not enabled.".to_string(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/decreasePrio`
pub struct TorrentsDecreasePrio {
    pub f: types::TorrentsDecreasePrioForm,
}

#[async_trait]
impl Endpoint for TorrentsDecreasePrio {
    type Query = ();
    type Form = types::TorrentsDecreasePrioForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/decreasePrio".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Torrent queueing is not enabled.".to_string(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/topPrio`
pub struct TorrentsTopPrio {
    pub f: types::TorrentsTopPrioForm,
}

#[async_trait]
impl Endpoint for TorrentsTopPrio {
    type Query = ();
    type Form = types::TorrentsTopPrioForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/topPrio".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Torrent queueing is not enabled.".to_string(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/bottomPrio`
pub struct TorrentsBottomPrio {
    pub f: types::TorrentsBottomPrioForm,
}

#[async_trait]
impl Endpoint for TorrentsBottomPrio {
    type Query = ();
    type Form = types::TorrentsBottomPrioForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/bottomPrio".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Torrent queueing is not enabled.".to_string(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/filePrio`
// TODO: Implement

/// # `/api/v2/torrents/downloadLimit`
pub struct TorrentsDownloadLimit {
    pub f: types::TorrentsDownloadLimitForm,
}

#[async_trait]
impl Endpoint for TorrentsDownloadLimit {
    type Query = ();
    type Form = types::TorrentsDownloadLimitForm;
    type Response = types::TorrentsDownloadLimitResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/downloadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsDownloadLimitResponse>().await?)
    }
}

/// # `/api/v2/torrents/setDownloadLimit`
pub struct TorrentsSetDownloadLimit {
    pub f: types::TorrentsSetDownloadLimitForm,
}

#[async_trait]
impl Endpoint for TorrentsSetDownloadLimit {
    type Query = ();
    type Form = types::TorrentsSetDownloadLimitForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setDownloadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setShareLimits`
pub struct TorrentsSetShareLimits {
    pub f: types::TorrentsSetShareLimitsForm,
}

#[async_trait]
impl Endpoint for TorrentsSetShareLimits {
    type Query = ();
    type Form = types::TorrentsSetShareLimitsForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setShareLimits".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/uploadLimit`
pub struct TorrentsUploadLimit {
    pub f: types::TorrentsUploadLimitForm,
}

#[async_trait]
impl Endpoint for TorrentsUploadLimit {
    type Query = ();
    type Form = types::TorrentsUploadLimitForm;
    type Response = types::TorrentsUploadLimitResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/uploadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsUploadLimitResponse>().await?)
    }
}

/// # `/api/v2/torrents/setUploadLimit`
pub struct TorrentsSetUploadLimit {
    pub f: types::TorrentsSetUploadLimitForm,
}

#[async_trait]
impl Endpoint for TorrentsSetUploadLimit {
    type Query = ();
    type Form = types::TorrentsSetUploadLimitForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setUploadLimit".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setLocation`
pub struct TorrentsSetLocation {
    pub f: types::TorrentsSetLocationForm,
}

#[async_trait]
impl Endpoint for TorrentsSetLocation {
    type Query = ();
    type Form = types::TorrentsSetLocationForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setLocation".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => Some(ClientError::BadRequest("Save path is empty".into())),
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Unable to create save path directory".into(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/rename`
pub struct TorrentsRename {
    pub f: types::TorrentsRenameForm,
}

#[async_trait]
impl Endpoint for TorrentsRename {
    type Query = ();
    type Form = types::TorrentsRenameForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/rename".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hash.clone(),
            }),
            StatusCode::CONFLICT => Some(ClientError::Conflict("Torrent name is empty".into())),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setCategory`
pub struct TorrentsSetCategory {
    pub f: types::TorrentsSetCategoryForm,
}

#[async_trait]
impl Endpoint for TorrentsSetCategory {
    type Query = ();
    type Form = types::TorrentsSetCategoryForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setCategory".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::TorrentNotFound {
                hash: self.f.hashes.join(", "),
            }),
            StatusCode::CONFLICT => {
                Some(ClientError::Conflict("Category name does not exist".into()))
            }
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/categories`
pub struct TorrentsCategories;

#[async_trait]
impl Endpoint for TorrentsCategories {
    type Query = ();
    type Form = ();
    type Response = types::TorrentsCategoriesResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/categories".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsCategoriesResponse>().await?)
    }
}

/// # `/api/v2/torrents/createCategory`
pub struct TorrentsCreateCategory {
    pub f: types::TorrentsCreateCategoryForm,
}

#[async_trait]
impl Endpoint for TorrentsCreateCategory {
    type Query = ();
    type Form = types::TorrentsCreateCategoryForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/createCategory".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => {
                Some(ClientError::BadRequest("Category name is empty".into()))
            }
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::CONFLICT => Some(ClientError::Conflict("Category name is invalid".into())),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/editCategory`
pub struct TorrentsEditCategory {
    pub f: types::TorrentsEditCategoryForm,
}

#[async_trait]
impl Endpoint for TorrentsEditCategory {
    type Query = ();
    type Form = types::TorrentsEditCategoryForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/editCategory".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => {
                Some(ClientError::BadRequest("Category name is empty".into()))
            }
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::CONFLICT => Some(ClientError::Conflict("Category name is invalid".into())),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/removeCategories`
pub struct TorrentsRemoveCategories {
    pub f: types::TorrentsRemoveCategoriesForm,
}

#[async_trait]
impl Endpoint for TorrentsRemoveCategories {
    type Query = ();
    type Form = types::TorrentsRemoveCategoriesForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/removeCategories".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/addTags`
pub struct TorrentsAddTags {
    pub f: types::TorrentsAddTagsForm,
}

#[async_trait]
impl Endpoint for TorrentsAddTags {
    type Query = ();
    type Form = types::TorrentsAddTagsForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/addTags".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/removeTags`
pub struct TorrentsRemoveTags {
    pub f: types::TorrentsRemoveTagsForm,
}

#[async_trait]
impl Endpoint for TorrentsRemoveTags {
    type Query = ();
    type Form = types::TorrentsRemoveTagsForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/removeTags".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/tags`
pub struct TorrentsTags;

#[async_trait]
impl Endpoint for TorrentsTags {
    type Query = ();
    type Form = ();
    type Response = types::TorrentsTagsResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/tags".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::TorrentsTagsResponse>().await?)
    }
}

/// # `/api/v2/torrents/createTags`
pub struct TorrentsCreateTags {
    pub f: types::TorrentsCreateTagsForm,
}

#[async_trait]
impl Endpoint for TorrentsCreateTags {
    type Query = ();
    type Form = types::TorrentsCreateTagsForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/createTags".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/deleteTags`
pub struct TorrentsDeleteTags {
    pub f: types::TorrentsDeleteTagsForm,
}

#[async_trait]
impl Endpoint for TorrentsDeleteTags {
    type Query = ();
    type Form = types::TorrentsDeleteTagsForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/deleteTags".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setAutoManagement`
pub struct TorrentsSetAutoManagement {
    pub f: types::TorrentsSetAutoManagementForm,
}

#[async_trait]
impl Endpoint for TorrentsSetAutoManagement {
    type Query = ();
    type Form = types::TorrentsSetAutoManagementForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setAutoManagement".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/toggleSequentialDownload`
pub struct TorrentsToggleSequentialDownload {
    pub f: types::TorrentsToggleSequentialDownloadForm,
}

#[async_trait]
impl Endpoint for TorrentsToggleSequentialDownload {
    type Query = ();
    type Form = types::TorrentsToggleSequentialDownloadForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/toggleSequentialDownload".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/toggleFirstLastPiecePrio`
pub struct TorrentsToggleFirstLastPiecePrio {
    pub f: types::TorrentsToggleFirstLastPiecePrioForm,
}

#[async_trait]
impl Endpoint for TorrentsToggleFirstLastPiecePrio {
    type Query = ();
    type Form = types::TorrentsToggleFirstLastPiecePrioForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/toggleFirstLastPiecePrio".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setForceStart`
pub struct TorrentsSetForceStart {
    pub f: types::TorrentsSetForceStartForm,
}

#[async_trait]
impl Endpoint for TorrentsSetForceStart {
    type Query = ();
    type Form = types::TorrentsSetForceStartForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setForceStart".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/setSuperSeeding`
pub struct TorrentsSetSuperSeeding {
    pub f: types::TorrentsSetSuperSeedingForm,
}

#[async_trait]
impl Endpoint for TorrentsSetSuperSeeding {
    type Query = ();
    type Form = types::TorrentsSetSuperSeedingForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/setSuperSeeding".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/renameFile`
pub struct TorrentsRenameFile {
    pub f: types::TorrentsRenameFileForm,
}

#[async_trait]
impl Endpoint for TorrentsRenameFile {
    type Query = ();
    type Form = types::TorrentsRenameFileForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/renameFile".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => {
                Some(ClientError::BadRequest("Missing new_path parameter".into()))
            }
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Invalid new_path or old_path, or new_path already in use".into(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/torrents/renameFolder`
pub struct TorrentsRenameFolder {
    pub f: types::TorrentsRenameFolderForm,
}

#[async_trait]
impl Endpoint for TorrentsRenameFolder {
    type Query = ();
    type Form = types::TorrentsRenameFolderForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/torrents/renameFolder".into()
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => {
                Some(ClientError::BadRequest("Missing new_path parameter".into()))
            }
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Invalid new_path or old_path, or new_path already in use".into(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/search/start`
pub struct SearchStart {
    pub f: types::SearchStartForm,
}

#[async_trait]
impl Endpoint for SearchStart {
    type Query = ();
    type Form = types::SearchStartForm;
    type Response = types::SearchStartResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/search/start".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "User has reached the limit of max Running searches (currently set to 5)".into(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SearchStartResponse>().await?)
    }
}

/// # `/api/v2/search/stop`
pub struct SearchStop {
    pub f: types::SearchStopForm,
}

#[async_trait]
impl Endpoint for SearchStop {
    type Query = ();
    type Form = types::SearchStopForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/search/stop".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::SearchJobNotFound { id: self.f.id }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}

/// # `/api/v2/search/status`
pub struct SearchStatus {
    pub q: types::SearchStatusQuery,
}

#[async_trait]
impl Endpoint for SearchStatus {
    type Query = types::SearchStatusQuery;
    type Form = ();
    type Response = types::SearchStatusResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/search/status".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::SearchJobNotFound {
                id: self.q.id.unwrap(),
            }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SearchStatusResponse>().await?)
    }
}

/// # `/api/v2/search/results`
pub struct SearchResults {
    pub q: types::SearchResultsQuery,
}

#[async_trait]
impl Endpoint for SearchResults {
    type Query = types::SearchResultsQuery;
    type Form = ();
    type Response = types::SearchResultsResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/search/results".into()
    }
    fn query(&self) -> Option<&Self::Query> {
        Some(&self.q)
    }
    fn method(&self) -> reqwest::Method {
        Method::GET
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::SearchJobNotFound { id: self.q.id }),
            StatusCode::CONFLICT => Some(ClientError::Conflict(
                "Offset is too large or too small".into(),
            )),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.json::<types::SearchResultsResponse>().await?)
    }
}

/// # `/api/v2/search/delete`
pub struct SearchDelete {
    pub f: types::SearchDeleteForm,
}

#[async_trait]
impl Endpoint for SearchDelete {
    type Query = ();
    type Form = types::SearchDeleteForm;
    type Response = String;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/search/delete".into()
    }
    fn form(&self) -> Option<&Self::Form> {
        Some(&self.f)
    }
    fn method(&self) -> reqwest::Method {
        Method::POST
    }
    fn check_status(&self, status: reqwest::StatusCode) -> Option<ClientError> {
        match status {
            StatusCode::OK => None,
            StatusCode::FORBIDDEN => Some(ClientError::NeedAuthentication),
            StatusCode::NOT_FOUND => Some(ClientError::SearchJobNotFound { id: self.f.id }),
            _ => Some(ClientError::Unknown),
        }
    }
    async fn de_response(&self, res: reqwest::Response) -> Result<Self::Response, ClientError> {
        Ok(res.text().await?)
    }
}