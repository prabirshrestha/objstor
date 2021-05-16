use async_trait::async_trait;
use objstor::{ObjstorError, ObjstorProvider};

#[derive(Clone, Debug)]
pub struct SqliteObjstorProvider {
    connection_string: String,
}

impl SqliteObjstorProvider {
    pub fn new(connection: &str) -> Self {
        Self {
            connection_string: connection.to_owned(),
        }
    }
}

#[async_trait]
impl ObjstorProvider for SqliteObjstorProvider {
    async fn init(&self) -> Result<(), ObjstorError> {
        Ok(())
    }
}
