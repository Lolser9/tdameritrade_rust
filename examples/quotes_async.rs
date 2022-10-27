use tdameritrade_rust::{
    output::quotes::{QuoteType, Quotes},
    AsyncTDAClient, TDAClientError,
};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Quote
    let symbol = "AAPL";
    let res = client.get_quote(symbol).await?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
        println!("{}", equity.close_price);
    }

    // Get Quotes
    let symbols = vec!["AAPL", "AMZN", "AMD", "NVDA"];
    let res = client.get_quotes(&symbols).await?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    for symbol in symbols.into_iter() {
        if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
            println!("{}", equity.close_price)
        }
    }

    Ok(())
}
