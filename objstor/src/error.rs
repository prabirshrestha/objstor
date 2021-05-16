use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjstorError {
    #[error("connection error")]
    ConnectionError(String),

    #[error("hash error")]
    HashError(String),

    #[error("provider migration error")]
    ProviderMigrationError(String),

    #[error("unknown error")]
    Unknown(String),
}
