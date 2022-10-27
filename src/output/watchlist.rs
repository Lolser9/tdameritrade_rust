use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
    pub name: String,
    pub watchlist_id: String,
    pub account_id: String,
    pub watchlist_items: Vec<WatchlistData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistData {
    pub sequence_id: i64,
    pub quantity: f64,
    pub average_price: f64,
    pub commission: f64,
    pub instrument: Instrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub symbol: String,
    pub asset_type: String,
}
