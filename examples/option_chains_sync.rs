use serde_json::Value;
use tdameritrade_rust::{OptionChainParams, SyncTDAClient, TDAClientError};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let mut client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Option Chain
    let option_params = OptionChainParams::default()
        .symbol("AAPL")
        .contract_type("CALL")
        .strategy("SINGLE")
        .range("ITM")
        .expiration_month("SEP")
        .option_type("S")
        .build()
        .expect("Failed To Build");

    let res = client.get_option_chain(&option_params)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let option_params = OptionChainParams::default()
        .symbol("AAPL")
        .build()
        .expect("Failed To Build");

    let res = client.get_option_chain(&option_params)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}