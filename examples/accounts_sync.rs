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

    // Get Account
    let fields = vec!["orders", "positions"];
    let res = client.get_account(acct_id, Some(&fields))?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Accounts
    let res = client.get_accounts(None)?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
