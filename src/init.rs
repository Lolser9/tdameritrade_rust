use crate::token::Token;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use thirtyfour_sync::prelude::*;
use urlencoding::decode;

/// Creates token file for subsequent use
///
/// Official Documentation: https://developer.tdameritrade.com/authentication/apis/post/token-0
pub fn create_token_file(
    chromedriver_path: String,
    client_id: String,
    redirect_uri: String,
    token_path: String,
) {
    // Start ChromeDriver
    let mut chrome_driver = Command::new(chromedriver_path)
        .spawn()
        .expect("Failed To Start Chrome Driver");

    // Start WebDriver
    let chrome = DesiredCapabilities::chrome();
    let driver =
        WebDriver::new("http://localhost:9515", chrome).expect("Unable To Start WebDriver");

    // Format URL
    let code_url: String = format!(
        "https://auth.tdameritrade.com/auth?response_type=code&client_id={}&redirect_uri={}",
        client_id, redirect_uri
    );

    // Get Authentification Code
    driver.get(code_url).expect("Unable To Go To Url");

    let code: String;
    let code_url: String = format!("{}/?code=", redirect_uri);

    // Loop Until Code Is In URL
    loop {
        let url: String = driver.current_url().expect("Unable To Get Current URL");
        if url.contains(&code_url) {
            // Get Code
            code = url.split("=").last().unwrap().to_string();
            break;
        }
    }

    // Decode Code
    let decoded_code: String = decode(code.as_str()).unwrap().to_string();

    // Kill WebDriver
    driver.quit().expect("Unable To Kill WebDriver");

    // Kill ChromeDriver
    chrome_driver.kill().expect("Unable To Kill ChromeDriver");

    // Get Token URL
    let token_url: &str = "https://api.tdameritrade.com/v1/oauth2/token";

    // Create Hashmap To Store Params
    let mut params: HashMap<String, String> = HashMap::new();

    // Parameters
    params.insert("grant_type".into(), "authorization_code".into());
    params.insert("access_type".into(), "offline".into());
    params.insert("code".into(), decoded_code);
    params.insert("client_id".into(), client_id);
    params.insert("redirect_uri".into(), redirect_uri.clone());

    // Create Reqwest Client
    let reqwest_client: Client = Client::new();

    // Post Request
    let res = reqwest_client
        .post(token_url)
        .form(&params)
        .send()
        .expect("Post Request Failed");

    // Convert JSON To Token
    let mut token: Token = res
        .json::<Token>()
        .expect("Unable To Convert JSON To Token");

    // Get Current Time
    let now: SystemTime = SystemTime::now();
    let epoch_time: u64 = now
        .duration_since(UNIX_EPOCH)
        .expect("Time somehow went backwards")
        .as_secs();

    // Replace Refresh Token Expire Time
    token.replace_refresh_token_expire_time(epoch_time + 6480000); // Refresh Token Expires After 75 Days

    // Write Token File
    fs::write(token_path, token.to_string()).expect("Unable To Write Token To File");
}
