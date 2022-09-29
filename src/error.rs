use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TDAClientError {
    /// reqwest library
    ClientConnectionError(reqwest::Error),
    /// serde_json library
    ClientParseError(serde_json::Error),
}

impl std::error::Error for TDAClientError {}

impl Display for TDAClientError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TDAClientError::ClientConnectionError(err) => {
                write!(f, "Web Connection Error: {}", err)
            }
            TDAClientError::ClientParseError(err) => write!(f, "Unable To Parse: {}", err),
        }
    }
}

impl From<serde_json::Error> for TDAClientError {
    fn from(err: serde_json::Error) -> Self {
        TDAClientError::ClientParseError(err)
    }
}

impl From<reqwest::Error> for TDAClientError {
    fn from(err: reqwest::Error) -> Self {
        TDAClientError::ClientConnectionError(err)
    }
}
