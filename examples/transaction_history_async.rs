use tdameritrade_rust::{output::transaction_history::Transaction, AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Account Id
    let acct_id = config::acct_id();

    // Get Transaction
    let transaction_id = 0;
    let res = client.get_transaction(acct_id, transaction_id).await?;
    let res_json = serde_json::from_str::<Transaction>(&res)?;
    println!("{}", res_json.transaction_item.cost);

    // Get Transactions
    let res = client
        .get_transactions(acct_id, "ALL", Some("AAPL"), "2022-01-01", "2022-12-31")
        .await?;
    let res_json = serde_json::from_str::<Vec<Transaction>>(&res)?;

    for transaction in res_json {
        println!("{}", transaction.transaction_item.cost);
    }

    Ok(())
}
