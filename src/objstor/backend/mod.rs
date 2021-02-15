pub mod sqlite;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ObjstorBackend {
    async fn init(&mut self) -> Result<()>;
}
