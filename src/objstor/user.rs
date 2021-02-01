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
    async fn init() -> Result<()>;
    async fn create_user(user: &User) -> Result<Option<String>>;
    async fn validate_password(username: &str, password: &str) -> Result<bool>;
}
