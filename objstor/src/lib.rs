mod error;
use async_trait::async_trait;
pub use error::ObjstorError;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

pub struct Storage {
    pub id: String,
}

pub struct Object {
    pub id: String,
}

#[async_trait]
pub trait ObjstorBackend: UserBackend + StorageBackend {
    async fn init(&self) -> Result<(), ObjstorError>;
}

#[async_trait]
pub trait UserBackend {}

#[async_trait]
pub trait StorageBackend {}
