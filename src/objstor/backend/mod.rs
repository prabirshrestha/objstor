pub mod sqlite;

use anyhow::Result;
use async_trait::async_trait;
use async_vfs::Vfs;
use chrono::{DateTime, Utc};

pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub is_locked: bool,
    pub created: DateTime<Utc>,
    pub is_admin: bool,
}

pub struct Storage {
    pub id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub provider: String,
    pub data: String,
    pub edata: String,
    pub is_locked: bool,
    pub is_index: bool,
}

pub struct Object {
    pub id: String,
    pub parent_id: Option<String>,
    pub storage_id: String,
    pub name: String,
    pub object_type: ObjectType,
    pub ctime: Option<DateTime<Utc>>,
    pub mtime: Option<DateTime<Utc>>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub description: Option<String>,
}

pub enum ObjectType {
    File = 0,
    Dir = 1,
}

#[async_trait]
pub trait ObjstorBackend: UserBackend + StorageBackend {
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

#[async_trait]
pub trait StorageBackend {
    async fn add_storage(&self) -> Result<String>;
    async fn get_storage(&self, id: &str) -> Result<Storage>;
    async fn remove_storage(&self, id: &str) -> Result<Storage>;
    async fn get_vfs(&self, id: &str) -> Result<Box<dyn Vfs>>;
}
