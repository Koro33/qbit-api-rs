use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Initialize Error")]
    Initialize(String),
    #[error("Token expired, Need Authentication")]
    NeedAuthentication,
    #[error("Authentication Failed")]
    Authentication,
    #[error("url parse failed")]
    UrlParse(#[from] url::ParseError),
    #[error("Invalid Headers")]
    InvalidHeaders(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Invalid Multipart: {0}")]
    InvalidMultipart(String),
    #[error("Send Request Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Torrents with hashes ({hash}) Not Found.")]
    TorrentNotFound {
        hash: String
    },
    #[error("Torrent File under {path} is invalid.")]
    TorrentFileInvalid {
        path: String
    },
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Error: {0}")]
    Other(String),
    #[error("Unknown Error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum TypesError {
    #[error("Error: {0}")]
    Other(String),
}

