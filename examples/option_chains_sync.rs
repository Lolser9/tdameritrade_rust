use tdameritrade_rust::{
    output::option_chains::OptionChain, OptionChainParams, SyncTDAClient, TDAClientError,
};
mod config;

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let client = SyncTDAClient::new(
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    )?;

    // Get Option Chain
    let option_params = OptionChainParams::default()
        .symbol("AAPL")
        .contract_type("CALL")
        .strategy("SINGLE")
        .range("ITM")
        .expiration_month("OCT")
        .option_type("S")
        .build()
        .expect("Failed To Build");

    let res = client.get_option_chain(&option_params)?;
    let res_json = serde_json::from_str::<OptionChain>(&res)?;

    let date = res_json.call_exp_date_map.data.keys().next().unwrap();
    for (option, option_data) in &res_json.call_exp_date_map.data[date] {
        println!("{}, {}", option, option_data[0].close_price);
    }

    // Alternate Example
    let option_params = OptionChainParams::default()
        .symbol("AAPL")
        .build()
        .expect("Failed To Build");

    let res = client.get_option_chain(&option_params)?;
    let res_json = serde_json::from_str::<OptionChain>(&res)?;

    let date = res_json.call_exp_date_map.data.keys().next().unwrap();

    for (option, option_data) in &res_json.put_exp_date_map.data[date] {
        println!("{}, {:?}", option, option_data[0].close_price);
    }

    for (option, option_data) in &res_json.call_exp_date_map.data[date] {
        println!("{}, {:?}", option, option_data[0].close_price);
    }

    Ok(())
}
