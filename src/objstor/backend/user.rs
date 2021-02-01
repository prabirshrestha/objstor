use crate::objstor::{User, UserBacked};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteUserBackend<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SqliteUserBackend<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        SqliteUserBackend { pool }
    }
}

#[async_trait]
impl<'a> UserBacked for SqliteUserBackend<'a> {
    async fn init(&self) -> anyhow::Result<()> {
        todo!()
    }

    async fn create_user(&self, user: &User) -> anyhow::Result<Option<String>> {
        todo!()
    }

    async fn validate_password(&self, username: &str, password: &str) -> anyhow::Result<bool> {
        todo!()
    }
}
