use serde_json::Value;
use tdameritrade_rust::{AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Movers
    let res = client.get_movers("$DJI", "up", "percent").await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let res = client.get_movers("$COMPX", "down", "value").await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
