use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/sync/maindata`
pub struct Maindata {
    pub q: types::sync::MaindataQuery,
}

#[async_trait]
impl Endpoint for Maindata {
    type Query = types::sync::MaindataQuery;
    type Form = ();
    type Response = types::sync::MaindataResponse;
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
        Ok(res.json::<types::sync::MaindataResponse>().await?)
    }
}

/// # `/api/v2/sync/torrentPeers`
pub struct TorrentPeers {
    pub q: types::sync::TorrentPeersQuery,
}

#[async_trait]
impl Endpoint for TorrentPeers {
    type Query = types::sync::TorrentPeersQuery;
    type Form = ();
    type Response = types::sync::TorrentPeersResponse;
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
        Ok(res.json::<types::sync::TorrentPeersResponse>().await?)
    }
}
