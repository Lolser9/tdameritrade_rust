use tdameritrade_rust::{output::transaction_history::Transaction, SyncTDAClient, TDAClientError};
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

    // Get Transaction
    let transaction_id = 0;
    let res = client.get_transaction(acct_id, transaction_id)?;
    let res_json = serde_json::from_str::<Transaction>(&res)?;
    println!("{}", res_json.transaction_item.cost);

    // Get Transactions
    let res = client.get_transactions(acct_id, "ALL", Some("AAPL"), "2021-01-01", "2021-12-31")?;
    let res_json = serde_json::from_str::<Vec<Transaction>>(&res)?;

    for transaction in res_json {
        println!("{}", transaction.transaction_item.cost);
    }

    Ok(())
}
