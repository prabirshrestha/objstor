use async_trait::async_trait;
use futures_core::future::BoxFuture;
use objstor::{ObjstorError, ObjstorProvider};
use sqlx::{
    migrate::{Migration, MigrationSource, Migrator},
    SqlitePool,
};

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
        let migrator = Migrator::new(self)
            .await
            .map_err(|e| ObjstorError::Unknown(e.to_string()))?;
        migrator
            .run(&self.pool)
            .await
            .map_err(|e| ObjstorError::Unknown(e.to_string()))?;
        Ok(())
    }
}

impl<'s> MigrationSource<'s> for &'s SqliteObjstorProvider {
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, sqlx::error::BoxDynError>> {
        Box::pin(async move {
            let migrations = vec![];
            Ok(migrations)
        })
    }
}
