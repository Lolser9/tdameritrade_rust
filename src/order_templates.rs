use crate::builders::OrderBuilder;

/// Equity buy market order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_buy_market(symbol: &str, quantity: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("MARKET")
        .set_session("NORMAL")
        .set_duration("DAY")
        .add_order_leg("BUY", "EQUITY", symbol, quantity)
}

/// Equity buy limit order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_buy_limit(symbol: &str, quantity: f64, price: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("LIMIT")
        .set_session("NORMAL")
        .set_duration("DAY")
        .set_price(price)
        .add_order_leg("BUY", "EQUITY", symbol, quantity)
}

/// Equity sell market order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_sell_market(symbol: &str, quantity: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("MARKET")
        .set_session("NORMAL")
        .set_duration("DAY")
        .add_order_leg("SELL", "EQUITY", symbol, quantity)
}

/// Equity sell limit order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_sell_limit(symbol: &str, quantity: f64, price: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("LIMIT")
        .set_session("NORMAL")
        .set_duration("DAY")
        .set_price(price)
        .add_order_leg("SELL", "EQUITY", symbol, quantity)
}

/// Equity sell short market order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_sell_short_market(symbol: &str, quantity: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("MARKET")
        .set_session("NORMAL")
        .set_duration("DAY")
        .add_order_leg("SELL_SHORT", "EQUITY", symbol, quantity)
}

/// Equity sell short limit order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_sell_short_limit(symbol: &str, quantity: f64, price: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("LIMIT")
        .set_session("NORMAL")
        .set_duration("DAY")
        .set_price(price)
        .add_order_leg("SELL_SHORT", "EQUITY", symbol, quantity)
}

/// Equity buy to cover market order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_buy_to_cover_market(symbol: &str, quantity: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("MARKET")
        .set_session("NORMAL")
        .set_duration("DAY")
        .add_order_leg("BUY_TO_COVER", "EQUITY", symbol, quantity)
}

/// Equity buy to cover limit order
///
/// - session: `NORMAL`
/// - duration: `DAY`
pub fn equity_buy_to_cover_limit(symbol: &str, quantity: f64, price: f64) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("SINGLE")
        .set_order_type("LIMIT")
        .set_session("NORMAL")
        .set_duration("DAY")
        .set_price(price)
        .add_order_leg("BUY_TO_COVER", "EQUITY", symbol, quantity)
}

/// OCO order
pub fn one_cancels_other(order1: OrderBuilder, order2: OrderBuilder) -> OrderBuilder {
    OrderBuilder::new()
        .set_order_strategy_type("OCO")
        .add_child_order_strategy(order1)
        .add_child_order_strategy(order2)
}

/// 1st Triggers 2nd Order
pub fn first_triggers_second(order1: OrderBuilder, order2: OrderBuilder) -> OrderBuilder {
    order1
        .set_order_strategy_type("TRIGGER")
        .add_child_order_strategy(order2)
}
