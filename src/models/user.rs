use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl User {
    /// Hash a password using bcrypt
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, hash)
    }
}
