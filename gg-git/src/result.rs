use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("command exit with non-zero status: {}", String::from_utf8_lossy(&.0.stdout))]
    Exit(std::process::Output),
}

pub type Result<T> = std::result::Result<T, Error>;