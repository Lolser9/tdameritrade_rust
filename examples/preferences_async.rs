use tdameritrade_rust::{output::preferences::Preferences, AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Preferences
    let res = client.get_preferences(acct_id).await?;
    let res_json = serde_json::from_str::<Preferences>(&res)?;
    println!("{:?}", res_json);

    // Update Preferences
    let update_spec = r#"{
        "expressTrading": false,
        "directOptionsRouting": false,
        "directEquityRouting": false,
        "defaultEquityOrderLegInstruction": "None",
        "defaultEquityOrderType": "MARKET",
        "defaultEquityOrderPriceLinkType": "NONE",
        "defaultEquityOrderDuration": "DAY",
        "defaultEquityOrderMarketSession": "NORMAL",
        "defaultEquityQuantity": 0,
        "mutualFundTaxLotMethod": "FIFO",
        "optionTaxLotMethod": "FIFO",
        "equityTaxLotMethod": "FIFO",
        "defaultAdvancedToolLaunch": "NONE",
        "authTokenTimeout": "FIFTY_FIVE_MINUTES"
    }"#;

    client.update_preferences(acct_id, update_spec).await?;

    Ok(())
}
