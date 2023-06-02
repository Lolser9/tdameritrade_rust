use tdameritrade_rust::{output::market_hours::MarketHours, AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Hours For Multiple Markets
    let markets = vec!["EQUITY", "OPTION", "FUTURE", "BOND", "FOREX"];
    let res = client
        .get_hours_for_multiple_markets(&markets, "2022-12-30")
        .await?;
    let res_json = serde_json::from_str::<MarketHours>(&res)?;
    println!("{:?}", res_json);

    // Get Hours For Single Market
    let res = client
        .get_hours_for_single_market("EQUITY", "2022-12-30")
        .await?;
    let res_json = serde_json::from_str::<MarketHours>(&res)?;
    println!("{:?}", res_json.equity.unwrap().equity_data);

    Ok(())
}
