use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Glob(#[from] glob::GlobError),
    #[error("{0}")]
    Pattern(#[from] glob::PatternError),
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}
