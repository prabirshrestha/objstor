use anyhow::Result;
use async_trait::async_trait;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub locked: bool,
}

#[async_trait]
pub trait UserBackend {
    async fn get_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn insert(&self, user: &User) -> Result<String>;
    async fn update(&self, user: &User) -> Result<()>;
}
