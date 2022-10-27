use chrono::{Duration, Utc};
use tdameritrade_rust::{
    output::price_history::PriceHistory, AsyncTDAClient, PriceHistoryParams, TDAClientError,
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

    // Returns 30 Minute Candles Over A Period Of 10 Trading Days Without Extended Hours Data
    let res = client.get_price_history(&price_history_params).await?;
    let res_json = serde_json::from_str::<PriceHistory>(&res)?;

    // Print Close Value In Every Candle
    for candle in res_json.candles {
        println!("{}", candle.close)
    }

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

    // Returns 30 Minute Candles Over A Period Of 30 Days Without Extended Hours Data
    let res = client.get_price_history(&price_history_params).await?;
    let res_json = serde_json::from_str::<PriceHistory>(&res)?;

    // Print Close Value In Every Candle
    for candle in res_json.candles {
        println!("{}", candle.close)
    }

    Ok(())
}
