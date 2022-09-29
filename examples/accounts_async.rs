use serde_json::Value;
use tdameritrade_rust::{AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Account
    let fields = vec!["orders", "positions"];
    let res = client.get_account(acct_id, Some(&fields)).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get Accounts
    let res = client.get_accounts(None).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
