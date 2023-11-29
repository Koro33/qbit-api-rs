use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;
use super::Endpoint;

/// # `/api/v2/torrents/info`
pub struct Info {
    pub q: types::torrents::InfoQuery,
}

#[async_trait]
impl Endpoint for Info {
    type Query = types::torrents::InfoQuery;
    type Form = ();
    type Response = types::torrents::InfoResponse;
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
        Ok(res.json::<types::torrents::InfoResponse>().await?)
    }
}

/// # `/api/v2/torrents/properties`
pub struct Properties {
    pub q: types::torrents::PropertiesQuery,
}

#[async_trait]
impl Endpoint for Properties {
    type Query = types::torrents::PropertiesQuery;
    type Form = ();
    type Response = types::torrents::PropertiesResponse;
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
        Ok(res.json::<types::torrents::PropertiesResponse>().await?)
    }
}

/// # `/api/v2/torrents/trackers`
pub struct Trackers {
    pub q: types::torrents::TrackersQuery,
}

#[async_trait]
impl Endpoint for Trackers {
    type Query = types::torrents::TrackersQuery;
    type Form = ();
    type Response = types::torrents::TrackersResponse;
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
        Ok(res.json::<types::torrents::TrackersResponse>().await?)
    }
}

/// # `/api/v2/torrents/webseeds`
pub struct Webseeds {
    pub q: types::torrents::WebseedsQuery,
}

#[async_trait]
impl Endpoint for Webseeds {
    type Query = types::torrents::WebseedsQuery;
    type Form = ();
    type Response = types::torrents::WebseedsResponse;
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
        Ok(res.json::<types::torrents::WebseedsResponse>().await?)
    }
}

/// # `/api/v2/torrents/files`
pub struct Files {
    pub q: types::torrents::FilesQuery,
}

