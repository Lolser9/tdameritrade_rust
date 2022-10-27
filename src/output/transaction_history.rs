use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_account: String,
    pub settlement_date: String,
    pub net_amount: f64,
    pub transaction_date: String,
    pub transaction_sub_type: String,
    pub transaction_id: i64,
    pub cash_balance_effect_flag: bool,
    pub description: String,
    pub fees: Fees,
    pub transaction_item: TransactionItem,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub r_fee: f64,
    pub additional_fee: f64,
    pub cdsc_fee: f64,
    pub reg_fee: f64,
    pub other_charges: f64,
    pub commission: f64,
    pub opt_reg_fee: f64,
    pub sec_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionItem {
    pub account_id: i64,
    pub cost: f64,
    pub instrument: Option<Instrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub symbol: String,
    pub cusip: Option<String>,
    pub asset_type: String,
}
