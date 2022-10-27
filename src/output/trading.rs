use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Order {
    SimpleOrder(SimpleOrder),
    ComplexOrder(ComplexOrder),
    SimpleSavedOrder(SimpleSavedOrder),
    Other(Value),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleOrder {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub complex_order_strategy_type: String,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: String,
    pub destination_link_name: String,
    pub price: Option<f64>,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub order_strategy_type: String,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub close_time: String,
    pub tag: String,
    pub account_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplexOrder {
    pub order_strategy_type: String,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub close_time: String,
    pub tag: Option<String>,
    pub account_id: i64,
    pub child_order_strategies: Vec<ChildOrderStrategy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSavedOrder {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub complex_order_strategy_type: String,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub order_strategy_type: String,
    pub cancelable: bool,
    pub editable: bool,
    pub saved_order_id: i64,
    pub saved_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildOrderStrategy {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub cancel_time: Option<String>,
    pub complex_order_strategy_type: String,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: String,
    pub destination_link_name: String,
    pub price: f64,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub order_strategy_type: String,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub close_time: String,
    pub tag: String,
    pub account_id: i64,
    #[serde(default)]
    pub order_activity_collection: Vec<OrderActivityCollection>,
    pub stop_price: Option<f64>,
    pub stop_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: String,
    pub leg_id: i64,
    pub instrument: Instrument,
    pub instruction: String,
    pub position_effect: Option<String>,
    pub quantity: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub asset_type: String,
    pub cusip: Option<String>,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivityCollection {
    pub activity_type: String,
    pub activity_id: i64,
    pub execution_type: String,
    pub quantity: f64,
    pub order_remaining_quantity: f64,
    pub execution_legs: Vec<ExecutionLeg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: i64,
    pub quantity: f64,
    pub mismarked_quantity: f64,
    pub price: f64,
    pub time: String,
}
