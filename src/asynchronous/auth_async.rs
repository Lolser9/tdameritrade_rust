use crate::token::{read_token_file, NewAccessToken, Token};
use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Debug)]
pub struct AsyncAuth {
    client_id: String,
    redirect_uri: String,
    token_path: String,
    access_token: String,
    refresh_token: String,
    access_token_expire_time: u64,
    refresh_token_expire_time: u64,
}

impl AsyncAuth {
    // Create New Auth
    pub fn new(client_id: String, redirect_uri: String, token_path: String) -> Self {
        // Read Token File
        let token: Token = read_token_file(&token_path);

        // Create New Auth
        let mut auth: AsyncAuth = AsyncAuth::default();

        // Fill Parameters
        auth.client_id = client_id;
        auth.redirect_uri = redirect_uri;
        auth.token_path = token_path;
        auth.access_token = token.access_token;
        auth.access_token_expire_time = 0;
        auth.refresh_token = token.refresh_token;
        auth.refresh_token_expire_time = token.refresh_token_expires_in;

        // Return Auth
        auth
    }

    // Get Access Token
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }

    // Check Token Validity
    pub async fn check_token_validity(&mut self) {
        // Get Current Time
        let now: SystemTime = SystemTime::now();
        let epoch_time: u64 = now
            .duration_since(UNIX_EPOCH)
            .expect("Time somehow went backwards")
            .as_secs();

        // Check If Refresh Token Is Valid
        if epoch_time > self.refresh_token_expire_time {
            self.request_new_token("refresh_token").await;
        }

        // Check If Access Token Is Valid
        if epoch_time > self.access_token_expire_time {
            self.request_new_token("access_token").await;
        }
    }

    // Request New Token
    pub async fn request_new_token(&mut self, token_type: &str) {
        // Create Client
        let reqwest_client: Client = Client::new();

        // Create Hashmap To Store Params
        let mut params: HashMap<String, String> = HashMap::new();

        // Parameters
        params.insert("grant_type".into(), "refresh_token".into());
        params.insert("refresh_token".into(), self.refresh_token.clone());
        params.insert("client_id".into(), self.client_id.clone());
        params.insert("redirect_uri".into(), self.redirect_uri.clone());

        // Alternate Parameter For Refresh Token
        if token_type == "refresh_token" {
            params.insert("access_type".into(), "offline".into());
        }

        // Create Url
        let url: String = "https://api.tdameritrade.com/v1/oauth2/token".into();

        // Request New Token
        let res = reqwest_client
            .post(url)
            .form(&params)
            .send()
            .await
            .expect("Post Request Failed");

        // Get Response Text
        let res_text: String = res.text().await.expect("Unable To Get Post Text");

        // Get JSON Response For Access Token
        if token_type == "access_token" {
            match serde_json::from_str::<NewAccessToken>(&res_text) {
                Ok(res_json) => {
                    // Get Current Time
                    let now: SystemTime = SystemTime::now();
                    let epoch_time: u64 = now
                        .duration_since(UNIX_EPOCH)
                        .expect("Time somehow went backwards")
                        .as_secs();

                    // Update Auth Values
                    self.access_token = res_json.access_token.clone();
                    self.access_token_expire_time = epoch_time + 1500; // 25 Minutes Until Access Token Refresh

                    // Read Token File
                    let mut token: Token = read_token_file(&self.token_path);

                    // Replace Access Token
                    token.replace_access_token(res_json.access_token);

                    // Write To File
                    fs::write(self.token_path.clone(), token.to_string())
                        .expect("Unable to write to file");
                }

                // Too Many Calls To TDAmeritrade Servers
                Err(_) => return,
            }
        }

        // Get JSON Response For Refresh Token
        if token_type == "refresh_token" {
            match serde_json::from_str::<Token>(&res_text) {
                Ok(res_json) => {
                    // Get Current Time
                    let now: SystemTime = SystemTime::now();
                    let epoch_time: u64 = now
                        .duration_since(UNIX_EPOCH)
                        .expect("Time somehow went backwards")
                        .as_secs();

                    // Update Auth Values
                    self.access_token = res_json.access_token.clone();
                    self.access_token_expire_time = epoch_time + 1500; // 25 Minutes Until Access Token Refresh
                    self.refresh_token = res_json.refresh_token.clone();
                    self.refresh_token_expire_time = epoch_time + 6480000; // Refresh Token Expires After 75 Days

                    // Read Token File
                    let mut token: Token = read_token_file(&self.token_path);

                    // Replace Tokens
                    token.replace_access_token(res_json.access_token);
                    token.replace_refresh_token(res_json.refresh_token);

                    // Replace Refresh Token Expire Time
                    token.replace_refresh_token_expire_time(epoch_time + 6480000); // Refresh Token Expires After 75 Days

                    // Write To File
                    fs::write(self.token_path.clone(), token.to_string())
                        .expect("Unable to write to file");
                }

                // Too Many Calls To TDAmeritrade Servers
                Err(_) => return,
            }
        }
    }
}
