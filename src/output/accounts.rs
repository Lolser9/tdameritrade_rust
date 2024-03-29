use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub securities_account: SecuritiesAccount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub account_id: String,
    pub round_trips: i64,
    pub is_day_trader: bool,
    pub is_closing_only_restricted: bool,
    pub positions: Option<Vec<Position>>,
    pub order_strategies: Option<Vec<OrderStrategy>>,
    pub initial_balances: InitialBalances,
    pub current_balances: CurrentBalances,
    pub projected_balances: ProjectedBalances,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: f64,
    pub average_price: f64,
    pub current_day_cost: f64,
    pub current_day_profit_loss: f64,
    pub current_day_profit_loss_percentage: f64,
    pub long_quantity: f64,
    pub settled_long_quantity: f64,
    pub settled_short_quantity: f64,
    pub instrument: Instrument,
    pub market_value: f64,
    pub maintenance_requirement: f64,
    pub previous_session_long_quantity: Option<f64>,
    pub previous_sessions_short_quantity: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStrategy {
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
    pub order_strategy_type: Option<String>,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub tag: String,
    pub account_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: String,
    pub leg_id: i64,
    pub instrument: Instrument,
    pub instruction: String,
    pub position_effect: String,
    pub quantity: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub asset_type: String,
    pub cusip: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialBalances {
    pub accrued_interest: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub bond_value: f64,
    pub cash_receipts: f64,
    pub liquidation_value: f64,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub is_in_call: bool,
    pub unsettled_cash: f64,
    pub cash_debit_call_value: f64,
    pub pending_deposits: f64,
    pub account_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBalances {
    pub accrued_interest: f64,
    pub cash_balance: f64,
    pub cash_receipts: f64,
    pub long_option_market_value: f64,
    pub liquidation_value: f64,
    pub long_market_value: f64,
    pub money_market_fund: f64,
    pub savings: f64,
    pub short_market_value: f64,
    pub pending_deposits: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_call: f64,
    pub long_non_marginable_market_value: f64,
    pub total_cash: f64,
    pub short_option_market_value: f64,
    pub mutual_fund_value: f64,
    pub bond_value: f64,
    pub cash_debit_call_value: f64,
    pub unsettled_cash: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedBalances {
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
}
