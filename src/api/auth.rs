use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/auth/login`
pub struct Login {
    pub f: types::auth::LoginForm,
}

#[async_trait]
impl Endpoint for Login {
    type Query = ();
    type Form = types::auth::LoginForm;
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
pub struct Logout;

#[async_trait]
impl Endpoint for Logout {
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
