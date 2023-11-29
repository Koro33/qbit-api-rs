use serde::{self, Deserialize, Serialize};
use serde_repr::*;

/// # `/api/v2/log/main`
#[derive(Debug, Clone, Serialize)]
pub struct MainQuery {
    pub normal: bool,
    pub info: bool,
    pub warning: bool,
    pub critical: bool,
    pub last_known_id: i64,
}

impl Default for MainQuery {
    fn default() -> Self {
        Self {
            normal: true,
            info: true,
            warning: true,
            critical: true,
            last_known_id: -1,
        }
    }
}

/// # `/api/v2/log/main`
pub type MainResponse = Vec<MainResponseItem>;

/// # `/api/v2/log/main`
/// [`MainResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct MainResponseItem {
    pub id: u64,
    pub message: String,
    pub timestamp: u64,
    pub r#type: MainType,
}

/// # `/api/v2/log/main`
/// [`MainResponseItem::r#type`]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MainType {
    NORMAL = 1,
    INFO = 2,
    WARNING = 4,
    CRITICAL = 8,
}

/// # `/api/v2/log/peers`
#[derive(Debug, Clone, Serialize)]
pub struct PeersQuery {
    pub last_known_id: i64,
}

impl Default for PeersQuery {
    fn default() -> Self {
        Self { last_known_id: -1 }
    }
}

/// # `/api/v2/log/peers`
pub type PeersResponse = Vec<PeersResponseItem>;

/// # `/api/v2/log/peers`
/// [`PeersResponse`]
#[derive(Debug, Clone, Deserialize)]
pub struct PeersResponseItem {
    pub id: u64,
    pub ip: String,
    pub timestamp: u64,
    pub blocked: bool,
    pub reason: String,
}
