//! Example implementation of the Wrapper callback trait.  Just logs callback
//! methods
use std::collections::HashSet;

use std::time::{Duration, UNIX_EPOCH};

use bigdecimal::BigDecimal;
use chrono::prelude::DateTime;
use chrono::Utc;
use log::*;

use twsapi::core::common::{
    BarData, CommissionReport, DepthMktDataDescription, FaDataType, FamilyCode, HistogramData,
    HistoricalTick, HistoricalTickBidAsk, HistoricalTickLast, NewsProvider, PriceIncrement,
    RealTimeBar, SmartComponent, TickAttrib, TickAttribBidAsk, TickAttribLast, TickByTickType,
    TickType,
};
use twsapi::core::contract::{
    Contract, ContractDescription, ContractDetails, DeltaNeutralContract,
};
use twsapi::core::execution::Execution;
use twsapi::core::order::{Order, OrderState, SoftDollarTier};
use twsapi::core::wrapper::Wrapper;

//==================================================================================================
/// Example implementation of the Wrapper callback trait.  Just logs callback
/// methods
pub struct DefaultWrapper {}

impl DefaultWrapper {
    pub fn new() -> Self {
        DefaultWrapper {}
    }
}

impl Default for DefaultWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Wrapper for DefaultWrapper {
    fn error(&mut self, request_id: i32, error_code: i32, error_string: &str) {
        error!(
            "request_id: {} ,error_code: {} , error_string:{}",
            request_id, error_code, error_string
        );
    }

    //----------------------------------------------------------------------------------------------
    fn win_error(&mut self, text: &str, last_error: i32) {
        error!("text: {} , last_error:{}", text, last_error);
    }

    //----------------------------------------------------------------------------------------------
    fn connect_ack(&mut self) {
        info!("Connected.");
    }

