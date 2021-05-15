use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjstorError {
    #[error("unknown error")]
    Unknown,
}
