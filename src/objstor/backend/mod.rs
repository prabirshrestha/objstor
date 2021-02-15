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
pub trait ObjstorBackend {
    async fn init(&mut self) -> Result<()>;
}
