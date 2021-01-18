use anyhow::Result;

pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub locked: bool,
}

trait UserBackend {
    fn get_by_id(id: &str) -> Result<Option<User>>;
    fn get_by_username(name: &str) -> Result<Option<User>>;
    fn insert(user: &User) -> Result<()>;
    fn update(user: &User) -> Result<()>;
}
