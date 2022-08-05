use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}
