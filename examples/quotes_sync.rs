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

    // Get Quote
    let res = client.get_quote("AAPL")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Quotes
    let symbols = vec!["AAPL", "AMZN", "AMD", "NVDA"];
    let res = client.get_quotes(&symbols)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
