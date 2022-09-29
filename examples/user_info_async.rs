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

    // Get Streamer Subscription Keys
    let accounts = vec![acct_id];
    let res = client.get_streamer_subscription_keys(&accounts).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Get User Principles
    let fields = vec![
        "streamerSubscriptionKeys",
        "streamerConnectionInfo",
        "preferences",
        "surrogateIds",
    ];
    let res = client.get_user_principals(Some(&fields)).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    // Alternate Example
    let res = client.get_user_principals(None).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;
    println!("{}", res_json);

    Ok(())
}
