use crate::error::TDAClientError;
use reqwest::blocking::Client;
use std::collections::HashMap;

static BASE_URL: &str = "https://api.tdameritrade.com/v1/";

pub fn delete(
    reqwest_client: &Client,
    access_token: String,
    url: String,
) -> Result<(), TDAClientError> {
    // Format Url
    let formatted_url: String = format!("{}{}", BASE_URL, url);

    // Put Request
    reqwest_client
        .delete(formatted_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;

    Ok(())
}

pub fn get(
    reqwest_client: &Client,
    access_token: String,
    params: HashMap<String, String>,
    url: String,
) -> Result<String, TDAClientError> {
    // Format Url
    let formatted_url: String = format!("{}{}", BASE_URL, url);

    // Get Request
    let res_text = reqwest_client
        .get(formatted_url)
        .query(&params)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?
        .text()?;

    // Return Response Text
    Ok(res_text)
}

pub fn patch(
    reqwest_client: &Client,
    access_token: String,
    body: String,
    url: String,
) -> Result<(), TDAClientError> {
    // Format Url
    let formatted_url: String = format!("{}{}", BASE_URL, url);

    // Put Request
    reqwest_client
        .patch(formatted_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("content-type", "application/json")
        .body(body)
        .send()?;

    Ok(())
}

pub fn post(
    reqwest_client: &Client,
    access_token: String,
    body: String,
    url: String,
) -> Result<(), TDAClientError> {
    // Format Url
    let formatted_url: String = format!("{}{}", BASE_URL, url);

    // Post Request
    reqwest_client
        .post(formatted_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("content-type", "application/json")
        .body(body)
        .send()?;

    Ok(())
}

pub fn put(
    reqwest_client: &Client,
    access_token: String,
    body: String,
    url: String,
) -> Result<(), TDAClientError> {
    // Format Url
    let formatted_url: String = format!("{}{}", BASE_URL, url);

    // Put Request
    reqwest_client
        .put(formatted_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("content-type", "application/json")
        .body(body)
        .send()?;

    Ok(())
}
