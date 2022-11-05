use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TDAClientError {
    /// Unable To Connect To TD Ameritrade API
    ClientConnectionError(reqwest::Error),
    /// Unable To Deserialize Output Into Struct
    ClientParseError(serde_json::Error),
    /// Unable To Find or Parse Token File
    TokenFileError(std::io::Error),
}

impl std::error::Error for TDAClientError {}

impl Display for TDAClientError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TDAClientError::ClientConnectionError(err) => {
                write!(f, "Web Connection Error: {}", err)
            }
            TDAClientError::ClientParseError(err) => write!(f, "Unable To Parse: {}", err),
            TDAClientError::TokenFileError(err) => {
                write!(f, "Unable To Find Or Parse Token File: {}", err)
            }
        }
    }
}

impl From<reqwest::Error> for TDAClientError {
    fn from(err: reqwest::Error) -> Self {
        TDAClientError::ClientConnectionError(err)
    }
}

impl From<serde_json::Error> for TDAClientError {
    fn from(err: serde_json::Error) -> Self {
        TDAClientError::ClientParseError(err)
    }
}

impl From<std::io::Error> for TDAClientError {
    fn from(err: std::io::Error) -> Self {
        TDAClientError::TokenFileError(err)
    }
}
