use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketHours {
    pub equity: std::option::Option<InnerEquity>,
    pub option: std::option::Option<InnerOption>,
    pub future: std::option::Option<InnerFuture>,
    pub bond: std::option::Option<InnerBond>,
    pub forex: std::option::Option<InnerForex>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerEquity {
    #[serde(rename = "EQ")]
    pub equity_data: EquityData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityData {
    pub date: String,
    pub market_type: String,
    pub exchange: String,
    pub category: String,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: EquitySessionHours,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquitySessionHours {
    pub pre_market: Vec<PreMarket>,
    pub regular_market: Vec<RegularMarket>,
    pub post_market: Vec<PostMarket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerOption {
    #[serde(rename = "EQO")]
    pub eqo: Eqo,
    #[serde(rename = "IND")]
    pub ind: Ind,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eqo {
    pub date: String,
    pub market_type: String,
    pub exchange: String,
    pub category: String,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: OptionSessionHours,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ind {
    pub date: String,
    pub market_type: String,
    pub exchange: String,
    pub category: String,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: OptionSessionHours,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionSessionHours {
    pub regular_market: Vec<RegularMarket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerFuture {
    #[serde(flatten)]
    pub futures: HashMap<String, FutureData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureData {
    pub date: String,
    pub market_type: String,
    pub exchange: String,
    pub category: String,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: FutureSessionHours,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureSessionHours {
    pub pre_market: std::option::Option<Vec<PreMarket>>,
    pub regular_market: Vec<RegularMarket>,
    pub outcry_market: std::option::Option<Vec<OutcryMarket>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcryMarket {
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerBond {
    #[serde(rename = "BON")]
    pub bond_data: BondData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondData {
    pub date: String,
    pub market_type: String,
    pub exchange: String,
    pub category: String,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: BondSessionHours,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondSessionHours {
    pub pre_market: Vec<PreMarket>,
    pub regular_market: Vec<RegularMarket>,
    pub post_market: Vec<PostMarket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerForex {
    #[serde(rename = "forex")]
    pub forex_data: ForexData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexData {
    pub date: String,
    pub market_type: String,
    pub exchange: std::option::Option<String>,
    pub category: std::option::Option<String>,
    pub product: String,
    pub product_name: std::option::Option<String>,
    pub is_open: bool,
    pub session_hours: std::option::Option<String>, // Forex doesn't return session hours
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreMarket {
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarket {
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMarket {
    pub start: String,
    pub end: String,
}
