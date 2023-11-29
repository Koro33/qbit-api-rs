use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/log/main`
pub struct Main {
    pub q: types::log::MainQuery,
}

#[async_trait]
impl Endpoint for Main {
    type Query = types::log::MainQuery;
    type Form = ();
    type Response = types::log::MainResponse;
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
        Ok(res.json::<types::log::MainResponse>().await?)
    }
}

/// # `/api/v2/log/peers`
pub struct Peers {
    pub q: types::log::PeersQuery,
}

#[async_trait]
impl Endpoint for Peers {
    type Query = types::log::PeersQuery;
    type Form = ();
    type Response = types::log::PeersResponse;
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
        Ok(res.json::<types::log::PeersResponse>().await?)
    }
}
