use crate::TDAClientError;
use derive_builder::Builder;
use serde::Serialize;

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

/// Builder to streamline creation of TD Ameritrade orders
///
/// Parameters
/// - session: The market session when order should be executed. Valid sessions are `NORMAL`, `AM`, `PM`, or `SEAMLESS`
/// - duration: Length of time order will be active. Valid durations are `DAY`, `GOOD_TILL_CANCEL`, or `FILL_OR_KILL`
/// - order_type: Type of order. Valid order types are `MARKET`, `LIMIT`, `STOP`, `STOP_LIMIT`, `TRAILING_STOP`, `MARKET_ON_CLOSE`, `EXERCISE`, `TRAILING_STOP_LIMIT`, `NET_DEBIT`, `NET_CREDIT`, or `NET_ZERO`
/// - complex_order_strategy_type: Strategy type for complex orders. Valid complex strategy types are `NONE`, `COVERED`, `VERTICAL`, `BACK_RATIO`, `CALENDAR`, `DIAGONAL`, `STRADDLE`, `STRANGLE`, `COLLAR_SYNTHETIC`, `BUTTERFLY`, `CONDOR`, `IRON_CONDOR`, `VERTICAL_ROLL`, `COLLAR_WITH_STOCK`, `DOUBLE_DIAGONAL`, `UNBALANCED_BUTTERFLY`, `UNBALANCED_CONDOR`, `UNBALANCED_IRON_CONDOR`, `UNBALANCED_VERTICAL_ROLL`, or `CUSTOM`
/// - quantity: Number of shares for the order. Accepts `f64`
/// - requested_destination: Preferred order destination. Valid destinations are `INET`, `ECN_ARCA`, `CBOE`, `AMEX`, `PHLX`, `ISE`, `BOX`, `NYSE`, `NASDAQ`, `BATS`, `C2`, or `AUTO`
/// - stop_price: The stop price. Accepts `f64`
/// - stop_price_link_basis: The stop price link basis. Valid stop price link baseis are `MANUAL`, `BASE`, `TRIGGER`, `LAST`, `BID`, `ASK`, `ASK_BID`, `MARK`, or `AVERAGE`
/// - stop_price_link_type: The stop price link type. Valid stop price link types are `VALUE`, `PERCENT`, or `TICK`
/// - stop_price_offset: The stop price offset. Accepts `f64`
/// - stop_type: The stop type. Valid stop types are `STANDARD`, `BID`, `ASK`, `LAST`, or `MARK`
/// - price_link_basis: The price link basis. Valid price link baseis are `MANUAL`, `BASE`, `TRIGGER`, `LAST`, `BID`, `ASK`, `ASK_BID`, `MARK`, or `AVERAGE`
/// - price_link_type: The price link type. Valid price link types are `VALUE`, `PERCENT`, or `TICK`
/// - price: Order price. Accepts `f64`
/// - order_leg_collection: Order Leg Collections
/// - activation_price: The activation price. Accepts `f64`
/// - special_instruction: Special instruction for order. Valid instructions are `ALL_OR_NONE`, `DO_NOT_REDUCE`, or `ALL_OR_NONE_DO_NOT_REDUCE`
/// - order_strategy_type: Strategy type for composite orders. Valid strategy types are `SINGLE`, `OCO`, or `TRIGGER`
/// - child_order_strategies: Order strategies for composite orders. Accepts `OrderBuilder`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    complex_order_strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_link_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_link_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    order_leg_collection: Vec<OrderLeg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    child_order_strategies: Vec<OrderBuilder>,
}

#[derive(Debug, Serialize)]
pub struct OrderLeg {
    instruction: String,
    instrument: Instrument,
    quantity: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    asset_type: String,
    symbol: String,
}

impl OrderBuilder {
    pub fn new() -> OrderBuilder {
        OrderBuilder {
            session: None,
            duration: None,
            order_type: None,
            complex_order_strategy_type: None,
            quantity: None,
            requested_destination: None,
            stop_price: None,
            stop_price_link_basis: None,
            stop_price_link_type: None,
            stop_price_offset: None,
            stop_type: None,
            price_link_basis: None,
            price_link_type: None,
            price: None,
            order_leg_collection: Vec::new(),
            activation_price: None,
            special_instruction: None,
            order_strategy_type: None,
            child_order_strategies: Vec::new(),
        }
    }

