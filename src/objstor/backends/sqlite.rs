use crate::objstor::{User, UserBackend};
use anyhow::Result;

struct SqliteUserBackend {}

impl UserBackend for SqliteUserBackend {
    fn get_by_id(&self, id: &str) -> Result<Option<User>> {
        todo!()
    }

    fn get_by_username(&self, name: &str) -> Result<Option<User>> {
        todo!()
    }

    fn insert(&self, user: &User) -> Result<String> {
        todo!()
    }

    fn update(&self, user: &User) -> Result<()> {
        todo!()
    }
}

pub fn new_sqlite_user_backend() -> impl UserBackend {
    return SqliteUserBackend {};
}
