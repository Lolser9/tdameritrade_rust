use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub change: f64,
    pub description: String,
    pub direction: String,
    pub last: f64,
    pub symbol: String,
    pub total_volume: i64,
}
