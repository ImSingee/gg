use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("clap error")]
    Clap(#[from] clap::Error),
    #[error("{0}")]
    Err(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn exit(&self) -> ! {
        match self {
            Error::Clap(e) => e.exit(),
            Error::Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub(crate) fn error(reason: &str) -> Error {
    Error::Err(reason.to_string())
}

pub(crate) fn exit<T>(result: Result<T>) -> ! {
    match result {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            e.exit();
        }
    }
}