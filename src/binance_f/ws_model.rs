use super::rest_model::{string_or_float, Asks, Bids};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum FuturesWebsocketEvent {
    #[serde(alias = "aggTrade")]
    AggrTrades(Box<AggrTradesEvent>),

    #[serde(alias = "markPriceUpdate")]
    MarkPrice(Box<MarkPriceEvent>),

    #[serde(alias = "kline")]
    Kline(Box<KlineEvent>),

    #[serde(alias = "continuous_kline")]
    ContinuousKline(Box<ContinuousKlineEvent>),

    #[serde(alias = "24hrTicker")]
    DayTicker(Box<DayTickerEvent>),

    #[serde(alias = "24hrMiniTicker")]
    MiniTicker(Box<MiniTickerEvent>),

    #[serde(alias = "bookTicker")]
    BookTicker(Box<BookTickerEvent>),

    #[serde(alias = "forceOrder")]
    Liquidation(Box<LiquidationEvent>),

    #[serde(alias = "depthUpdate")]
    DepthOrderBook(Box<DepthOrderBookEvent>),

    // todo nav
    // todo nav_kline
    // todo composite_index
    // todo margin_call
    #[serde(alias = "ACCOUNT_UPDATE")]
    AccountUpdate(Box<AccountUpdateEvent>),
    #[serde(alias = "ORDER_TRADE_UPDATE")]
    OrderTradeUpdate(Box<OrderTradeUpdateEvent>),
    #[serde(alias = "ACCOUNT_CONFIG_UPDATE")]
    AccountConfigUpdate(Box<AccountConfigUpdateEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub iceberg_qty: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "a")]
    pub account_update: AccountUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "m")]
    pub event_reason_type: String,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    pub ac: AccountConfigUpdate1,
    #[serde(skip)]
    pub ai: Option<AccountConfigUpdate2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigUpdate1 {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigUpdate2 {
    #[serde(rename = "j")]
    pub multi_assets_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(rename = "cw", with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "bc", with = "string_or_float")]
    pub balances_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "string_or_float")]
    pub position_amount: f64,
    #[serde(rename = "ep", with = "string_or_float")]
    pub entry_price: f64,
    #[serde(rename = "cr", with = "string_or_float")]
    pub accumulated_realized: f64,
    #[serde(rename = "up", with = "string_or_float")]
    pub unrealized_pnl: f64,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw", with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(skip, rename = "ma")]
    pub margin_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTradeUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "o")]
    pub order_trade_update: OrderTradeUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTradeUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", with = "string_or_float")]
    pub original_quantity: f64,
    #[serde(rename = "p", with = "string_or_float")]
    pub original_price: f64,
    #[serde(rename = "ap", with = "string_or_float")]
    pub average_price: f64,
    #[serde(rename = "sp", with = "string_or_float")]
    pub stop_price: f64,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_float")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", with = "string_or_float")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "L", with = "string_or_float")]
    pub last_filled_price: f64,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(skip, rename = "n", with = "string_or_float_opt")]
    pub commission: Option<f64>,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b", with = "string_or_float")]
    pub bid_notinal: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub ask_notinal: f64,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "R")]
    pub reduce_only: bool,
    #[serde(rename = "wt")]
    pub stop_price_working_type: String,
    #[serde(rename = "ot")]
    pub original_order_type: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(skip, rename = "cp")]
    pub close_all_post_condition_order: Option<bool>,
    #[serde(skip, rename = "AP", with = "string_or_float_opt")]
    pub activation_price: Option<f64>,
    #[serde(skip, rename = "cr", with = "string_or_float_opt")]
    pub callback_rate: Option<f64>,
    #[serde(rename = "rp", with = "string_or_float")]
    pub realized_profit: f64,
    // undoced
    #[serde(rename = "pP")]
    pub p_p: bool,
    #[serde(with = "string_or_float")]
    pub si: f64,
    #[serde(with = "string_or_float")]
    pub ss: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggrTradesEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "f")]
    pub first_break_trade_id: u64,
    #[serde(rename = "l")]
    pub last_break_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

// https://binance-docs.github.io/apidocs/futures/en/#mark-price-stream
// https://binance-docs.github.io/apidocs/delivery/en/#mark-price-stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "P")]
    pub estimate_settle_price: String,
    #[serde(rename = "T")]
    pub next_funding_time: u64,
    #[serde(rename = "i")]
    pub index_price: Option<String>,
    #[serde(rename = "p")]
    pub mark_price: String,
    #[serde(rename = "r")]
    pub funding_rate: String,
    #[serde(rename = "s")]
    pub symbol: String,
}

// Object({"E": Number(1626118018407), "e": String("forceOrder"), "o": Object({"S": String("SELL"), "T": Number(1626118018404), "X": String("FILLED"), "ap": String("33028.07"), "f": String("IOC"), "l": String("0.010"), "o": String("LIMIT"), "p": String("32896.00"), "q": String("0.010"), "s": String("BTCUSDT"), "z": String("0.010")})})
// https://binance-docs.github.io/apidocs/futures/en/#liquidation-order-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "o")]
    pub liquidation_order: LiquidationOrder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q")]
    pub original_quantity: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "ap")]
    pub average_price: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,
    #[serde(rename = "z")]
    pub order_filled_accumulated_quantity: String,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b", with = "string_or_float")]
    pub best_bid: f64,
    #[serde(rename = "B", with = "string_or_float")]
    pub best_bid_qty: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub best_ask: f64,
    #[serde(rename = "A", with = "string_or_float")]
    pub best_ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "w")]
    pub average_price: String,
    #[serde(rename = "c")]
    pub current_close: String,
    #[serde(rename = "Q")]
    pub current_close_qty: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "q")]
    pub quote_volume: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: Kline,
}

// https://binance-docs.github.io/apidocs/futures/en/#continuous-contract-kline-candlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlineEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: String,
    #[serde(rename = "k")]
    pub kline: ContinuousKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub active_buy_volume: String,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,
    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub active_buy_volume: String,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,
    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "pu")]
    pub previous_final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bids>,
    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}
