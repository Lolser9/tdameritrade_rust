use crate::TDAClientError;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    scope: String,
    expires_in: i64,
    pub refresh_token_expires_in: u64,
    token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAccessToken {
    pub access_token: String,
    scope: String,
    expires_in: i64,
    token_type: String,
}

impl Token {
    // Replace Access Token
    pub fn replace_access_token(&mut self, new_access_token: String) {
        self.access_token = new_access_token;
    }

    // Replace Refresh Token
    pub fn replace_refresh_token(&mut self, new_refresh_token: String) {
        self.refresh_token = new_refresh_token;
    }

    // Replace Refresh Token Time
    pub fn replace_refresh_token_expire_time(&mut self, new_refresh_token_expire_time: u64) {
        self.refresh_token_expires_in = new_refresh_token_expire_time
    }

    // Convert To String
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// Read Token File
pub fn read_token_file(token_file_path: &String) -> Result<Token, TDAClientError> {
    // Open File
    let file_contents: String = fs::read_to_string(token_file_path)?;

    // Convert Token File To Token
    let token: Token = serde_json::from_str(file_contents.as_str())?;

    // Return Token
    Ok(token)
}
