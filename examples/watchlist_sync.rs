use tdameritrade_rust::{SyncTDAClient, TDAClientError};
mod config;
use serde_json::Value;

fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Watchlists For Multiple Accounts
    let res = client.get_watchlists_for_multiple_accounts()?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Watchlists For Single Account
    let res = client.get_watchlists_for_single_account(acct_id)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Watchlist
    let get_watchlist_id = 0;
    let res = client.get_watchlist(acct_id, get_watchlist_id)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Create Watchlist
    create_watchlist(&mut client, acct_id)?; // Creates Watchlist With Symbols AAPL and AMZN

    // Replace Watchlist
    let replace_watchlist_id = 0;
    replace_watchlist(&mut client, acct_id, replace_watchlist_id)?; // Replaces Watchlist With Symbols AMD and NVDA

    // Update Watchlist
    let update_watchlist_id = 0;
    update_watchlist(&mut client, acct_id, update_watchlist_id)?; // Adds Symbols AAPL and AMZN on existing watchlist

    // Delete Watchlist
    let delete_watchlist_id = 0;
    client.delete_watchlist(acct_id, delete_watchlist_id)?;

    Ok(())
}

fn create_watchlist(client: &mut SyncTDAClient, acct_id: i64) -> Result<(), TDAClientError> {
    // Create Watchlist Spec
    let create_spec = r#"{
        "name": "Created Watchlist",
        "watchlistItems": [
            {
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {
                    "symbol": "AAPL",
                    "assetType": "EQUITY"
                }
            },
            {
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {
                    "symbol": "AMZN",
                    "assetType": "EQUITY"
                }
            }
        ]
    }"#;

    client.create_watchlist(acct_id, create_spec)?;

    Ok(())
}

fn replace_watchlist(
    client: &mut SyncTDAClient,
    acct_id: i64,
    watchlist_id: i64,
) -> Result<(), TDAClientError> {
    let replace_spec = format!(
        r#"{{
        "name": "Replaced Watchlist",
        "watchlistId": "{}",
        "watchlistItems": [
            {{
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {{
                    "symbol": "AMD",
                    "assetType": "EQUITY"
                }}
            }},
            {{
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {{
                    "symbol": "NVDA",
                    "assetType": "EQUITY"
                }}
            }}
        ]
    }}"#,
        watchlist_id
    );

    client.replace_watchlist(acct_id, watchlist_id, &replace_spec)?;

    Ok(())
}

fn update_watchlist(
    client: &mut SyncTDAClient,
    acct_id: i64,
    watchlist_id: i64,
) -> Result<(), TDAClientError> {
    let update_spec = format!(
        r#"{{
        "name": "Updated Watchlist",
        "watchlistId": "{}",
        "watchlistItems": [
            {{
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {{
                    "symbol": "AAPL",
                    "assetType": "EQUITY"
                }}
            }},
            {{
                "quantity": 0,
                "averagePrice": 0,
                "commission": 0,
                "purchasedDate": "2022-01-01",
                "instrument": {{
                    "symbol": "AMZN",
                    "assetType": "EQUITY"
                }}
            }}
        ]
    }}"#,
        watchlist_id
    );

    client.update_watchlist(acct_id, watchlist_id, &update_spec)?;

    Ok(())
}
