pub mod app;
pub mod auth;
pub mod log;
pub mod search;
pub mod sync;
pub mod torrents;
pub mod transfer;

use crate::error::ClientError;
use async_trait::async_trait;
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
