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

    // Search Instruments
    let res = client.search_instruments("AAPL", "symbol-search").await?; // Returns Data For Symbol AAPL
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let res = client.search_instruments("A.*", "symbol-regex").await?; // Returns Data For All Symbols That Start With Letter A
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Instrument
    let msft_cusip = 594918104;
    let res = client.get_instrument(msft_cusip).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
