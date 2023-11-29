use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/transfer/info`
pub struct Info;

#[async_trait]
impl Endpoint for Info {
    type Query = ();
    type Form = ();
    type Response = types::transfer::InfoResponse;
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
        Ok(res.json::<types::transfer::InfoResponse>().await?)
    }
}

/// # `/api/v2/transfer/speedLimitsMode`
pub struct SpeedLimitsMode;

#[async_trait]
impl Endpoint for SpeedLimitsMode {
    type Query = ();
    type Form = ();
    type Response = types::transfer::SpeedLimitsModeResponse;
    fn relative_path(&self) -> Cow<str> {
        "/api/v2/transfer/speedLimitsMode".into()
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
        Ok(res
            .json::<types::transfer::SpeedLimitsModeResponse>()
            .await?)
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

/// # `/api/v2/transfer/setDownloadLimit`
pub struct SetDownloadLimit {
    pub f: types::transfer::SetDownloadLimitForm,
}

#[async_trait]
impl Endpoint for SetDownloadLimit {
    type Query = ();
    type Form = types::transfer::SetDownloadLimitForm;
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

/// # `/api/v2/transfer/setUploadLimit`
pub struct SetUploadLimit {
    pub f: types::transfer::SetUploadLimitForm,
}

#[async_trait]
impl Endpoint for SetUploadLimit {
    type Query = ();
    type Form = types::transfer::SetUploadLimitForm;
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
    pub f: types::transfer::BanPeersForm,
}

#[async_trait]
impl Endpoint for BanPeers {
    type Query = ();
    type Form = types::transfer::BanPeersForm;
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
