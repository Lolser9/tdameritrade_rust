use tdameritrade_rust::{SyncTDAClient, TDAClientError};
mod config;
use serde_json::Value;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let mut client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Search Instruments
    let res = client.search_instruments("AAPL", "symbol-search")?; // Returns Data For Symbol AAPL
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let res = client.search_instruments("A.*", "symbol-regex")?; // Returns Data For All Symbols That Start With Letter A
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Instrument
    let msft_cusip = 594918104;
    let res = client.get_instrument(msft_cusip)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
