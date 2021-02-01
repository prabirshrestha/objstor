use crate::objstor::{User, UserBacked};
use async_trait::async_trait;
use sqlx::{Executor, SqlitePool};

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
        // TODO: add more index
        let mut conn = self.pool.acquire().await?;
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS user (
                id varchar(256) PRIMARY KEY,
                username varchar(256) UNIQUE NOT NULL,
                password varchar(256) NOT NULL,
                created DATETIME NOT NULL,
                locked BOOLEAN NOT NULL CHECK (locked IN (0,1)),
                is_admin BOOLEAN NOT NULL CHECK (is_admin IN (0,1))
            )"#,
        )
        .await?;

        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS storage (
                id varchar(256) PRIMARY KEY,
                name varchar(500) NOT NULL,
                created DATETIME NOT NULL,
                type varchar(256) NOT NULL,
                data TEXT NOT NULL,
                indexed BOOLEAN NOT NULL CHECK (indexed IN (0,1))
            )"#,
        )
        .await?;
        Ok(())
    }

    async fn create_user(&self, user: &User) -> anyhow::Result<Option<String>> {
        todo!()
    }

    async fn validate_password(&self, username: &str, password: &str) -> anyhow::Result<bool> {
        todo!()
    }
}
