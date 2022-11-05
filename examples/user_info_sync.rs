use tdameritrade_rust::{
    output::user_info::{StreamerSubscriptionKeys, UserPrincipals},
    SyncTDAClient, TDAClientError,
};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let mut client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Streamer Subscription Keys
    let accounts = vec![acct_id];
    let res = client.get_streamer_subscription_keys(&accounts)?;
    let res_json = serde_json::from_str::<StreamerSubscriptionKeys>(&res)?;

    for key in res_json.keys {
        println!("{}", key.key);
    }

    // Get User Principles
    let fields = vec![
        "streamerSubscriptionKeys",
        "streamerConnectionInfo",
        "preferences",
        "surrogateIds",
    ];
    let res = client.get_user_principals(Some(&fields))?;
    let res_json = serde_json::from_str::<UserPrincipals>(&res)?;
    println!("{:?}", res_json.streamer_subscription_keys.unwrap().keys);

    // Alternate Example
    let res = client.get_user_principals(None)?;
    let res_json = serde_json::from_str::<UserPrincipals>(&res)?;
    println!("{}", res_json.user_id);

    Ok(())
}
