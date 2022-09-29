use serde_json::Value;
use tdameritrade_rust::{SyncTDAClient, TDAClientError};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let mut client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Movers
    let res = client.get_movers("$DJI", "up", "percent")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let res = client.get_movers("$COMPX", "down", "value")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
