use chrono::{Duration, Utc};
use serde_json::Value;
use tdameritrade_rust::{AsyncTDAClient, PriceHistoryParams, TDAClientError};
mod config;

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );

    // Get Price History With Period
    let price_history_params = PriceHistoryParams::default()
        .symbol("AAPL")
        .period_type("day")
        .period(10)
        .frequency_type("minute")
        .frequency(30)
        .need_extended_hours_data(false)
        .build()
        .expect("Failed To Build");

    let res = client.get_price_history(&price_history_params).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;

    // Returns 30 minute candles over a period of 10 Trading days without extended hours data
    println!("{}", res_json);

    // Get Price History With Start And End Time
    let current_utc_time = Utc::now();
    let past_utc_time = current_utc_time - Duration::days(30);

    let price_history_params = PriceHistoryParams::default()
        .symbol("AAPL")
        .period_type("day")
        .frequency_type("minute")
        .frequency(30)
        .start_date(past_utc_time.timestamp_millis())
        .end_date(current_utc_time.timestamp_millis())
        .need_extended_hours_data(false)
        .build()
        .expect("Failed To Build");

    let res = client.get_price_history(&price_history_params).await?;
    let res_json = serde_json::from_str::<Value>(&res)?;

    // Returns 30 minute candles over a period of 30 days without extended hours data
    println!("{}", res_json);

    Ok(())
}
