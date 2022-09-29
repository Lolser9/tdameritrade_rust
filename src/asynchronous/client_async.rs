use crate::asynchronous::{auth_async::AsyncAuth, requests_async};
use crate::builders::{OptionChain, PriceHistory};
use crate::error::TDAClientError;
use itertools::Itertools;
use reqwest::Client;
use std::collections::HashMap;

pub struct AsyncTDAClient {
    reqwest_client: Client,
    auth: AsyncAuth,
}

impl AsyncTDAClient {
    /// Create New Asynchronous TDAClient
    pub fn new(client_id: String, redirect_uri: String, token_path: String) -> AsyncTDAClient {
        // Create Auth Client
        let auth: AsyncAuth = AsyncAuth::new(client_id, redirect_uri, token_path);

        // Create Reqwest Client
        let reqwest_client: Client = Client::new();

        // Create New AsyncTDAClient
        AsyncTDAClient {
            reqwest_client,
            auth,
        }
    }

    //// Accounts ////

    /// Account balances, positions, and orders for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - fields: Balances displayed by default. Valid fields are `positions` or `orders` (Optional)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D-0
    pub async fn get_account(
        &mut self,
        acct_id: i64,
        fields: Option<&Vec<&str>>,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameter
        if let Some(f) = fields {
            // Convert Vector To String
            let cleaned_fields: String = f.join(",");

            params.insert("fields".into(), cleaned_fields);
        }

        // Format URL
        let url: String = format!("accounts/{}", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Account balances, positions, and orders for all linked accounts
    ///
    /// Parameters
    /// - fields: Balances displayed by default. Valid fields are `positions` or `orders` (Optional)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts-0
    pub async fn get_accounts(
        &mut self,
        fields: Option<&Vec<&str>>,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameter
        if let Some(f) = fields {
            // Convert Vector To String
            let cleaned_fields: String = f.join(",");

            params.insert("fields".into(), cleaned_fields);
        }

        // Format URL
        let url: String = "accounts".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Instruments ////

    /// Search or retrieve instrument data, including fundamental data
    ///
    /// Parameters
    /// - symbol: Value to pass to the search. See projection description for more information
    /// - projection: The type of request
    ///     - symbol-search: Retrieve instrument data of a specific symbol or cusip
    ///     - symbol-regex: Retrieve instrument data for all symbols matching regex. Example: symbol=`XYZ.*` will return all symbols beginning with XYZ
    ///     - desc-search: Retrieve instrument data for instruments whose description contains the word supplied. Example: symbol=`FakeCompany` will return all instruments with FakeCompany in the description
    ///     - desc-regex: Search description with full regex support. Example: symbol=`XYZ.[A-C]` returns all instruments whose descriptions contain a word beginning with XYZ followed by a character A through C
    ///     - fundamental: Returns fundamental data for a single instrument specified by exact symbol.
    ///
    /// Official Documentation: https://developer.tdameritrade.com/instruments/apis/get/instruments
    pub async fn search_instruments(
        &mut self,
        symbol: &str,
        projection: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Parameters
        params.insert("symbol".into(), symbol.into());
        params.insert("projection".into(), projection.into());

        // Format URL
        let url: String = "instruments".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Get an instrument by CUSIP
    ///
    /// Parameters
    /// - cusip: CUSIP number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/instruments/apis/get/instruments/%7Bcusip%7D
    pub async fn get_instrument(&mut self, cusip: i64) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("instruments/{}", cusip);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Market Hours ////

    /// Retrieve market hours for specified markets
    ///
    /// Parameters
    /// - markets: The markets for which you're requesting market hours. Valid markets are `EQUITY`, `OPTION`, `FUTURE`, `BOND`, or `FOREX`
    /// - date: The date for which market hours information is requested. Valid ISO-8601 formats are `yyyy-MM-dd` or `yyyy-MM-dd'T'HH:mm:ssz`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/market-hours/apis/get/marketdata/hours
    pub async fn get_hours_for_multiple_markets(
        &mut self,
        markets: &Vec<&str>,
        date: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Convert Vector To String
        let cleaned_markets: String = markets.join(",");

        // Parameters
        params.insert("markets".into(), cleaned_markets);
        params.insert("date".into(), date.into());

        // Format URL
        let url: String = "marketdata/hours".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Retrieve market hours for specified single market
    ///
    /// Parameters
    /// - markets: The markets for which you're requesting market hours. Valid markets are `EQUITY`, `OPTION`, `FUTURE`, `BOND`, or `FOREX`
    /// - date: The date for which market hours information is requested. Valid ISO-8601 formats are `yyyy-MM-dd` or `yyyy-MM-dd'T'HH:mm:ssz`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/market-hours/apis/get/marketdata/%7Bmarket%7D/hours
    pub async fn get_hours_for_single_market(
        &mut self,
        market: &str,
        date: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Parameter
        params.insert("date".into(), date.into());

        // Format URL
        let url: String = format!("marketdata/{}/hours", market);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Movers ////

    /// Top 10 (up or down) movers by value or percent for a particular market
    ///
    /// Parameters
    /// - index: The index symbol to get movers from. Valid indexes are `$COMPX`, `$DJI`, or `$SPX.X`
    /// - direction: To return movers with the specified directions. Valid directions are `up` or `down`
    /// - change: To return movers with the specified change types. Valid change types are `percent` or `value`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/movers/apis/get/marketdata/%7Bindex%7D/movers
    pub async fn get_movers(
        &mut self,
        index: &str,
        direction: &str,
        change: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Parameters
        params.insert("direction".into(), direction.into());
        params.insert("change".into(), change.into());

        // Format URL
        let url: String = format!("marketdata/{}/movers", index);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Option Chains ////

    /// Get option chain for an optionable symbol
    ///
    /// Parameters
    /// - option_params: `OptionChain` object
    ///     - symbol: Enter one symbol (case-sensitive)
    ///     - contract_type: Type of contracts to return in the chain. Valid contract types are `CALL`, `PUT`, or `ALL`. Default is `ALL` (Optional)
    ///     - strike_count: The number of strikes to return above and below the at-the-money price (Optional)
    ///     - include_quotes: Include quotes for options in the option chain. Default is false (Optional)
    ///     - strategy: Passing a value returns a strategy chain. Valid strategy values are `SINGLE`, `ANALYTICAL` (allows use of the volatility, underlyingPrice, interestRate, and daysToExpiration params to calculate theoretical values), `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, or `ROLL`. Default is `SINGLE` (Optional)
    ///     - interval: Strike interval for spread strategy chains (Optional)
    ///     - strike: Provide a strike price to return options only at that strike price (Optional)
    ///     - range: Returns options for the given range. Valid ranges are `ITM` (In the money), `NTM` (Near the money), `OTM` (Out of the money), `SAK` (Strikes above market), `SBK` (Strikes below market), `SNK` (Strikes near market), or `ALL` (All strikes). Default is `ALL` (Optional)
    ///     - from_date: Only return expirations after this date. For strategies, expiration refers to the nearest term expiration in the strategy. Valid ISO-8601 formats are `yyyy-MM-dd` or `yyyy-MM-dd'T'HH:mm:ssz` (Optional)
    ///     - to_date: Only return expirations before this date. For strategies, expiration refers to the nearest term expiration in the strategy. Valid ISO-8601 formats are `yyyy-MM-dd` or `yyyy-MM-dd'T'HH:mm:ssz` (Optional)
    ///     - volatility: Volatility to use in calculations. Applies only to `ANALYTICAL` strategy chains (Optional)
    ///     - underlying_price: Underlying price to use in calculations. Applies only to `ANALYTICAL` strategy chains (Optional)
    ///     - interest_rate: Interest rate to use in calculations. Applies only to `ANALYTICAL` strategy chains (Optional)
    ///     - days_to_expiration: Days to expiration to use in calculations. Applies only to `ANALYTICAL` strategy chains (Optional)
    ///     - expiration_month: Return only options expiring in the specified month. Month is given in the three character format. Example: `JAN`. Default is `ALL` (Optional)
    ///     - option_type: Type of contracts to return. Valid option types are `S` (Standard contracts), `NS` (Non-standard contracts), or `ALL` (All contracts). Default is `ALL` (Optional)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/option-chains/apis/get/marketdata/chains
    pub async fn get_option_chain(
        &mut self,
        option_params: &OptionChain,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameters
        if option_params.strike_count != -1 {
            params.insert("strikeCount".into(), option_params.strike_count.to_string());
        }

        if option_params.interval != -1.0 {
            params.insert("interval".into(), option_params.interval.to_string());
        }

        if option_params.strike != -1.0 {
            params.insert("strike".into(), option_params.strike.to_string());
        }

        if option_params.from_date != "" {
            params.insert("fromDate".into(), option_params.from_date.clone());
        }

        if option_params.to_date != "" {
            params.insert("toDate".into(), option_params.to_date.clone());
        }

        if option_params.volatility != -1.0 {
            params.insert("volatility".into(), option_params.volatility.to_string());
        }

        if option_params.underlying_price != -1.0 {
            params.insert(
                "underlyingPrice".into(),
                option_params.underlying_price.to_string(),
            );
        }

        if option_params.interest_rate != -1.0 {
            params.insert(
                "interestRate".into(),
                option_params.interest_rate.to_string(),
            );
        }

        if option_params.days_to_expiration != -1 {
            params.insert(
                "daysToExpiration".into(),
                option_params.days_to_expiration.to_string(),
            );
        }

        // Required Parameters
        params.insert("symbol".into(), option_params.symbol.clone());
        params.insert("contractType".into(), option_params.contract_type.clone());
        params.insert(
            "includeQuotes".into(),
            option_params.include_quotes.to_string(),
        );
        params.insert("strategy".into(), option_params.strategy.clone());
        params.insert("range".into(), option_params.range.clone());
        params.insert("expMonth".into(), option_params.expiration_month.clone());
        params.insert("optionType".into(), option_params.option_type.clone());

        // Format URL
        let url: String = "marketdata/chains".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Preferences ////

    /// Preferences for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/user-principal/apis/get/accounts/%7BaccountId%7D/preferences-0
    pub async fn get_preferences(&mut self, acct_id: i64) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/preferences", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Update preferences for a specific account. Please note that the directOptionsRouting and directEquityRouting values cannot be modified via this operation
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - preference_spec: Preference body
    ///
    /// Official Documentation: https://developer.tdameritrade.com/user-principal/apis/put/accounts/%7BaccountId%7D/preferences-0
    pub async fn update_preferences(
        &mut self,
        acct_id: i64,
        preference_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Preference Spec To String
        let body: String = preference_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/preferences", acct_id);

        // Update Preferences
        requests_async::put(&self.reqwest_client, access_token, body, url).await
    }

    //// Price History ////

    /// Get price history for a symbol
    ///
    /// Parameters
    /// - history_params: `PriceHistory` object
    ///     - symbol: Enter one symbol (case-sensitive)
    ///     - period_type: The type of period to show. Valid period types are `day`, `month`, `year`, or `ytd` (year to date)
    ///     - period: The number of periods to show. Valid periods for day are `1`, `2`, `3`, `4`, `5`, or `10`. Valid periods for month are `1`, `2`, `3`, or `6`. Valid periods for year are `1`, `2`, `3`, `5`, `10`, `15`, or `20`. Valid periods for ytd are `1` (Optional)
    ///     - frequency_type: The type of frequency with which a new candle is formed. Valid frequency types for day are `minute`. Valid frequency types for month are `daily` or `weekly`. Valid frequency types for year are `daily`, `weekly`, or `monthly`. Valid frequency types for ytd are `daily` or `weekly`
    ///     - frequency: The number of the frequency_type to be included in each candle. Valid frequencies for minute are `1`, `5`, `10`, `15`, or `30`. Valid frequencies for daily are `1`. Valid frequencies for weekly are `1`. Valid frequencies for monthly are `1`
    ///     - start_date: Start date as milliseconds since epoch. If start_date and end_date are provided, period should not be provided (Optional)
    ///     - end_date: End date as milliseconds since epoch. If start_date and end_date are provided, period should not be provided (Optional)
    ///     - need_extended_hours_data: Returns extended hours data. Default is true (Optional)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
    pub async fn get_price_history(
        &mut self,
        history_params: &PriceHistory,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameters
        if history_params.period != -1 {
            params.insert("period".into(), history_params.period.to_string());
        }

        if history_params.start_date != -1 {
            params.insert("startDate".into(), history_params.start_date.to_string());
        }

        if history_params.end_date != -1 {
            params.insert("endDate".into(), history_params.end_date.to_string());
        }

        // Required Parameters
        params.insert("periodType".into(), history_params.period_type.clone());
        params.insert(
            "frequencyType".into(),
            history_params.frequency_type.clone(),
        );
        params.insert("frequency".into(), history_params.frequency.to_string());
        params.insert(
            "needExtendedHoursData".into(),
            history_params.need_extended_hours_data.to_string(),
        );

        // Format URL
        let url: String = format!("marketdata/{}/pricehistory", history_params.symbol);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Quotes ////

    /// Get quote for a symbol
    ///
    /// Parameters
    /// - symbol: Enter one symbol (case-sensitive)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/quotes/apis/get/marketdata/%7Bsymbol%7D/quotes
    pub async fn get_quote(&mut self, symbol: &str) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("marketdata/{}/quotes", symbol);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Get quote for one or more symbols
    ///
    /// Parameters
    /// - symbols: Enter one or more symbols in a vector (case-sensitive)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/quotes/apis/get/marketdata/quotes
    pub async fn get_quotes(&mut self, symbols: &Vec<&str>) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Convert Vector To String
        let cleaned_symbols: String = symbols.join(",");

        // Parameter
        params.insert("symbol".into(), cleaned_symbols);

        // Format URL
        let url: String = "marketdata/quotes".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Trading ////

    /// Get a specific order for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Order number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D/orders/%7BorderId%7D-0
    pub async fn get_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/orders/{}", acct_id, order_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Orders for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - max_results: The max number of orders to retrieve
    /// - from_entered_time: Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are `yyyy-MM-dd`
    /// - to_entered_time: Specifies that no orders entered after this time should be returned. Valid ISO-8601 formats are `yyyy-MM-dd`
    /// - status: Specifies that only orders of this status should be returned. Valid statuses are `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELLED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, or `EXPIRED`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D/orders-0
    pub async fn get_orders_by_path(
        &mut self,
        acct_id: i64,
        max_results: i64,
        from_entered_time: &str,
        to_entered_time: &str,
        status: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Parameters
        params.insert("maxResults".into(), max_results.to_string());
        params.insert("fromEnteredTime".into(), from_entered_time.to_string());
        params.insert("toEnteredTime".into(), to_entered_time.to_string());
        params.insert("status".into(), status.to_string());

        // Format URL
        let url: String = format!("accounts/{}/orders", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// All orders for a specific account or, if acct_id isn't specified, orders will be returned for all linked accounts
    ///
    /// Parameters
    /// - acct_id: Account number (Optional)
    /// - max_results: The max number of orders to retrieve
    /// - from_entered_time: Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are `yyyy-MM-dd`
    /// - to_entered_time: Specifies that no orders entered after this time should be returned. Valid ISO-8601 formats are `yyyy-MM-dd`
    /// - status: Specifies that only orders of this status should be returned. Valid statuses are `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELLED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, or `EXPIRED`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/orders-0
    pub async fn get_orders_by_query(
        &mut self,
        acct_id: Option<i64>,
        max_results: i64,
        from_entered_time: &str,
        to_entered_time: &str,
        status: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameter
        if let Some(id) = acct_id {
            params.insert("accountId".into(), id.to_string());
        }

        // Required Parameters
        params.insert("maxResults".into(), max_results.to_string());
        params.insert("fromEnteredTime".into(), from_entered_time.to_string());
        params.insert("toEnteredTime".into(), to_entered_time.to_string());
        params.insert("status".into(), status.to_string());

        // Format URL
        let url: String = "orders".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Place an order for a specific account. Order throttle limits may apply
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_spec: Order body
    ///
    /// Order Examples: https://developer.tdameritrade.com/content/place-order-samples
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/post/accounts/%7BaccountId%7D/orders-0
    pub async fn place_order(
        &mut self,
        acct_id: i64,
        order_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Order Spec To String
        let body: String = order_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/orders", acct_id);

        // Place Order
        requests_async::post(&self.reqwest_client, access_token, body, url).await
    }

    /// Replace an existing order for an account. The existing order will be replaced by the new order. Once replaced, the old order will be canceled and a new order will be created. Order throttle limits may apply
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Order number
    /// - order_spec: Order body
    ///
    /// Order Examples: https://developer.tdameritrade.com/content/place-order-samples
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/put/accounts/%7BaccountId%7D/orders/%7BorderId%7D-0
    pub async fn replace_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
        order_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Order Spec To String
        let body: String = order_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/orders/{}", acct_id, order_id);

        // Replace Order
        requests_async::put(&self.reqwest_client, access_token, body, url).await
    }

    /// Cancel a specific order for a specific account. Order throttle limits may apply
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Order number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/delete/accounts/%7BaccountId%7D/orders/%7BorderId%7D-0
    pub async fn cancel_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Format URL
        let url: String = format!("accounts/{}/orders/{}", acct_id, order_id);

        // Cancel Order
        requests_async::delete(&self.reqwest_client, access_token, url).await
    }

    /// Specific saved order by its ID, for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Saved order number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D/savedorders/%7BsavedOrderId%7D-0
    pub async fn get_saved_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/savedorders/{}", acct_id, order_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Saved orders for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D/savedorders-0
    pub async fn get_saved_orders_by_path(
        &mut self,
        acct_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/savedorders", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Save an order for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_spec: Order body
    ///
    /// Order Examples: https://developer.tdameritrade.com/content/place-order-samples
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/post/accounts/%7BaccountId%7D/savedorders-0
    pub async fn create_saved_order(
        &mut self,
        acct_id: i64,
        order_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Order Spec To String
        let body: String = order_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/savedorders", acct_id);

        // Create Saved Order
        requests_async::post(&self.reqwest_client, access_token, body, url).await
    }

    /// Replace an existing saved order for an account. The existing saved order will be replaced by the new order
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Order number
    /// - order_spec: Order body
    ///
    /// Order Examples: https://developer.tdameritrade.com/content/place-order-samples
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/put/accounts/%7BaccountId%7D/savedorders/%7BsavedOrderId%7D-0
    pub async fn replace_saved_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
        order_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Order Spec To String
        let body: String = order_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/savedorders/{}", acct_id, order_id);

        // Replace Order
        requests_async::put(&self.reqwest_client, access_token, body, url).await
    }

    /// Delete a specific saved order for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - order_id: Order number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/account-access/apis/delete/accounts/%7BaccountId%7D/savedorders/%7BsavedOrderId%7D-0
    pub async fn delete_saved_order(
        &mut self,
        acct_id: i64,
        order_id: i64,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Format URL
        let url: String = format!("accounts/{}/savedorders/{}", acct_id, order_id);

        // Cancel Order
        requests_async::delete(&self.reqwest_client, access_token, url).await
    }

    //// Transaction History ////

    /// Transaction for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - transaction_id: Transaction number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/transaction-history/apis/get/accounts/%7BaccountId%7D/transactions/%7BtransactionId%7D-0
    pub async fn get_transaction(
        &mut self,
        acct_id: i64,
        transaction_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/transactions/{}", acct_id, transaction_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Transactions for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - transaction_type: Only transactions with the specified type will be returned. Valid transaction types are `ALL`, `TRADE`, `BUY_ONLY`, `SELL_ONLY`, `CASH_IN_OR_CASH_OUT`, `CHECKING`, `DIVIDEND`, `INTEREST`, `OTHER`, or `ADVISOR_FEES`
    /// - symbol: Only transactions with the specified symbol will be returned (case-sensitive) (Optional)
    /// - start_date: Only transactions after the start date will be returned. The maximum date range is one year. Valid ISO-8601 formats are `yyyy-MM-dd`
    /// - end_date: Only transactions before the end date will be returned. The maximum date range is one year. Valid ISO-8601 formats are `yyyy-MM-dd`
    ///
    /// Official Documentation: https://developer.tdameritrade.com/transaction-history/apis/get/accounts/%7BaccountId%7D/transactions-0
    pub async fn get_transactions(
        &mut self,
        acct_id: i64,
        transaction_type: &str,
        symbol: Option<&str>,
        start_date: &str,
        end_date: &str,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameter
        if let Some(s) = symbol {
            params.insert("symbol".into(), s.into());
        }

        // Required Parameters
        params.insert("type".into(), transaction_type.into());
        params.insert("startDate".into(), start_date.into());
        params.insert("endDate".into(), end_date.into());

        // Format URL
        let url: String = format!("accounts/{}/transactions", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// User Info ////

    /// SubscriptionKey for provided accounts or default accounts
    ///
    /// Parameters
    /// - acct_ids: Account number(s)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/user-principal/apis/get/userprincipals/streamersubscriptionkeys-0
    pub async fn get_streamer_subscription_keys(
        &mut self,
        acct_ids: &Vec<i64>,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Convert Vector To String
        let cleaned_accts: String = acct_ids.iter().join(",");

        // Parameter
        params.insert("accountIds".into(), cleaned_accts);

        // Format URL
        let url: String = "userprincipals/streamersubscriptionkeys".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// User principal details
    ///
    /// Parameters
    /// - fields: Enter additional fields into a vector. None of these fields are returned by default. Valid fields are `streamerSubscriptionKeys`, `streamerConnectionInfo`, `preferences`, or `surrogateIds` (Optional)
    ///
    /// Official Documentation: https://developer.tdameritrade.com/user-principal/apis/get/userprincipals-0
    pub async fn get_user_principals(
        &mut self,
        fields: Option<&Vec<&str>>,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let mut params: HashMap<String, String> = HashMap::new();

        // Optional Parameter
        if let Some(f) = fields {
            // Convert Vector To String
            let cleaned_fields: String = f.join(",");

            params.insert("fields".into(), cleaned_fields);
        }

        // Format URL
        let url: String = "userprincipals".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    //// Watchlist ////

    /// Specific watchlist for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - watchlist_id: Watchlist number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/get/accounts/%7BaccountId%7D/watchlists/%7BwatchlistId%7D-0
    pub async fn get_watchlist(
        &mut self,
        acct_id: i64,
        watchlist_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/watchlists/{}", acct_id, watchlist_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// All watchlists of an account
    ///
    /// Parameters
    /// - acct_id: Account number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/get/accounts/%7BaccountId%7D/watchlists-0
    pub async fn get_watchlists_for_single_account(
        &mut self,
        acct_id: i64,
    ) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = format!("accounts/{}/watchlists", acct_id);

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// All watchlists for all of the user's linked accounts
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/get/accounts/watchlists-0
    pub async fn get_watchlists_for_multiple_accounts(&mut self) -> Result<String, TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Create HashMap To Store Parameters
        let params: HashMap<String, String> = HashMap::new();

        // Format URL
        let url: String = "accounts/watchlists".into();

        // Return String Response
        requests_async::get(&self.reqwest_client, access_token, params, url).await
    }

    /// Create watchlist for specific account. This method does not verify that the symbol or asset type are valid
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - watchlist_spec: Watchlist body
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/post/accounts/%7BaccountId%7D/watchlists-0
    pub async fn create_watchlist(
        &mut self,
        acct_id: i64,
        watchlist_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Watchlist Spec To String
        let body: String = watchlist_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/watchlists", acct_id);

        // Create Watchlist
        requests_async::post(&self.reqwest_client, access_token, body, url).await
    }

    /// Replace watchlist for a specific account. This method does not verify that the symbol or asset type are valid
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - watchlist_id: Watchlist number
    /// - watchlist_spec: Watchlist body
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/put/accounts/%7BaccountId%7D/watchlists/%7BwatchlistId%7D-0
    pub async fn replace_watchlist(
        &mut self,
        acct_id: i64,
        watchlist_id: i64,
        watchlist_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Watchlist Spec To String
        let body: String = watchlist_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/watchlists/{}", acct_id, watchlist_id);

        // Replace Watchlist
        requests_async::put(&self.reqwest_client, access_token, body, url).await
    }

    /// Partially update watchlist for a specific account: change watchlist name, add to the beginning/end of a watchlist, update or delete items in a watchlist. This method does not verify that the symbol or asset type are valid
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - watchlist_id: Watchlist number
    /// - watchlist_spec: Watchlist body
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/patch/accounts/%7BaccountId%7D/watchlists/%7BwatchlistId%7D-0
    pub async fn update_watchlist(
        &mut self,
        acct_id: i64,
        watchlist_id: i64,
        watchlist_spec: &str,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Convert Watchlist Spec To String
        let body: String = watchlist_spec.into();

        // Format URL
        let url: String = format!("accounts/{}/watchlists/{}", acct_id, watchlist_id);

        // Update Watchlist
        requests_async::patch(&self.reqwest_client, access_token, body, url).await
    }

    /// Delete watchlist for a specific account
    ///
    /// Parameters
    /// - acct_id: Account number
    /// - watchlist_id: Watchlist number
    ///
    /// Official Documentation: https://developer.tdameritrade.com/watchlist/apis/delete/accounts/%7BaccountId%7D/watchlists/%7BwatchlistId%7D-0
    pub async fn delete_watchlist(
        &mut self,
        acct_id: i64,
        watchlist_id: i64,
    ) -> Result<(), TDAClientError> {
        // Check Token Validity
        self.auth.check_token_validity().await;

        // Get Access Token
        let access_token: String = self.auth.get_access_token();

        // Format URL
        let url: String = format!("accounts/{}/watchlists/{}", acct_id, watchlist_id);

        // Delete Watchlist
        requests_async::delete(&self.reqwest_client, access_token, url).await
    }
}