    //----------------------------------------------------------------------------------------------
    fn market_data_type(&mut self, request_id: i32, market_data_type: i32) {
        info!(
            "market_data_type -- request_id: {}, market_data_type: {}",
            request_id, market_data_type
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_price(&mut self, request_id: i32, tick_type: TickType, price: f64, attrib: TickAttrib) {
        info!(
            "tick_size -- request_id: {}, tick_type: {}, price: {}, attrib: {}",
            request_id, tick_type, price, attrib
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_size(&mut self, request_id: i32, tick_type: TickType, size: i32) {
        info!(
            "tick_size -- request_id: {}, tick_type: {}, size: {}",
            request_id, tick_type, size
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_snapshot_end(&mut self, request_id: i32) {
        info!("tick_snapshot_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn tick_generic(&mut self, request_id: i32, tick_type: TickType, value: f64) {
        info!(
            "tick_generic -- request_id: {}, tick_type: {}, value: {}",
            request_id, tick_type, value
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_string(&mut self, request_id: i32, tick_type: TickType, value: &str) {
        info!(
            "tick_string -- request_id: {}, tick_type: {}, value: {}",
            request_id, tick_type, value
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_exchange_for_physical(
        &mut self,
        request_id: i32,
        tick_type: TickType,
        basis_points: f64,
        formatted_basis_points: &str,
        total_dividends: f64,
        hold_days: i32,
        future_last_trade_date: &str,
        dividend_impact: f64,
        dividends_to_last_trade_date: f64,
    ) {
        info!(
            "tick_efp -- request_id: {},
             tick_type: {},
             basis_points: {},
             formatted_basis_points: {},
             total_dividends: {},
             hold_days: {},
             future_last_trade_date: {},
             dividend_impact: {},
             dividends_to_last_trade_date: {},",
            request_id,
            tick_type,
            basis_points,
            formatted_basis_points,
            total_dividends,
            hold_days,
            future_last_trade_date,
            dividend_impact,
            dividends_to_last_trade_date,
        );
    }

    //----------------------------------------------------------------------------------------------
    fn order_status(
        &mut self,
        order_id: i32,
        status: &str,
        filled: f64,
        remaining: f64,
        avg_fill_price: f64,
        perm_id: i32,
        parent_id: i32,
        last_fill_price: f64,
        client_id: i32,
        why_held: &str,
        market_cap_price: f64,
    ) {
        info!(
            "order_status -- order_id: {}, status: {}, filled: {}, remaining: {}, avg_fill_price: {}, \
            perm_id: {}, parent_id: {}, last_fill_price: {}, client_id: {}, why_held: {}, market_cap_price: {}",
            order_id, status, filled, remaining, avg_fill_price, perm_id, parent_id, last_fill_price,
            client_id, why_held, market_cap_price
        );
    }

    //----------------------------------------------------------------------------------------------
    fn open_order(
        &mut self,
        order_id: i32,
        contract: Contract,
        order: Order,
        order_state: OrderState,
    ) {
        info!(
            "open_order -- order_id: {}, contract: {}, order: {}, order_state: {}",
            order_id, contract, order, order_state
        );
    }

    //----------------------------------------------------------------------------------------------
    fn open_order_end(&mut self) {
        info!("open_order_end. (no parmeters passed)");
    }

    //----------------------------------------------------------------------------------------------
    fn connection_closed(&mut self) {
        info!("connection_closed. (no parmeters passed)");
    }

    //----------------------------------------------------------------------------------------------
    fn update_account_value(&mut self, key: &str, val: &str, currency: &str, account_name: &str) {
        info!(
            "key: {}, value: {}, ccy: {}, account: {}.",
            key, val, currency, account_name
        );
    }

    //----------------------------------------------------------------------------------------------
    fn update_portfolio(
        &mut self,
        contract: Contract,
        position: f64,
        market_price: f64,
        market_value: f64,
        average_cost: f64,
        unrealized_pnl: f64,
        realized_pnl: f64,
        account_name: &str,
    ) {
        info!(
            "update_portfolio -- contract: {}, position: {}, market_price: {}, market_value: {},
             average_cost: {}, unrealized_pnl: {},  realized_pnl: {},  account_name: {}",
            contract,
            position,
            market_price,
            market_value,
            average_cost,
            unrealized_pnl,
            realized_pnl,
            account_name
        );
    }

    //----------------------------------------------------------------------------------------------
    fn update_account_time(&mut self, time_stamp: &str) {
        info!("update_account_time: {}.", time_stamp);
    }

    //----------------------------------------------------------------------------------------------
    fn account_download_end(&mut self, account_name: &str) {
        info!("account_download_end: {}.", account_name);
    }

    //----------------------------------------------------------------------------------------------
    fn next_valid_id(&mut self, order_id: i32) {
        info!("next_valid_id -- order_id: {}", order_id);
    }

    //----------------------------------------------------------------------------------------------
    fn contract_details(&mut self, request_id: i32, contract_details: ContractDetails) {
        info!(
            "contract_details -- request_id: {}, contract_details: {}",
            request_id, contract_details
        );
    }

    //----------------------------------------------------------------------------------------------
    fn bond_contract_details(&mut self, request_id: i32, contract_details: ContractDetails) {
        info!(
            "bond_contract_details -- request_id: {}, contract_details: {}",
            request_id, contract_details
        );
    }

    //----------------------------------------------------------------------------------------------
    fn contract_details_end(&mut self, request_id: i32) {
        info!("contract_details_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn exec_details(&mut self, request_id: i32, contract: Contract, execution: Execution) {
        info!(
            "exec_details -- request_id: {}, contract: {}, execution: {}",
            request_id, contract, execution
        );
    }

    //----------------------------------------------------------------------------------------------
    fn exec_details_end(&mut self, request_id: i32) {
        info!("exec_details_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn update_market_depth(
        &mut self,
        request_id: i32,
        position: i32,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
    ) {
        info!(
            "update_market_depth -- request_id: {}, position: {}, operation: {}, side: {}, price: {}, size: {}",
            request_id, position, operation, side, price, size
        );
    }

    //----------------------------------------------------------------------------------------------
    fn update_market_depth_l2(
        &mut self,
        request_id: i32,
        position: i32,
        market_maker: &str,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
        is_smart_depth: bool,
    ) {
        info!(
            "update_market_depth_l2 -- request_id: {}, position: {}, market_maker: {}, operation: {}, side: {}, price: {}, size: {}, is_smart_depth: {},",
            request_id, position, market_maker, operation, side, price, size, is_smart_depth
        );
    }

    //----------------------------------------------------------------------------------------------
    fn update_news_bulletin(
        &mut self,
        msg_id: i32,
        msg_type: i32,
        news_message: &str,
        origin_exch: &str,
    ) {
        info!(
            "update_news_bulletin -- msg_id: {}, msg_type: {}, news_message: {}, origin_exch: {}",
            msg_id, msg_type, news_message, origin_exch
        );
    }

    //----------------------------------------------------------------------------------------------
    fn managed_accounts(&mut self, accounts_list: &str) {
        info!("managed_accounts -- accounts_list: {}", accounts_list);
    }

    //----------------------------------------------------------------------------------------------
    fn receive_financial_advisor(&mut self, fa_data: FaDataType, cxml: &str) {
        info!("receive_fa -- fa_data: {}, cxml: {}", fa_data, cxml);
    }

    //----------------------------------------------------------------------------------------------
    fn historical_data(&mut self, request_id: i32, bar: BarData) {
        info!(
            "historical_data -- request_id: {}, bar: {}",
            request_id, bar
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_data_end(&mut self, request_id: i32, start: &str, end: &str) {
        info!(
            "historical_data_end -- request_id: {}, start: {}, end: {}",
            request_id, start, end
        );
    }

    //----------------------------------------------------------------------------------------------
    fn scanner_parameters(&mut self, xml: &str) {
        info!("scanner_parameters -- xml: {}", xml);
    }

    //----------------------------------------------------------------------------------------------
    fn scanner_data(
        &mut self,
        request_id: i32,
        rank: i32,
        contract_details: ContractDetails,
        distance: &str,
        benchmark: &str,
        projection: &str,
        legs_str: &str,
    ) {
        info!(
            "scanner_data -- request_id: {}, rank: {},
             contract_details: {},
             distance: {},
             benchmark: {},
             projection: {},
             legs_str: {}",
            request_id, rank, contract_details, distance, benchmark, projection, legs_str
        );
    }

    //----------------------------------------------------------------------------------------------
    fn scanner_data_end(&mut self, request_id: i32) {
        info!("scanner_data_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn realtime_bar(&mut self, request_id: i32, bar: RealTimeBar) {
        info!(
            "realtime_bar -- request_id: {}, date_time: {}, open: {}, high: {}, low: {}, close: {}, volume: {}, wap: {}, count: {}",
            request_id,
            bar.date_time,
            bar.open,
            bar.high,
            bar.low,
            bar.close,
            bar.volume,
            bar.wap,
            bar.count,
        );
    }

    //----------------------------------------------------------------------------------------------
    fn current_time(&mut self, time: i64) {
        // Creates a new SystemTime from the specified number of whole seconds
        let d = UNIX_EPOCH + Duration::from_secs(time as u64);
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(d);
        // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        info!("current_time -- time: {}", timestamp_str);
    }

    //----------------------------------------------------------------------------------------------
    fn fundamental_data(&mut self, request_id: i32, data: &str) {
        info!(
            "fundamental_data -- request_id: {}, delta_neutral_contract: {}",
            request_id, data
        );
    }

    //----------------------------------------------------------------------------------------------
    fn delta_neutral_validation(
        &mut self,
        request_id: i32,
        delta_neutral_contract: DeltaNeutralContract,
    ) {
        info!(
            "delta_neutral_validation -- request_id: {}, delta_neutral_contract: {}",
            request_id, delta_neutral_contract
        );
    }

    //----------------------------------------------------------------------------------------------
    fn commission_report(&mut self, commission_report: CommissionReport) {
        info!(
            "commission_report -- commission_report: {}",
            commission_report
        );
    }

    //----------------------------------------------------------------------------------------------
    fn position(&mut self, account: &str, contract: Contract, position: f64, avg_cost: f64) {
        info!(
            "position -- account: {}, contract: [{}], position: {}, avg_cost: {}",
            account, contract, position, avg_cost
        );
    }

    //----------------------------------------------------------------------------------------------
    fn position_end(&mut self) {
        info!("position_end -- (no params are passed in this one)");
    }

    //----------------------------------------------------------------------------------------------
    fn account_summary(
        &mut self,
        request_id: i32,
        account: &str,
        tag: &str,
        value: &str,
        currency: &str,
    ) {
        info!(
            "account_summary -- request_id: {}, account: {}, tag: {}, value: {}, currency: {}",
            request_id, account, tag, value, currency
        );
    }

    //----------------------------------------------------------------------------------------------
    fn account_summary_end(&mut self, request_id: i32) {
        info!("account_summary_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn verify_message_api(&mut self, api_data: &str) {
        info!("verify_message_api -- api_data: {}", api_data);
    }

    //----------------------------------------------------------------------------------------------
    fn verify_completed(&mut self, is_successful: bool, error_text: &str) {
        info!(
            "verify_completed -- is_successful: {}, error_text: {}",
            is_successful, error_text
        );
    }

    //----------------------------------------------------------------------------------------------
    fn verify_and_auth_message_api(&mut self, api_data: &str, xyz_challange: &str) {
        info!(
            "verify_and_auth_message_api -- api_data: {}, xyz_challange: {}",
            api_data, xyz_challange
        );
    }

    //----------------------------------------------------------------------------------------------
    fn verify_and_auth_completed(&mut self, is_successful: bool, error_text: &str) {
        info!(
            "verify_and_auth_completed -- is_successful: {}, error_text: {}",
            is_successful, error_text
        );
    }

    //----------------------------------------------------------------------------------------------
    fn display_group_list(&mut self, request_id: i32, groups: &str) {
        info!(
            "display_group_list -- request_id: {}, error_text: {}",
            request_id, groups
        );
    }

    //----------------------------------------------------------------------------------------------
    fn display_group_updated(&mut self, request_id: i32, contract_info: &str) {
        info!(
            "display_group_updated -- request_id: {}, contract_info: {}",
            request_id, contract_info
        );
    }

    //----------------------------------------------------------------------------------------------
    fn position_multi(
        &mut self,
        request_id: i32,
        account: &str,
        model_code: &str,
        contract: Contract,
        pos: f64,
        avg_cost: f64,
    ) {
        info!(
            "position_multi -- request_id: {}, account: {}, model_code: {}, contract: {}, pos: {}, \
             avg_cost: {}",
            request_id, account, model_code, contract, pos, avg_cost
        );
    }

    //----------------------------------------------------------------------------------------------
    fn position_multi_end(&mut self, request_id: i32) {
        info!("position_multi_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn account_update_multi(
        &mut self,
        request_id: i32,
        account: &str,
        model_code: &str,
        key: &str,
        value: &str,
        currency: &str,
    ) {
        info!(
            "account_update_multi -- request_id: {}, account: {}, model_code: {}, key: {}, value: {}, currency: {}",
            request_id, account, model_code, key, value, currency
        );
    }

    //----------------------------------------------------------------------------------------------
    fn account_update_multi_end(&mut self, request_id: i32) {
        info!("account_update_multi_end -- request_id: {}", request_id);
    }

    //----------------------------------------------------------------------------------------------
    fn tick_option_computation(
        &mut self,
        request_id: i32,
        tick_type: TickType,
        implied_vol: f64,
        delta: f64,
        opt_price: f64,
        pv_dividend: f64,
        gamma: f64,
        vega: f64,
        theta: f64,
        und_price: f64,
    ) {
        info!(
            "tick_option_computation -- request_id: {}, tick_type: {}, implied_vol: {}, delta: {}, \
             opt_price: {}, pv_dividend: {},  gamma: {}, vega: {}, theta: {}, und_price: {}",
            request_id,
            tick_type,
            implied_vol,
            delta,
            opt_price,
            pv_dividend,
            gamma,
            vega,
            theta,
            und_price
        );
    }

    //----------------------------------------------------------------------------------------------
    fn security_definition_option_parameter(
        &mut self,
        request_id: i32,
        exchange: &str,
        underlying_con_id: i32,
        trading_class: &str,
        multiplier: &str,
        expirations: HashSet<String>,
        strikes: HashSet<BigDecimal>,
    ) {
        info!(
            "tick_option_computation -- request_id: {}, exchange: {}, underlying_con_id: {}, \
             trading_class: {}, multiplier: {}, expirations: {:?},  strikes: {:?}",
            request_id,
            exchange,
            underlying_con_id,
            trading_class,
            multiplier,
            expirations
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>(),
            strikes.iter().cloned().collect::<Vec<BigDecimal>>()
        );
    }

    //----------------------------------------------------------------------------------------------
    fn security_definition_option_parameter_end(&mut self, request_id: i32) {
        info!(
            "security_definition_option_parameter_end -- request_id: {}",
            request_id
        );
    }

    //----------------------------------------------------------------------------------------------
    fn soft_dollar_tiers(&mut self, request_id: i32, tiers: Vec<SoftDollarTier>) {
        info!(
            "soft_dollar_tiers -- request_id: {}, tiers: {:?}",
            request_id, tiers
        );
    }

    //----------------------------------------------------------------------------------------------
    fn family_codes(&mut self, family_codes: Vec<FamilyCode>) {
        info!("family_codes -- family_codes: {:?}", family_codes);
    }

    //----------------------------------------------------------------------------------------------
    fn symbol_samples(&mut self, request_id: i32, contract_descriptions: Vec<ContractDescription>) {
        info!(
            "symbol_samples -- request_id: {}, contract_descriptions: {:?}",
            request_id, contract_descriptions
        );
    }

    //----------------------------------------------------------------------------------------------
    fn market_depth_exchanges(
        &mut self,
        depth_market_data_descriptions: Vec<DepthMktDataDescription>,
    ) {
        info!(
            "market_depth_exchanges -- depth_market_data_descriptions: {:?}",
            depth_market_data_descriptions
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_news(
        &mut self,
        ticker_id: i32,
        time_stamp: i32,
        provider_code: &str,
        article_id: &str,
        headline: &str,
        extra_data: &str,
    ) {
        info!(
            "tick_news -- ticker_id: {}, time_stamp: {}, provider_code: {}, article_id: {}, \
             headline: {}, extra_data: {},",
            ticker_id, time_stamp, provider_code, article_id, headline, extra_data
        );
    }

    //----------------------------------------------------------------------------------------------
    fn smart_components(&mut self, request_id: i32, smart_components: Vec<SmartComponent>) {
        info!(
            "smart_components -- request_id: {}, smart_components: {:?}",
            request_id, smart_components
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_request_parameters(
        &mut self,
        ticker_id: i32,
        min_tick: f64,
        bbo_exchange: &str,
        snapshot_permissions: i32,
    ) {
        info!(
            "tick_request_params -- ticker_id: {}, min_tick: {}, bbo_exchange: {}, snapshot_permissions: {}",
            ticker_id, min_tick, bbo_exchange, snapshot_permissions
        );
    }

    //----------------------------------------------------------------------------------------------
    fn news_providers(&mut self, news_providers: Vec<NewsProvider>) {
        info!("news_providers -- news_providers: {:?}", news_providers);
    }

    //----------------------------------------------------------------------------------------------
    fn news_article(&mut self, request_id: i32, article_type: i32, article_text: &str) {
        info!(
            "news_article -- request_id: {}, article_type: {}, article_text: {}",
            request_id, article_type, article_text
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_news(
        &mut self,
        request_id: i32,
        time: &str,
        provider_code: &str,
        article_id: &str,
        headline: &str,
    ) {
        info!(
            "historical_news -- request_id: {}, time: {}, provider_code: {}, article_id: {}, headline: {}",
            request_id, time, provider_code, article_id, headline
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_news_end(&mut self, request_id: i32, has_more: bool) {
        info!(
            "historical_news_end -- request_id: {}, has_more: {}",
            request_id, has_more
        );
    }

    //----------------------------------------------------------------------------------------------
    fn head_timestamp(&mut self, request_id: i32, head_timestamp: &str) {
        info!(
            "head_timestamp -- request_id: {}, head_timestamp: {}",
            request_id, head_timestamp
        );
    }

    //----------------------------------------------------------------------------------------------
    fn histogram_data(&mut self, request_id: i32, items: Vec<HistogramData>) {
        info!(
            "histogram_data -- request_id: {}, items: {:?}",
            request_id, items
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_data_update(&mut self, request_id: i32, bar: BarData) {
        info!(
            "historical_data_update -- request_id: {}, bar: {}",
            request_id, bar
        );
    }

    //----------------------------------------------------------------------------------------------
    fn reroute_market_data_request(&mut self, request_id: i32, con_id: i32, exchange: &str) {
        info!(
            "reroute_market_data_req -- request_id: {}, con_id: {}, exchange: {}",
            request_id, con_id, exchange
        );
    }

    //----------------------------------------------------------------------------------------------
    fn reroute_market_depth_request(&mut self, request_id: i32, con_id: i32, exchange: &str) {
        info!(
            "reroute_market_depth_req -- request_id: {}, con_id: {}, exchange: {}",
            request_id, con_id, exchange
        );
    }

    //----------------------------------------------------------------------------------------------
    fn market_rule(&mut self, market_rule_id: i32, price_increments: Vec<PriceIncrement>) {
        info!(
            "market_rule -- market_rule_id: {}, price_increments: {:?}",
            market_rule_id, price_increments
        );
    }

    //----------------------------------------------------------------------------------------------
    fn profit_and_loss(
        &mut self,
        request_id: i32,
        daily_pn_l: f64,
        unrealized_pn_l: f64,
        realized_pn_l: f64,
    ) {
        info!(
            "pnl -- request_id: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {})",
            request_id, daily_pn_l, unrealized_pn_l, realized_pn_l
        );
    }

    //----------------------------------------------------------------------------------------------
    fn profit_and_loss_single(
        &mut self,
        request_id: i32,
        pos: i32,
        daily_pn_l: f64,
        unrealized_pn_l: f64,
        realized_pn_l: f64,
        value: f64,
    ) {
        info!(
            "pnl_single -- request_id: {}, pos: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {}, value: {})",
            request_id, pos, daily_pn_l, unrealized_pn_l, realized_pn_l, value
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_ticks(&mut self, request_id: i32, ticks: Vec<HistoricalTick>, done: bool) {
        info!(
            "historical_ticks -- request_id: {}, ticks: {:?}, done: {}",
            request_id, ticks, done
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_ticks_bid_ask(
        &mut self,
        request_id: i32,
        ticks: Vec<HistoricalTickBidAsk>,
        done: bool,
    ) {
        info!(
            "historical_ticks_bid_ask -- request_id: {}, ticks: {:?}, done: {}",
            request_id, ticks, done
        );
    }

    //----------------------------------------------------------------------------------------------
    fn historical_ticks_last(
        &mut self,
        request_id: i32,
        ticks: Vec<HistoricalTickLast>,
        done: bool,
    ) {
        info!(
            "historical_ticks_last -- request_id: {}, ticks: {:?}, done: {}",
            request_id, ticks, done
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_by_tick_all_last(
        &mut self,
        request_id: i32,
        tick_type: TickByTickType,
        time: i64,
        price: f64,
        size: i32,
        tick_attrib_last: TickAttribLast,
        exchange: &str,
        special_conditions: &str,
    ) {
        info!(
            "tick_by_tick_all_last -- request_id: {}, tick_type: {}, time: {}, price: {}, size: {}, \
             tick_attrib_last: {}, exchange: {}, special_conditions: {}",
            request_id, tick_type, time, price, size, tick_attrib_last, exchange, special_conditions
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_by_tick_bid_ask(
        &mut self,
        request_id: i32,
        time: i64,
        bid_price: f64,
        ask_price: f64,
        bid_size: i32,
        ask_size: i32,
        tick_attrib_bid_ask: TickAttribBidAsk,
    ) {
        info!(
            "tick_by_tick_bid_ask -- request_id: {}, time: {}, bid_price: {}, ask_price: {}, bid_size: {}, \
             ask_size: {}, tick_attrib_last: {}",
            request_id, time, bid_price, ask_price, bid_size, ask_size, tick_attrib_bid_ask
        );
    }

    //----------------------------------------------------------------------------------------------
    fn tick_by_tick_mid_point(&mut self, request_id: i32, time: i64, mid_point: f64) {
        info!(
            "tick_by_tick_mid_point -- request_id: {}, time: {}, mid_point: {}",
            request_id, time, mid_point
        );
    }

    //----------------------------------------------------------------------------------------------
    fn order_bound(&mut self, request_id: i32, api_client_id: i32, api_order_id: i32) {
        info!(
            "order_bound -- request_id: {}, api_client_id: {}, api_order_id: {}",
            request_id, api_client_id, api_order_id
        );
    }

    //----------------------------------------------------------------------------------------------
    fn completed_order(&mut self, contract: Contract, order: Order, order_state: OrderState) {
        info!(
            "completed_order -- contract: [{}], order: [{}], order_state: [{}]",
            contract, order, order_state
        );
    }

    //----------------------------------------------------------------------------------------------
    fn completed_orders_end(&mut self) {
        info!("completed_orders_end -- (no parameters for this message)");
    }
}
