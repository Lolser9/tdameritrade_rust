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

    // Get Order
    let res = client.get_order(acct_id, 9094169803)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Orders By Path
    let res = client.get_orders_by_path(acct_id, 5, "2022-01-01", "2022-12-31", "FILLED")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Orders By Query
    let res = client.get_orders_by_query(None, 5, "2022-01-01", "2022-12-31", "FILLED")?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Place Order
    place_order(&mut client, acct_id)?; // Limit Order For 1 Share Of AAPL At $1000

    // Replace Order
    let replace_order_id = 0;
    replace_order(&mut client, acct_id, replace_order_id)?; // Replace Existing Order For 1 Share Of AAPL At $1000

    // Cancel Order
    let cancel_order_id = 0;
    client.cancel_order(acct_id, cancel_order_id)?;

    // Get Saved Order
    let saved_order_id = 0;
    let res = client.get_saved_order(acct_id, saved_order_id)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Saved Order By Path
    let res = client.get_saved_orders_by_path(acct_id)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Create Saved Order
    create_saved_order(&mut client, acct_id)?; // Create Saved Limit Order For 1 Share Of AAPL At $1000

    // Replace Saved Order
    let replace_order_id = 0;
    replace_saved_order(&mut client, acct_id, replace_order_id)?; // Replace Existing Saved Order For 1 Share Of AAPL At $1000

    // Delete Saved Order
    let delete_order_id = 0;
    client.delete_saved_order(acct_id, delete_order_id)?;

    Ok(())
}

fn place_order(client: &mut SyncTDAClient, acct_id: i64) -> Result<(), TDAClientError> {
    let order_spec = r#"{
        "orderType": "LIMIT",
        "session": "SEAMLESS",
        "duration": "GOOD_TILL_CANCEL",
        "price": 1000.00,
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
          {
            "instruction": "BUY",
            "quantity": 1,
            "instrument": {
              "symbol": "AAPL",
              "assetType": "EQUITY"
            }
          }
        ]
      }"#;

    client.place_order(acct_id, order_spec)?;

    Ok(())
}

fn replace_order(
    client: &mut SyncTDAClient,
    acct_id: i64,
    order_id: i64,
) -> Result<(), TDAClientError> {
    let order_spec = r#"{
        "orderType": "LIMIT",
        "session": "SEAMLESS",
        "duration": "GOOD_TILL_CANCEL",
        "price": 1000.00,
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
          {
            "instruction": "BUY",
            "quantity": 1,
            "instrument": {
              "symbol": "AAPL",
              "assetType": "EQUITY"
            }
          }
        ]
      }"#;

    client.replace_order(acct_id, order_id, order_spec)?;

    Ok(())
}

fn create_saved_order(client: &mut SyncTDAClient, acct_id: i64) -> Result<(), TDAClientError> {
    let order_spec = r#"{
        "orderType": "LIMIT",
        "session": "SEAMLESS",
        "duration": "GOOD_TILL_CANCEL",
        "price": 1000.00,
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
          {
            "instruction": "BUY",
            "quantity": 1,
            "instrument": {
              "symbol": "AAPL",
              "assetType": "EQUITY"
            }
          }
        ]
      }"#;

    client.create_saved_order(acct_id, order_spec)?;

    Ok(())
}

fn replace_saved_order(
    client: &mut SyncTDAClient,
    acct_id: i64,
    order_id: i64,
) -> Result<(), TDAClientError> {
    let order_spec = r#"{
        "orderType": "LIMIT",
        "session": "SEAMLESS",
        "duration": "GOOD_TILL_CANCEL",
        "price": 1000.00,
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
          {
            "instruction": "BUY",
            "quantity": 1,
            "instrument": {
              "symbol": "AAPL",
              "assetType": "EQUITY"
            }
          }
        ]
      }"#;

    client.replace_saved_order(acct_id, order_id, order_spec)?;

    Ok(())
}
