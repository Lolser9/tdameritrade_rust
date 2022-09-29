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

    // Get Hours For Multiple Markets
    let markets = vec!["EQUITY", "OPTION", "FUTURE", "BOND", "FOREX"];
    let res = client.get_hours_for_multiple_markets(&markets, "2022-12-30")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Hours For Single Market
    let res = client.get_hours_for_single_market("EQUITY", "2022-12-30")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
