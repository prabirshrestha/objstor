use async_trait::async_trait;
use objstor::{ObjstorError, ObjstorProvider, UserObjstorProvider};
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct SqliteObjstorProvider {
    pool: SqlitePool,
}

impl SqliteObjstorProvider {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn connect(connection_string: &str) -> Result<Self, ObjstorError> {
        Ok(Self::new(
            SqlitePool::connect(connection_string)
                .await
                .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?,
        ))
    }
}

#[async_trait]
impl ObjstorProvider for SqliteObjstorProvider {
    async fn init(&self) -> Result<(), ObjstorError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| ObjstorError::ProviderMigrationError(e.to_string()))?;

        if !self.has_users().await? {
            // create users
        }

        Ok(())
    }
}

#[async_trait]
impl UserObjstorProvider for SqliteObjstorProvider {
    async fn has_users(&self) -> Result<bool, ObjstorError> {
        let user_count: i64 = sqlx::query_scalar("SELECT count(*) FROM user")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ObjstorError::ConnectionError(e.to_string()))?;
        Ok(user_count == 0)
    }
}
