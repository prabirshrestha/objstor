use super::ObjstorBackend;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct SqliteObjstorBackend {
    client: SqlitePool,
}

#[async_trait]
impl ObjstorBackend for SqliteObjstorBackend {
    async fn init(&mut self) -> Result<()> {
        todo!()
    }
}
