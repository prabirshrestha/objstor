use crate::objstor::{User, UserBacked};
use async_trait::async_trait;

pub struct SqliteUserBackend {}

impl SqliteUserBackend {
    fn new(connstr: &str) -> Self {
        SqliteUserBackend {}
    }
}

#[async_trait]
impl UserBacked for SqliteUserBackend {
    async fn init() -> anyhow::Result<()> {
        todo!()
    }

    async fn create_user(user: &User) -> anyhow::Result<Option<String>> {
        todo!()
    }

    async fn validate_password(username: &str, password: &str) -> anyhow::Result<bool> {
        todo!()
    }
}
