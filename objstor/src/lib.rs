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
    async fn validate_user(&self, username: &str, password: &str) -> Result<bool, ObjstorError>;
}

const NANOID_ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn new_id() -> String {
    nanoid::nanoid!(21, &NANOID_ALPHABET)
}

pub fn hash_with_salt(contents: &str, salt: &str) -> Result<String, ObjstorError> {
    Ok(bcrypt::hash_with_salt(contents, 12, salt.as_bytes())
        .map_err(|e| ObjstorError::HashError(e.to_string()))?
        .to_string())
}
