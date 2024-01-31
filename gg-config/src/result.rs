use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
}

impl Error {
    pub fn is_not_exist(&self) -> bool {
        match self {
            Error::IO(err) => err.kind() == std::io::ErrorKind::NotFound,
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;