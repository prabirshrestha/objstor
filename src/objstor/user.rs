use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub locked: bool,
    pub created: DateTime<Utc>,
    pub is_admin: bool,
}

#[async_trait]
pub trait UserBackend: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn create_user(&self, user: &User) -> Result<String>;
    async fn validate_password(&self, username: &str, password: &str) -> Result<bool>;
}
