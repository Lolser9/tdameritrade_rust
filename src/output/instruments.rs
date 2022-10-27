use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruments {
    #[serde(flatten)]
    pub instruments: HashMap<String, InstrumentData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentData {
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub exchange: String,
    pub asset_type: String,
}
