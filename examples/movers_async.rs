use tdameritrade_rust::{output::movers::Mover, AsyncTDAClient, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Movers
    let res = client.get_movers("$DJI", "up", "percent").await?;
    let res_json = serde_json::from_str::<Vec<Mover>>(&res)?;

    for mover in res_json {
        println!("{}, {}", mover.symbol, mover.change);
    }

    // Alternate Example
    let res = client.get_movers("$COMPX", "down", "value").await?;
    let res_json = serde_json::from_str::<Vec<Mover>>(&res)?;

    for mover in res_json {
        println!("{}, {}", mover.symbol, mover.change);
    }

    Ok(())
}
