use serde_json::Value;
use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Option<String>,
    pub strategy: String,
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub interest_rate: f64,
    pub underlying_price: f64,
    pub volatility: f64,
    pub days_to_expiration: f64,
    pub number_of_contracts: i64,
    pub put_exp_date_map: ExpDateMap,
    pub call_exp_date_map: ExpDateMap,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpDateMap {
    #[serde(flatten)]
    pub data: HashMap<String, HashMap<String, Vec<OptionData>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionData {
    pub put_call: String,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub bid_ask_size: String,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: Option<String>,
    pub trade_time_in_long: i64,
    pub quote_time_in_long: i64,
    pub net_change: f64,
    #[serde(deserialize_with = "ok_or_none")]
    pub volatility: Option<f64>,
    #[serde(deserialize_with = "ok_or_none")]
    pub delta: Option<f64>,
    #[serde(deserialize_with = "ok_or_none")]
    pub gamma: Option<f64>,
    #[serde(deserialize_with = "ok_or_none")]
    pub theta: Option<f64>,
    pub vega: f64,
    #[serde(deserialize_with = "ok_or_none")]
    pub rho: Option<f64>,
    pub open_interest: i64,
    pub time_value: f64,
    #[serde(deserialize_with = "ok_or_none")]
    pub theoretical_option_value: Option<f64>,
    pub theoretical_volatility: f64,
    pub option_deliverables_list: Option<OptionDeliverables>,
    pub strike_price: f64,
    pub expiration_date: i64,
    pub days_to_expiration: i64,
    pub expiration_type: String,
    pub last_trading_day: i64,
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: Option<String>,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub intrinsic_value: f64,
    pub penny_pilot: bool,
    pub non_standard: bool,
    pub in_the_money: bool,
    pub mini: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverables {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: String,
    pub currency_type: String,
}

// Converts NaN values to None
fn ok_or_none<'a, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'a>,
    T: Deserialize<'a>,
{
    let v = Value::deserialize(deserializer)?;
    Ok(T::deserialize(v).ok())
}
