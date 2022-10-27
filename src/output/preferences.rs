use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub express_trading: bool,
    pub direct_options_routing: bool,
    pub direct_equity_routing: bool,
    pub default_equity_order_leg_instruction: String,
    pub default_equity_order_type: String,
    pub default_equity_order_price_link_type: String,
    pub default_equity_order_duration: String,
    pub default_equity_order_market_session: String,
    pub default_equity_quantity: i64,
    pub mutual_fund_tax_lot_method: String,
    pub option_tax_lot_method: String,
    pub equity_tax_lot_method: String,
    pub default_advanced_tool_launch: String,
    pub auth_token_timeout: String,
}
