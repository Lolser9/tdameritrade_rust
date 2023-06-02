use tdameritrade_rust::{
    order_templates::equity_buy_limit, output::trading::Order, SyncTDAClient, TDAClientError,
};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Order
    let order_id = 0;
    let res = client.get_order(acct_id, order_id)?;
    let res_json = serde_json::from_str::<Order>(&res)?;

    if let Order::SimpleOrder(simple_order) = res_json {
        println!("{}", simple_order.status);
    } else if let Order::ComplexOrder(complex_order) = res_json {
        println!("{}", complex_order.status);
    }

    // Get Orders By Path
    let res = client.get_orders_by_path(acct_id, 5, "2022-01-01", "2022-12-31", "FILLED")?;
    let res_json = serde_json::from_str::<Vec<Order>>(&res)?;

    for order in res_json {
        if let Order::SimpleOrder(simple_order) = order {
            println!("{}", simple_order.status);
        } else if let Order::ComplexOrder(complex_order) = order {
            println!("{}", complex_order.status);
        }
    }

    // Get Orders By Query
    let res = client.get_orders_by_query(None, 5, "2022-01-01", "2022-12-31", "FILLED")?;
    let res_json = serde_json::from_str::<Vec<Order>>(&res)?;

    for order in res_json {
        if let Order::SimpleOrder(simple_order) = order {
            println!("{}", simple_order.status);
        } else if let Order::ComplexOrder(complex_order) = order {
            println!("{}", complex_order.status);
        }
    }

    // Place Order
    place_order(&client, acct_id)?; // Limit Order For 1 Share Of AAPL At $1000

    // Replace Order
    let replace_order_id = 0;
    replace_order(&client, acct_id, replace_order_id)?; // Replace Existing Order For 1 Share Of AAPL At $1000

    // Cancel Order
    let cancel_order_id = 0;
    client.cancel_order(acct_id, cancel_order_id)?;

    // Get Saved Order
    let saved_order_id = 0;
    let res = client.get_saved_order(acct_id, saved_order_id)?;
    let res_json = serde_json::from_str::<Order>(&res)?;

    if let Order::SimpleSavedOrder(simple_saved_order) = res_json {
        println!("{}", simple_saved_order.session);
    }

    // Get Saved Order By Path
    let res = client.get_saved_orders_by_path(acct_id)?;
    let res_json = serde_json::from_str::<Vec<Order>>(&res)?;

    for order in res_json {
        if let Order::SimpleSavedOrder(simple_saved_order) = order {
            println!("{}", simple_saved_order.session);
        }
    }

    // Create Saved Order
    create_saved_order(&client, acct_id)?; // Create Saved Limit Order For 1 Share Of AAPL At $1000

    // Replace Saved Order
    let replace_order_id = 0;
    replace_saved_order(&client, acct_id, replace_order_id)?; // Replace Existing Saved Order For 1 Share Of AAPL At $1000

    // Delete Saved Order
    let delete_order_id = 0;
    client.delete_saved_order(acct_id, delete_order_id)?;

    Ok(())
}

fn place_order(client: &SyncTDAClient, acct_id: i64) -> Result<(), TDAClientError> {
    let order_spec = equity_buy_limit("AAPL", 1.0, 1000.0).build()?;

    /*
      Translates to this
    {
      "session": "NORMAL",
      "duration": "DAY",
      "orderType": "LIMIT",
      "price": 1000.0,
      "orderLegCollection": [
        {
          "instruction": "BUY",
          "instrument": {
            "assetType": "EQUITY",
            "symbol": "AAPL"
          },
          "quantity": 1.0
        }
      ],
      "orderStrategyType": "SINGLE"
    }
    */

    client.place_order(acct_id, &order_spec)?;

    Ok(())
}

fn replace_order(
    client: &SyncTDAClient,
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

fn create_saved_order(client: &SyncTDAClient, acct_id: i64) -> Result<(), TDAClientError> {
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
    client: &SyncTDAClient,
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