    // Session
    pub fn set_session(mut self, session: &str) -> OrderBuilder {
        self.session = Some(session.to_string());
        self
    }

    pub fn clear_session(mut self) -> OrderBuilder {
        self.session = None;
        self
    }

    // Duration
    pub fn set_duration(mut self, duration: &str) -> OrderBuilder {
        self.duration = Some(duration.to_string());
        self
    }

    pub fn clear_duration(mut self) -> OrderBuilder {
        self.duration = None;
        self
    }

    // Order Type
    pub fn set_order_type(mut self, order_type: &str) -> OrderBuilder {
        self.order_type = Some(order_type.to_string());
        self
    }

    pub fn clear_order_type(mut self) -> OrderBuilder {
        self.order_type = None;
        self
    }

    // Complex Order Strategy Type
    pub fn set_complex_order_strategy_type(
        mut self,
        complex_order_strategy_type: &str,
    ) -> OrderBuilder {
        self.complex_order_strategy_type = Some(complex_order_strategy_type.to_string());
        self
    }

    pub fn clear_complex_order_strategy_type(mut self) -> OrderBuilder {
        self.complex_order_strategy_type = None;
        self
    }

    // Quantity
    pub fn set_quantity(mut self, quantity: f64) -> OrderBuilder {
        self.quantity = Some(quantity);
        self
    }

    pub fn clear_quantity(mut self) -> OrderBuilder {
        self.quantity = None;
        self
    }

    // Requested Destination
    pub fn set_requested_destination(mut self, requested_destination: &str) -> OrderBuilder {
        self.requested_destination = Some(requested_destination.to_string());
        self
    }

    pub fn clear_requested_destination(mut self) -> OrderBuilder {
        self.requested_destination = None;
        self
    }

