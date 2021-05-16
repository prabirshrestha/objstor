mod error;
use async_trait::async_trait;
pub use error::ObjstorError;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
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
    async fn create_user(&self, user: &User) -> Result<String, ObjstorError>;
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_with_salt(contents: &str, salt: &str) -> Result<String, ObjstorError> {
    Ok(bcrypt::hash_with_salt(contents, 12, salt.as_bytes())
        .map_err(|e| ObjstorError::HashError(e.to_string()))?
        .to_string())
}
