use async_trait::async_trait;
use objstor::{ObjstorError, ObjstorProvider};
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
        Ok(())
    }
}