#[async_trait]
impl Endpoint for Files {
    type Query = types::torrents::FilesQuery;
    type Form = ();
    type Response = types::torrents::FilesResponse;
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
        Ok(res.json::<types::torrents::FilesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pieceStates`
pub struct PieceStates {
    pub q: types::torrents::PieceStatesQuery,
}

#[async_trait]
impl Endpoint for PieceStates {
    type Query = types::torrents::PieceStatesQuery;
    type Form = ();
    type Response = types::torrents::PieceStatesResponse;
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
        Ok(res.json::<types::torrents::PieceStatesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pieceHashes`
pub struct PieceHashes {
    pub q: types::torrents::PieceHashesQuery,
}

#[async_trait]
impl Endpoint for PieceHashes {
    type Query = types::torrents::PieceHashesQuery;
    type Form = ();
    type Response = types::torrents::PieceHashesResponse;
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
        Ok(res.json::<types::torrents::PieceHashesResponse>().await?)
    }
}

/// # `/api/v2/torrents/pause`
pub struct Pause {
    pub f: types::torrents::PauseForm,
}

#[async_trait]
impl Endpoint for Pause {
    type Query = ();
    type Form = types::torrents::PauseForm;
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
pub struct Resume {
    pub f: types::torrents::ResumeForm,
}

#[async_trait]
impl Endpoint for Resume {
    type Query = ();
    type Form = types::torrents::ResumeForm;
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
pub struct Delete {
    pub f: types::torrents::DeleteForm,
}

#[async_trait]
impl Endpoint for Delete {
    type Query = ();
    type Form = types::torrents::DeleteForm;
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
pub struct Recheck {
    pub f: types::torrents::RecheckForm,
}

#[async_trait]
impl Endpoint for Recheck {
    type Query = ();
    type Form = types::torrents::RecheckForm;
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
pub struct Reannounce {
    pub f: types::torrents::ReannounceForm,
}

#[async_trait]
impl Endpoint for Reannounce {
    type Query = ();
    type Form = types::torrents::ReannounceForm;
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
pub struct Add {
    pub mp: types::torrents::AddMultipart,
}

#[async_trait]
impl Endpoint for Add {
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
pub struct AddTrackers {
    pub f: types::torrents::AddTrackersForm,
}

#[async_trait]
impl Endpoint for AddTrackers {
    type Query = ();
    type Form = types::torrents::AddTrackersForm;
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
pub struct EditTracker {
    pub f: types::torrents::EditTrackerForm,
}

#[async_trait]
impl Endpoint for EditTracker {
    type Query = ();
    type Form = types::torrents::EditTrackerForm;
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
pub struct RemoveTrackers {
    pub f: types::torrents::RemoveTrackersForm,
}

#[async_trait]
impl Endpoint for RemoveTrackers {
    type Query = ();
    type Form = types::torrents::RemoveTrackersForm;
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
pub struct AddPeers {
    pub f: types::torrents::AddPeersForm,
}

#[async_trait]
impl Endpoint for AddPeers {
    type Query = ();
    type Form = types::torrents::AddPeersForm;
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
pub struct IncreasePrio {
    pub f: types::torrents::IncreasePrioForm,
}

#[async_trait]
impl Endpoint for IncreasePrio {
    type Query = ();
    type Form = types::torrents::IncreasePrioForm;
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
pub struct DecreasePrio {
    pub f: types::torrents::DecreasePrioForm,
}

#[async_trait]
impl Endpoint for DecreasePrio {
    type Query = ();
    type Form = types::torrents::DecreasePrioForm;
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
pub struct TopPrio {
    pub f: types::torrents::TopPrioForm,
}

#[async_trait]
impl Endpoint for TopPrio {
    type Query = ();
    type Form = types::torrents::TopPrioForm;
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
pub struct BottomPrio {
    pub f: types::torrents::BottomPrioForm,
}

#[async_trait]
impl Endpoint for BottomPrio {
    type Query = ();
    type Form = types::torrents::BottomPrioForm;
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
pub struct DownloadLimit {
    pub f: types::torrents::DownloadLimitForm,
}

#[async_trait]
impl Endpoint for DownloadLimit {
    type Query = ();
    type Form = types::torrents::DownloadLimitForm;
    type Response = types::torrents::DownloadLimitResponse;
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
        Ok(res.json::<types::torrents::DownloadLimitResponse>().await?)
    }
}

/// # `/api/v2/torrents/setDownloadLimit`
pub struct SetDownloadLimit {
    pub f: types::torrents::SetDownloadLimitForm,
}

#[async_trait]
impl Endpoint for SetDownloadLimit {
    type Query = ();
    type Form = types::torrents::SetDownloadLimitForm;
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
pub struct SetShareLimits {
    pub f: types::torrents::SetShareLimitsForm,
}

#[async_trait]
impl Endpoint for SetShareLimits {
    type Query = ();
    type Form = types::torrents::SetShareLimitsForm;
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
pub struct UploadLimit {
    pub f: types::torrents::UploadLimitForm,
}

#[async_trait]
impl Endpoint for UploadLimit {
    type Query = ();
    type Form = types::torrents::UploadLimitForm;
    type Response = types::torrents::UploadLimitResponse;
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
        Ok(res.json::<types::torrents::UploadLimitResponse>().await?)
    }
}

/// # `/api/v2/torrents/setUploadLimit`
pub struct SetUploadLimit {
    pub f: types::torrents::SetUploadLimitForm,
}

#[async_trait]
impl Endpoint for SetUploadLimit {
    type Query = ();
    type Form = types::torrents::SetUploadLimitForm;
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
pub struct SetLocation {
    pub f: types::torrents::SetLocationForm,
}

#[async_trait]
impl Endpoint for SetLocation {
    type Query = ();
    type Form = types::torrents::SetLocationForm;
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
pub struct Rename {
    pub f: types::torrents::RenameForm,
}

#[async_trait]
impl Endpoint for Rename {
    type Query = ();
    type Form = types::torrents::RenameForm;
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
pub struct SetCategory {
    pub f: types::torrents::SetCategoryForm,
}

#[async_trait]
impl Endpoint for SetCategory {
    type Query = ();
    type Form = types::torrents::SetCategoryForm;
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
pub struct Categories;

#[async_trait]
impl Endpoint for Categories {
    type Query = ();
    type Form = ();
    type Response = types::torrents::CategoriesResponse;
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
        Ok(res.json::<types::torrents::CategoriesResponse>().await?)
    }
}

/// # `/api/v2/torrents/createCategory`
pub struct CreateCategory {
    pub f: types::torrents::CreateCategoryForm,
}

#[async_trait]
impl Endpoint for CreateCategory {
    type Query = ();
    type Form = types::torrents::CreateCategoryForm;
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
pub struct EditCategory {
    pub f: types::torrents::EditCategoryForm,
}

#[async_trait]
impl Endpoint for EditCategory {
    type Query = ();
    type Form = types::torrents::EditCategoryForm;
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
pub struct RemoveCategories {
    pub f: types::torrents::RemoveCategoriesForm,
}

#[async_trait]
impl Endpoint for RemoveCategories {
    type Query = ();
    type Form = types::torrents::RemoveCategoriesForm;
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
pub struct AddTags {
    pub f: types::torrents::AddTagsForm,
}

#[async_trait]
impl Endpoint for AddTags {
    type Query = ();
    type Form = types::torrents::AddTagsForm;
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
pub struct RemoveTags {
    pub f: types::torrents::RemoveTagsForm,
}

#[async_trait]
impl Endpoint for RemoveTags {
    type Query = ();
    type Form = types::torrents::RemoveTagsForm;
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
pub struct Tags;

#[async_trait]
impl Endpoint for Tags {
    type Query = ();
    type Form = ();
    type Response = types::torrents::TagsResponse;
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
        Ok(res.json::<types::torrents::TagsResponse>().await?)
    }
}

/// # `/api/v2/torrents/createTags`
pub struct CreateTags {
    pub f: types::torrents::CreateTagsForm,
}

#[async_trait]
impl Endpoint for CreateTags {
    type Query = ();
    type Form = types::torrents::CreateTagsForm;
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
pub struct DeleteTags {
    pub f: types::torrents::DeleteTagsForm,
}

#[async_trait]
impl Endpoint for DeleteTags {
    type Query = ();
    type Form = types::torrents::DeleteTagsForm;
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
pub struct SetAutoManagement {
    pub f: types::torrents::SetAutoManagementForm,
}

#[async_trait]
impl Endpoint for SetAutoManagement {
    type Query = ();
    type Form = types::torrents::SetAutoManagementForm;
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
pub struct ToggleSequentialDownload {
    pub f: types::torrents::ToggleSequentialDownloadForm,
}

#[async_trait]
impl Endpoint for ToggleSequentialDownload {
    type Query = ();
    type Form = types::torrents::ToggleSequentialDownloadForm;
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
pub struct ToggleFirstLastPiecePrio {
    pub f: types::torrents::ToggleFirstLastPiecePrioForm,
}

#[async_trait]
impl Endpoint for ToggleFirstLastPiecePrio {
    type Query = ();
    type Form = types::torrents::ToggleFirstLastPiecePrioForm;
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
pub struct SetForceStart {
    pub f: types::torrents::SetForceStartForm,
}

#[async_trait]
impl Endpoint for SetForceStart {
    type Query = ();
    type Form = types::torrents::SetForceStartForm;
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
pub struct SetSuperSeeding {
    pub f: types::torrents::SetSuperSeedingForm,
}

#[async_trait]
impl Endpoint for SetSuperSeeding {
    type Query = ();
    type Form = types::torrents::SetSuperSeedingForm;
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
pub struct RenameFile {
    pub f: types::torrents::RenameFileForm,
}

#[async_trait]
impl Endpoint for RenameFile {
    type Query = ();
    type Form = types::torrents::RenameFileForm;
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
pub struct RenameFolder {
    pub f: types::torrents::RenameFolderForm,
}

#[async_trait]
impl Endpoint for RenameFolder {
    type Query = ();
    type Form = types::torrents::RenameFolderForm;
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