use thiserror::Error;

/// Error info enum
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    ColorParserError(String),

    #[error("{0}")]
    InvalidParamsError(String),

    #[error("unknown error, please report this error to the developers")]
    Unknown,
}
