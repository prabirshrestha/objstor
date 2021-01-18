use crate::objstor::{User, UserBackend};
use anyhow::Result;
use async_trait::async_trait;

struct SqliteUserBackend {}

#[async_trait]
impl UserBackend for SqliteUserBackend {
    async fn get_by_id(&self, id: &str) -> Result<Option<User>> {
        todo!()
    }

    async fn get_by_username(&self, name: &str) -> Result<Option<User>> {
        todo!()
    }

    async fn insert(&self, user: &User) -> Result<String> {
        todo!()
    }

    async fn update(&self, user: &User) -> Result<()> {
        todo!()
    }
}

pub fn new_sqlite_user_backend() -> impl UserBackend {
    return SqliteUserBackend {};
}
