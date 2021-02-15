pub mod sqlite;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub is_locked: bool,
    pub created: DateTime<Utc>,
    pub is_admin: bool,
}

#[async_trait]
pub trait ObjstorBackend: UserBackend {
    async fn init(&self) -> Result<()>;
}

#[async_trait]
pub trait UserBackend {
    async fn create_user(&self, user: &User) -> Result<String>;
    async fn validate_user(&self, username: &str, password: &str) -> Result<bool>;
    async fn change_password(
        &self,
        username: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<()>;
}
