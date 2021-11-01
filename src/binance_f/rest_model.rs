use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    pub price: Decimal,
    pub qty: Decimal,
}

impl Asks {
    pub fn new(price: Decimal, qty: Decimal) -> Asks {
        Asks { price, qty }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    pub price: Decimal,
    pub qty: Decimal,
}

impl Bids {
    pub fn new(price: Decimal, qty: Decimal) -> Bids {
        Bids { price, qty }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        notional: Option<String>,
        min_notional: Option<String>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: Option<u16> },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: String },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Klines {
    AllKlines(Vec<Kline>),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub open_time: i64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub close_time: i64,
    #[serde(with = "string_or_float")]
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    #[serde(with = "string_or_float")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(with = "string_or_float")]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u16,
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

// User Stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<BookTicker>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub exchange_filters: Vec<String>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: u64,
    pub assets: Vec<Assets>,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub maint_margin_percent: f64,
    #[serde(with = "string_or_float")]
    pub required_margin_percent: f64,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u64,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: u16,
    #[serde(with = "string_or_float")]
    pub trigger_protect: f64,
    pub filters: Vec<Filters>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookPartial {
    pub last_update_id: u64,
    // Undocumented
    #[serde(rename = "E")]
    pub event_time: u64,
    // Undocumented
    #[serde(rename = "T")]
    pub transaction_time: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Trades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub is_buyer_maker: bool,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AggTrades {
    AllAggTrades(Vec<AggTrade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "m")]
    pub maker: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPrices {
    AllMarkPrices(Vec<MarkPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "string_or_float")]
    pub average_price: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub reduce_only: bool,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub orig_type: String,
    #[serde(with = "string_or_float", default = "default_activation_price")]
    pub activation_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub orig_type: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub orig_type: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    pub margin_type: String,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    pub leverage: String,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    pub symbol: String,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub margin_available: bool,
    pub update_time: u64,
}

// Account models
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionModeResponse {
    pub dual_side_position: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsMarginResponse {
    pub multi_assets_margin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairQuery {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthQuery {
    pub symbol: String,
    pub limit: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairAndWindowQuery {
    pub symbol: String,
    pub recv_window: u64,
}

/// How long will an order stay alive
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TimeInForce {
    /// Good Till Canceled
    GTC,
    /// Immediate Or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Good till expired
    GTX,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelAllOpenOrdersResponse {
    code: i16,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponse {
    Ack,
    Result,
    Full,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SideEffectType {
    NoSideEffect,
    MarginBuy,
    AutoRepay,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// By default, buy
impl Default for OrderSide {
    fn default() -> Self {
        Self::Buy
    }
}

/// Order types, the following restrictions apply
/// LIMIT_MAKER are LIMIT orders that will be rejected if they would immediately match and trade as a taker.
/// STOP_LOSS and TAKE_PROFIT will execute a MARKET order when the stopPrice is reached.
/// Any LIMIT or LIMIT_MAKER type order can be made an iceberg order by sending an icebergQty.
/// Any order with an icebergQty MUST have timeInForce set to GTC.
/// MARKET orders using quantity specifies how much a user wants to buy or sell based on the market price.
/// MARKET orders using quoteOrderQty specifies the amount the user wants to spend (when buying) or receive (when selling) of the quote asset; the correct quantity will be determined based on the market liquidity and quoteOrderQty.
/// MARKET orders using quoteOrderQty will not break LOT_SIZE filter rules; the order will execute a quantity that will have the notional value as close as possible to quoteOrderQty.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
    #[serde(other)]
    Other,
}

/// By default, use market orders
impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Deserialize, Clone)]
pub struct Empty {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub asset: String,
    pub margin_available: String,
    pub auto_asset_exchange: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum HistoricalTrades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
    pub open_time: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub close_time: u64,
    #[serde(with = "string_or_float")]
    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    #[serde(with = "string_or_float")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(with = "string_or_float")]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ContinuousKlines {
    AllContinuousKlines(Vec<ContinuousKline>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKline {
    pub open_time: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub volume: f64,

    pub close_time: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub quote_asset_volume: f64,

    pub number_of_basic_data: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_base_asset_volume: f64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IndexPriceKlines {
    AllIndexPriceKlines(Vec<IndexPriceKline>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKline {
    pub open_time: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub volume: f64,

    pub close_time: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub quote_asset_volume: f64,

    pub number_of_basic_data: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_base_asset_volume: f64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPriceKlines {
    AllMarkPriceKlines(Vec<MarkPriceKline>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float_opt")]
    pub estimated_settle_price: Option<f64>,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    #[serde(with = "string_or_float")]
    pub interest_rate: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PremiumIndexs {
    AllPremiumIndexs(Vec<PremiumIndex>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price_change: f64,
    #[serde(with = "string_or_float")]
    pub price_change_percent: f64,
    #[serde(with = "string_or_float")]
    pub weighted_avg_price: f64,
    #[serde(with = "string_or_float")]
    pub prev_close_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Ticker24hrs {
    AllTicker24hrs(Vec<Ticker24hr>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TickerPrices {
    AllTickerPrices(Vec<TickerPrice>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum OpenInterestHistorys {
    AllOpenInterestHists(Vec<OpenInterestHistory>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatio {
    pub symbol: String,
    pub long_short_ratio: f64,
    pub long_account: f64,
    pub short_account: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TopLongShortAccountRatios {
    AllTopLongShortAccountRatios(Vec<TopLongShortAccountRatio>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatio {
    pub symbol: String,
    pub long_short_ratio: f64,
    #[serde(rename = "longAccount")]
    pub long_position: f64,
    #[serde(rename = "shortAccount")]
    pub short_position: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TopLongShortPositionRatios {
    AllTopLongShortPositionRatios(Vec<TopLongShortPositionRatio>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatio {
    pub symbol: String,
    pub long_short_ratio: f64,
    pub long_account: f64,
    pub short_account: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GlobalLongShortPositionRatios {
    AllGlobalLongShortPositionRatios(Vec<GlobalLongShortAccountRatio>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TakerlongshortRatio {
    #[serde(with = "string_or_float")]
    pub buy_sell_ratio: f64,
    #[serde(with = "string_or_float")]
    pub buy_vol: f64,
    #[serde(with = "string_or_float")]
    pub sell_vol: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TakerlongshortRatios {
    AllTakerlongshortRatios(Vec<TakerlongshortRatio>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LvtKline {
    pub open_time: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub real_leverage: f64,
    pub close_time: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub quote_asset_volume: f64,
    pub number_of_nav_update: u64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_base_asset_volume: f64,
    #[serde(with = "string_or_float")]
    #[serde(skip_serializing)]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LvtKlines {
    AllLvtKlines(Vec<LvtKline>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexInfo {
    pub symbol: String,
    pub time: u64,
    pub component: String,
    pub base_asset_list: Vec<BaseAsset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseAsset {
    pub base_asset: String,
    pub quote_asset: String,
    #[serde(with = "string_or_float")]
    pub weight_in_quantity: f64,
    #[serde(with = "string_or_float")]
    pub weight_in_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
    pub mins: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistoryQuery {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: u16,
    pub symbol: String,
    pub interval: Option<String>,
    pub period: Option<String>,
}

impl HistoryQuery {
    pub fn validate(&self) -> super::errors::Result<()> {
        if let Some(period) = &self.period {
            if !PERIODS.contains(&period.as_str()) {
                return Err(super::errors::Error::InvalidPeriod(period.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub funding_time: u64,
    #[serde(with = "string_or_float")]
    pub funding_rate: f64,
}

pub static PERIODS: &[&str] = &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"];

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "string_or_float")]
    pub open_interest: f64,
    pub symbol: String,
    pub time: u64,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHistory {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub sum_open_interest: f64,
    #[serde(with = "string_or_float")]
    pub sum_open_interest_value: f64,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub long_account: f64,
    #[serde(with = "string_or_float")]
    pub long_short_ratio: f64,
    #[serde(with = "string_or_float")]
    pub short_account: f64,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub bracket: u8,
    pub initial_leverage: u8,
    pub notional_cap: u64,
    pub notional_floor: u64,
    pub maint_margin_ratio: f64,
    pub cum: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolBrackets {
    pub symbol: String,
    pub brackets: Vec<LeverageBracket>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyCmd {
    pub id: i32,
    pub method: String,
    pub params: (String, bool),
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => super::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(super::string_or_float::deserialize(deserializer)?))
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}

fn default_stop_price() -> f64 {
    0.0
}
fn default_activation_price() -> f64 {
    0.0
}
fn default_price_rate() -> f64 {
    0.0
}
