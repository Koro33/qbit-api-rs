
use serde::{self, Serialize};

/// # `/api/v2/auth/login`
#[derive(Debug, Clone, Default, Serialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
