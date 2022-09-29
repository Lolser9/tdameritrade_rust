use derive_builder::Builder;

#[derive(Debug, Builder, Default, Clone)]
#[builder(setter(into))]
#[builder(name = "OptionChainParams")]
pub struct OptionChain {
    pub symbol: String,
    #[builder(default = "\"ALL\".into()")]
    pub contract_type: String,
    #[builder(default = "-1")]
    pub strike_count: i64,
    #[builder(default = "false")]
    pub include_quotes: bool,
    #[builder(default = "\"SINGLE\".into()")]
    pub strategy: String,
    #[builder(default = "-1.0")]
    pub interval: f64,
    #[builder(default = "-1.0")]
    pub strike: f64,
    #[builder(default = "\"ALL\".into()")]
    pub range: String,
    #[builder(default)]
    pub from_date: String,
    #[builder(default)]
    pub to_date: String,
    #[builder(default = "-1.0")]
    pub volatility: f64,
    #[builder(default = "-1.0")]
    pub underlying_price: f64,
    #[builder(default = "-1.0")]
    pub interest_rate: f64,
    #[builder(default = "-1")]
    pub days_to_expiration: i64,
    #[builder(default = "\"ALL\".into()")]
    pub expiration_month: String,
    #[builder(default = "\"ALL\".into()")]
    pub option_type: String,
}

#[derive(Debug, Builder, Default, Clone)]
#[builder(setter(into))]
#[builder(name = "PriceHistoryParams")]
pub struct PriceHistory {
    pub symbol: String,
    pub period_type: String,
    #[builder(default = "-1")]
    pub period: i8,
    pub frequency_type: String,
    pub frequency: i8,
    #[builder(default = "-1")]
    pub start_date: i64,
    #[builder(default = "-1")]
    pub end_date: i64,
    #[builder(default = "true")]
    pub need_extended_hours_data: bool,
}
