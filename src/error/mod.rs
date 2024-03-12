use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error info enum
#[derive(Error, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    #[error("{0}")]
    ColorParserError(String),

    #[error("{0}")]
    InvalidParamsError(String),

    #[error("unknown error, please report this error to the developers")]
    Unknown,
}
