use tdameritrade_rust::{
    output::instruments::{InstrumentData, Instruments},
    SyncTDAClient, TDAClientError,
};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Search Instruments
    let symbol = "AAPL";
    let res = client.search_instruments(symbol, "symbol-search")?; // Returns Data For Symbol AAPL
    let res_json = serde_json::from_str::<Instruments>(&res)?;
    println!("{}, {}", symbol, res_json.instruments[symbol].asset_type);

    // Alternate Example
    let res = client.search_instruments("A.*", "symbol-regex")?; // Returns Data For All Symbols That Start With Letter A
    let res_json = serde_json::from_str::<Instruments>(&res)?;

    for (symbol, instrument_data) in res_json.instruments.into_iter() {
        println!("{}, {}", symbol, instrument_data.asset_type);
    }

    // Get Instrument
    let msft_cusip = "594918104";
    let res = client.get_instrument(msft_cusip)?;
    let res_json = serde_json::from_str::<Vec<InstrumentData>>(&res)?;
    println!("{}, {}", res_json[0].symbol, res_json[0].asset_type);

    Ok(())
}
