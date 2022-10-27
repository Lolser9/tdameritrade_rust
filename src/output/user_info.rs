use serde::{Deserialize, Serialize};

/// Deserialize get_user_principals function output
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPrincipals {
    pub user_id: String,
    pub user_cd_domain_id: String,
    pub primary_account_id: String,
    pub last_login_time: String,
    pub token_expiration_time: String,
    pub login_time: String,
    pub access_level: String,
    pub stale_password: bool,
    pub streamer_info: Option<StreamerInfo>,
    pub professional_status: String,
    pub quotes: Quotes,
    pub streamer_subscription_keys: Option<StreamerSubscriptionKeys>,
    pub exchange_agreements: ExchangeAgreements,
    pub accounts: Vec<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerInfo {
    pub streamer_binary_url: String,
    pub streamer_socket_url: String,
    pub token: String,
    pub token_timestamp: String,
    pub user_group: String,
    pub access_level: String,
    pub acl: String,
    pub app_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotes {
    pub is_nyse_delayed: bool,
    pub is_nasdaq_delayed: bool,
    pub is_opra_delayed: bool,
    pub is_amex_delayed: bool,
    pub is_cme_delayed: bool,
    pub is_ice_delayed: bool,
    pub is_forex_delayed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerSubscriptionKeys {
    pub keys: Vec<Key>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeAgreements {
    #[serde(rename = "NASDAQ_EXCHANGE_AGREEMENT")]
    pub nasdaq_exchange_agreement: String,
    #[serde(rename = "NYSE_EXCHANGE_AGREEMENT")]
    pub nyse_exchange_agreement: String,
    #[serde(rename = "OPRA_EXCHANGE_AGREEMENT")]
    pub opra_exchange_agreement: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub display_name: String,
    pub account_cd_domain_id: String,
    pub company: String,
    pub segment: String,
    pub surrogate_ids: Option<SurrogateIds>,
    pub preferences: Option<Preferences>,
    pub acl: String,
    pub authorizations: Authorizations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurrogateIds {
    #[serde(rename = "SCARR")]
    pub scarr: String,
    #[serde(rename = "Market Edge")]
    pub market_edge: String,
    #[serde(rename = "Zacks")]
    pub zacks: String,
    #[serde(rename = "Localytics")]
    pub localytics: String,
    #[serde(rename = "Market Watch")]
    pub market_watch: String,
    #[serde(rename = "Flybits")]
    pub flybits: String,
    #[serde(rename = "BOZEL")]
    pub bozel: String,
    #[serde(rename = "WallStreetStrategies")]
    pub wall_street_strategies: String,
    #[serde(rename = "STS")]
    pub sts: String,
    #[serde(rename = "SiteCatalyst")]
    pub site_catalyst: String,
    #[serde(rename = "OpinionLab")]
    pub opinion_lab: String,
    #[serde(rename = "BriefingTrader")]
    pub briefing_trader: String,
    #[serde(rename = "WSOD")]
    pub wsod: String,
    #[serde(rename = "SP")]
    pub sp: String,
    #[serde(rename = "DART")]
    pub dart: String,
    #[serde(rename = "EF")]
    pub ef: String,
    #[serde(rename = "GK")]
    pub gk: String,
    pub e_pay: String,
    #[serde(rename = "VB")]
    pub vb: String,
    #[serde(rename = "Layer")]
    pub layer: String,
    #[serde(rename = "PWS")]
    pub pws: String,
    #[serde(rename = "Investools")]
    pub investools: String,
    #[serde(rename = "MIN")]
    pub min: String,
    #[serde(rename = "MGP")]
    pub mgp: String,
    #[serde(rename = "VCE")]
    pub vce: String,
    #[serde(rename = "HAVAS")]
    pub havas: String,
    #[serde(rename = "MSTAR")]
    pub mstar: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorizations {
    pub apex: bool,
    pub level_two_quotes: bool,
    pub stock_trading: bool,
    pub margin_trading: bool,
    pub streaming_news: bool,
    pub option_trading_level: String,
    pub streamer_access: bool,
    pub advanced_margin: bool,
    pub scottrade_account: bool,
    pub auto_position_effect: bool,
}
