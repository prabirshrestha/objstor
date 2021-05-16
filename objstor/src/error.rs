use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjstorError {
    #[error("connection error")]
    ConnectionError(String),

    #[error("empty password")]
    EmptyPassword(),

    #[error("hash error")]
    HashError(String),

    #[error("invalid user id")]
    InvalidUserId(String),

    #[error("provider migration error")]
    ProviderMigrationError(String),

    #[error("unknown error")]
    Unknown(String),
}
