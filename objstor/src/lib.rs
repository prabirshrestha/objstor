mod error;
use async_trait::async_trait;
pub use error::ObjstorError;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub is_locked: bool,
    pub is_admin: bool,
}

pub struct Storage {
    pub id: String,
}

pub struct Object {
    pub id: String,
}

#[async_trait]
pub trait ObjstorProvider: UserObjstorProvider + Send + Sync + 'static {
    async fn init(&self) -> Result<(), ObjstorError>;
}

#[async_trait]
pub trait UserObjstorProvider {
    async fn has_users(&self) -> Result<bool, ObjstorError>;
}
