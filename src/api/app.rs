use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/app/version`
pub struct Version;

#[async_trait]
impl Endpoint for Version {
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
pub struct WebApiVersion;

#[async_trait]
impl Endpoint for WebApiVersion {
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
pub struct BuildInfo;

#[async_trait]
impl Endpoint for BuildInfo {
    type Query = ();
    type Form = ();
    type Response = types::app::BuildInfoResponse;
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
        Ok(res.json::<types::app::BuildInfoResponse>().await?)
    }
}

/// # `/api/v2/app/shutdown`
pub struct Shutdown;

#[async_trait]
impl Endpoint for Shutdown {
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
pub struct Preferences;

#[async_trait]
impl Endpoint for Preferences {
    type Query = ();
    type Form = ();
    type Response = types::app::Preferences;
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
        Ok(res.json::<types::app::Preferences>().await?)
    }
}

/// # `/api/v2/app/setPreferences`
pub struct SetPreferences {
    pub f: types::app::SetPreferencesForm,
}

#[async_trait]
impl Endpoint for SetPreferences {
    type Query = ();
    type Form = types::app::SetPreferencesForm;
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
pub struct DefaultSavePath;

#[async_trait]
impl Endpoint for DefaultSavePath {
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
