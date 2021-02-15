use super::ObjstorBackend;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct SqliteObjstorBackend {
    client: SqlitePool,
}

impl SqliteObjstorBackend {
    pub async fn new(connection_string: &str) -> Result<Self> {
        Ok(SqliteObjstorBackend {
            client: SqlitePool::connect(connection_string).await?,
        })
    }
}

#[async_trait]
impl ObjstorBackend for SqliteObjstorBackend {
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }
}
