use super::{ObjstorBackend, User, UserBackend};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct SqliteObjstorBackend {
    pool: SqlitePool,
}

impl SqliteObjstorBackend {
    pub async fn new(connection_string: &str) -> Result<Self> {
        Ok(SqliteObjstorBackend {
            pool: SqlitePool::connect(connection_string).await?,
        })
    }
}

#[async_trait]
impl ObjstorBackend for SqliteObjstorBackend {
    async fn init(&mut self) -> Result<()> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS user (
                id varchar(256) PRIMARY KEY,
                username varchar(256) UNIQUE NOT NULL,
                password varchar(256) NOT NULL,
                created DATETIME NOT NULL,
                is_locked BOOLEAN NOT NULL CHECK (is_locked IN (0,1)),
                is_admin BOOLEAN NOT NULL CHECK (is_admin IN (0,1))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl UserBackend for SqliteObjstorBackend {
    async fn create_user(&mut self, user: &User) -> Result<String> {
        todo!()
    }
}
