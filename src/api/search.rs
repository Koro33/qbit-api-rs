use super::Endpoint;
use crate::error::ClientError;
use crate::types;
use async_trait::async_trait;
use reqwest::{Method, StatusCode};
use std::borrow::Cow;

/// # `/api/v2/search/start`
pub struct Start {
    pub f: types::search::StartForm,
}

#[async_trait]
impl Endpoint for Start {
    type Query = ();
    type Form = types::search::StartForm;
    type Response = types::search::StartResponse;
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
        Ok(res.json::<types::search::StartResponse>().await?)
    }
}

/// # `/api/v2/search/stop`
pub struct Stop {
    pub f: types::search::StopForm,
}

#[async_trait]
impl Endpoint for Stop {
    type Query = ();
    type Form = types::search::StopForm;
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
pub struct Status {
    pub q: types::search::StatusQuery,
}

#[async_trait]
impl Endpoint for Status {
    type Query = types::search::StatusQuery;
    type Form = ();
    type Response = types::search::StatusResponse;
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
        Ok(res.json::<types::search::StatusResponse>().await?)
    }
}

/// # `/api/v2/search/results`
pub struct Results {
    pub q: types::search::ResultsQuery,
}

#[async_trait]
impl Endpoint for Results {
    type Query = types::search::ResultsQuery;
    type Form = ();
    type Response = types::search::ResultsResponse;
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
        Ok(res.json::<types::search::ResultsResponse>().await?)
    }
}

/// # `/api/v2/search/delete`
pub struct Delete {
    pub f: types::search::DeleteForm,
}

#[async_trait]
impl Endpoint for Delete {
    type Query = ();
    type Form = types::search::DeleteForm;
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
