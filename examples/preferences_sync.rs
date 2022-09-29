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

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Preferences
    let res = client.get_preferences(acct_id)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

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

    client.update_preferences(acct_id, update_spec)?;

    Ok(())
}
