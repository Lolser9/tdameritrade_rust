use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuoteType {
    Equity(EquityQuote),
    MutualFund(MutualFundQuote),
    Future(FutureQuote),
    FutureOption(FutureOptionQuote),
    Index(IndexQuote),
    Option(OptionQuote),
    Forex(ForexQuote),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quotes {
    #[serde(flatten)]
    pub symbol: HashMap<String, QuoteType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityQuote {
    pub asset_type: String,
    pub asset_main_type: String,
    pub cusip: String,
    pub asset_sub_type: String,
    pub symbol: String,
    pub description: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub bid_id: String,
    pub ask_price: f64,
    pub ask_size: i64,
    pub ask_id: String,
    pub last_price: f64,
    pub last_size: i64,
    pub last_id: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub bid_tick: String,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: f64,
    pub exchange: String,
    pub exchange_name: String,
    pub marginable: bool,
    pub shortable: bool,
    pub volatility: f64,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: f64,
    #[serde(rename = "nAV")]
    pub n_av: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub regular_market_last_price: f64,
    pub regular_market_last_size: i64,
    pub regular_market_net_change: f64,
    pub regular_market_trade_time_in_long: i64,
    pub net_percent_change_in_double: f64,
    pub mark_change_in_double: f64,
    pub mark_percent_change_in_double: f64,
    pub regular_market_percent_change_in_double: f64,
    pub delayed: bool,
    pub realtime_entitled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundQuote {
    pub asset_type: String,
    pub asset_main_type: String,
    pub cusip: String,
    pub asset_sub_type: String,
    pub symbol: String,
    pub description: String,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub trade_time_in_long: i64,
    pub exchange: String,
    pub exchange_name: String,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: f64,
    #[serde(rename = "nAV")]
    pub n_av: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub net_percent_change_in_double: f64,
    pub delayed: bool,
    pub realtime_entitled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureQuote {
    pub symbol: String,
    pub bid_price_in_double: i64,
    pub ask_price_in_double: i64,
    pub last_price_in_double: i64,
    pub bid_id: String,
    pub ask_id: String,
    pub high_price_in_double: i64,
    pub low_price_in_double: i64,
    pub close_price_in_double: i64,
    pub exchange: String,
    pub description: String,
    pub last_id: String,
    pub open_price_in_double: i64,
    pub change_in_double: i64,
    pub future_percent_change: i64,
    pub exchange_name: String,
    pub security_status: String,
    pub open_interest: i64,
    pub mark: i64,
    pub tick: i64,
    pub tick_amount: i64,
    pub product: String,
    pub future_price_format: String,
    pub future_trading_hours: String,
    pub future_is_tradable: bool,
    pub future_multiplier: i64,
    pub future_is_active: bool,
    pub future_settlement_price: i64,
    pub future_active_symbol: String,
    pub future_expiration_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureOptionQuote {
    pub symbol: String,
    pub bid_price_in_double: i64,
    pub ask_price_in_double: i64,
    pub last_price_in_double: i64,
    pub high_price_in_double: i64,
    pub low_price_in_double: i64,
    pub close_price_in_double: i64,
    pub description: String,
    pub open_price_in_double: i64,
    pub net_change_in_double: i64,
    pub open_interest: i64,
    pub exchange_name: String,
    pub security_status: String,
    pub volatility: i64,
    pub money_intrinsic_value_in_double: i64,
    pub multiplier_in_double: i64,
    pub digits: i64,
    pub strike_price_in_double: i64,
    pub contract_type: String,
    pub underlying: String,
    pub time_value_in_double: i64,
    pub delta_in_double: i64,
    pub gamma_in_double: i64,
    pub theta_in_double: i64,
    pub vega_in_double: i64,
    pub rho_in_double: i64,
    pub mark: i64,
    pub tick: i64,
    pub tick_amount: i64,
    pub future_is_tradable: bool,
    pub future_trading_hours: String,
    pub future_percent_change: i64,
    pub future_is_active: bool,
    pub future_expiration_date: i64,
    pub expiration_type: String,
    pub exercise_type: String,
    pub in_the_money: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexQuote {
    pub symbol: String,
    pub description: String,
    pub last_price: i64,
    pub open_price: i64,
    pub high_price: i64,
    pub low_price: i64,
    pub close_price: i64,
    pub net_change: i64,
    pub total_volume: i64,
    pub trade_time_in_long: i64,
    pub exchange: String,
    pub exchange_name: String,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: i64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: i64,
    pub security_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionQuote {
    pub symbol: String,
    pub description: String,
    pub bid_price: i64,
    pub bid_size: i64,
    pub ask_price: i64,
    pub ask_size: i64,
    pub last_price: i64,
    pub last_size: i64,
    pub open_price: i64,
    pub high_price: i64,
    pub low_price: i64,
    pub close_price: i64,
    pub net_change: i64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: i64,
    pub open_interest: i64,
    pub volatility: i64,
    pub money_intrinsic_value: i64,
    pub multiplier: i64,
    pub strike_price: i64,
    pub contract_type: String,
    pub underlying: String,
    pub time_value: i64,
    pub deliverables: String,
    pub delta: i64,
    pub gamma: i64,
    pub theta: i64,
    pub vega: i64,
    pub rho: i64,
    pub security_status: String,
    pub theoretical_option_value: i64,
    pub underlying_price: i64,
    pub uv_expiration_type: String,
    pub exchange: String,
    pub exchange_name: String,
    pub settlement_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexQuote {
    pub symbol: String,
    pub bid_price_in_double: i64,
    pub ask_price_in_double: i64,
    pub last_price_in_double: i64,
    pub high_price_in_double: i64,
    pub low_price_in_double: i64,
    pub close_price_in_double: i64,
    pub exchange: String,
    pub description: String,
    pub open_price_in_double: i64,
    pub change_in_double: i64,
    pub percent_change: i64,
    pub exchange_name: String,
    pub digits: i64,
    pub security_status: String,
    pub tick: i64,
    pub tick_amount: i64,
    pub product: String,
    pub trading_hours: String,
    pub is_tradable: bool,
    pub market_maker: String,
    #[serde(rename = "52WkHighInDouble")]
    pub n52wk_high_in_double: i64,
    #[serde(rename = "52WkLowInDouble")]
    pub n52wk_low_in_double: i64,
    pub mark: i64,
}
