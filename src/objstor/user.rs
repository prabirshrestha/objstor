use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub locked: bool,
    pub created_time: Utc,
}

#[async_trait]
pub trait UserBacked {
    async fn init(&self) -> Result<()>;
    async fn create_user(&self, user: &User) -> Result<Option<String>>;
    async fn validate_password(&self, username: &str, password: &str) -> Result<bool>;
}
