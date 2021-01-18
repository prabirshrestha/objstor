use anyhow::Result;

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub locked: bool,
}

pub trait UserBackend {
    fn get_by_id(&self, id: &str) -> Result<Option<User>>;
    fn get_by_username(&self, username: &str) -> Result<Option<User>>;
    fn insert(&self, user: &User) -> Result<String>;
    fn update(&self, user: &User) -> Result<()>;
}