    // Stop Price
    pub fn set_stop_price(mut self, stop_price: f64) -> OrderBuilder {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn clear_stop_price(mut self) -> OrderBuilder {
        self.stop_price = None;
        self
    }

    // Stop Price Link Basis
    pub fn set_stop_price_link_basis(mut self, stop_price_link_basis: &str) -> OrderBuilder {
        self.stop_price_link_basis = Some(stop_price_link_basis.to_string());
        self
    }

    pub fn clear_stop_price_link_basis(mut self) -> OrderBuilder {
        self.stop_price_link_basis = None;
        self
    }

    // Stop Price Link Type
    pub fn set_stop_price_link_type(mut self, stop_price_link_type: &str) -> OrderBuilder {
        self.stop_price_link_type = Some(stop_price_link_type.to_string());
        self
    }

    pub fn clear_stop_price_link_type(mut self) -> OrderBuilder {
        self.stop_price_link_type = None;
        self
    }

    // Stop Price Offset
    pub fn set_stop_price_offset(mut self, stop_price_offset: f64) -> OrderBuilder {
        self.stop_price_offset = Some(stop_price_offset);
        self
    }

    pub fn clear_stop_price_offset(mut self) -> OrderBuilder {
        self.stop_price_offset = None;
        self
    }

    // Stop Type
    pub fn set_stop_type(mut self, stop_type: &str) -> OrderBuilder {
        self.stop_type = Some(stop_type.to_string());
        self
    }

    pub fn clear_stop_type(mut self) -> OrderBuilder {
        self.stop_type = None;
        self
    }

    // Price Link Basis
    pub fn set_price_link_basis(mut self, price_link_basis: &str) -> OrderBuilder {
        self.price_link_basis = Some(price_link_basis.to_string());
        self
    }

    pub fn clear_price_link_basis(mut self) -> OrderBuilder {
        self.price_link_basis = None;
        self
    }

    // Price Link Type
    pub fn set_price_link_type(mut self, price_link_type: &str) -> OrderBuilder {
        self.price_link_type = Some(price_link_type.to_string());
        self
    }

    pub fn clear_price_link_type(mut self) -> OrderBuilder {
        self.price_link_type = None;
        self
    }

    // Price
    pub fn set_price(mut self, price: f64) -> OrderBuilder {
        self.price = Some(price);
        self
    }

    pub fn clear_price(mut self) -> OrderBuilder {
        self.price = None;
        self
    }

    // Activation Price
    pub fn set_activation_price(mut self, activation_price: f64) -> OrderBuilder {
        self.activation_price = Some(activation_price);
        self
    }

    pub fn clear_activation_price(mut self) -> OrderBuilder {
        self.activation_price = None;
        self
    }

    // Special Instruction
    pub fn set_special_instruction(mut self, special_instruction: &str) -> OrderBuilder {
        self.special_instruction = Some(special_instruction.to_string());
        self
    }

    pub fn clear_special_instruction(mut self) -> OrderBuilder {
        self.special_instruction = None;
        self
    }

    // Order Strategy Type
    pub fn set_order_strategy_type(mut self, order_strategy_type: &str) -> OrderBuilder {
        self.order_strategy_type = Some(order_strategy_type.to_string());
        self
    }

    pub fn clear_order_strategy_type(mut self) -> OrderBuilder {
        self.order_strategy_type = None;
        self
    }

    // Child Order Strategy
    pub fn add_child_order_strategy(mut self, child_order_strategy: OrderBuilder) -> OrderBuilder {
        self.child_order_strategies.push(child_order_strategy);
        self
    }

    pub fn clear_child_order_strategies(mut self) -> OrderBuilder {
        self.child_order_strategies = Vec::new();
        self
    }

    /// Add Order Leg
    ///
    /// Parameters
    /// - instruction: Type of trade. Valid instructions are `BUY`, `SELL`, `BUY_TO_COVER`, `SELL_SHORT`, `BUY_TO_OPEN`, `BUY_TO_CLOSE`, `SELL_TO_OPEN`, `SELL_TO_CLOSE` or `EXCHANGE`
    /// - instrument_asset_type: Type of asset traded. Valid asset types are `EQUITY`, `OPTION`, `INDEX`, `MUTUAL_FUND`, `CASH_EQUIVALENT`, `FIXED_INCOME`, or `CURRENCY`
    /// - instrument_symbol: Symbol traded. Accepts `&str`
    /// - quantity: Amount of shares traded. Accepts `f64`
    pub fn add_order_leg(
        mut self,
        instruction: &str,
        instrument_asset_type: &str,
        instrument_symbol: &str,
        quantity: f64,
    ) -> OrderBuilder {
        self.order_leg_collection.push(OrderLeg {
            instruction: instruction.to_string(),
            instrument: Instrument {
                asset_type: instrument_asset_type.to_string(),
                symbol: instrument_symbol.to_string(),
            },
            quantity,
        });
        self
    }

    pub fn clear_order_legs(mut self) -> OrderBuilder {
        self.order_leg_collection = Vec::new();
        self
    }

    // Build OrderBuilder
    pub fn build(self) -> Result<String, TDAClientError> {
        let built_order = OrderBuilder {
            session: self.session,
            duration: self.duration,
            order_type: self.order_type,
            complex_order_strategy_type: self.complex_order_strategy_type,
            quantity: self.quantity,
            requested_destination: self.requested_destination,
            stop_price: self.stop_price,
            stop_price_link_basis: self.stop_price_link_basis,
            stop_price_link_type: self.stop_price_link_type,
            stop_price_offset: self.stop_price_offset,
            stop_type: self.stop_type,
            price_link_basis: self.price_link_basis,
            price_link_type: self.price_link_type,
            price: self.price,
            order_leg_collection: self.order_leg_collection,
            activation_price: self.activation_price,
            special_instruction: self.special_instruction,
            order_strategy_type: self.order_strategy_type,
            child_order_strategies: self.child_order_strategies,
        };

        Ok(serde_json::to_string_pretty(&built_order)?)
    }
}
