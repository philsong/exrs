use super::rest_model::{string_or_float, string_or_float_opt, string_or_u64};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvent {
    Instruments(Box<InstrumentsEvent>),
    Ticker(Box<TickerEvent>),
    OrderBook(Box<OrderBookEvent>),
    Account(Box<AccountEvent>),
    Position(Box<PositionsEvent>),
    BalancePosition(Box<BalancePositionEvent>),
    Order(Box<OrderEvent>),
    AlgoOrders(Box<AlgoOrdersEvent>),
    AdvanceAlgoOrdersEvent(Box<AdvanceAlgoOrdersEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketResponse {
    Login(LoginResponse),
    Subscription(SubscriptionResponse),
    FailureSubscription(FailureSubscriptionResponse),
    Unsubscription(UnsubscriptionResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub op: String,
    pub args: Vec<LoginConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginConfig {
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: String,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub event: String,
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequest {
    pub op: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionResponse {
    pub event: String,
    pub arg: Arg,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FailureSubscriptionResponse {
    pub event: String,
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscriptionRequest {
    pub op: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnsubscriptionResponse {
    pub event: String,
    pub arg: Arg,
}

// Public Channels Starts from here
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsEvent {
    pub arg: Arg,
    pub data: Vec<Instruments>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instruments {
    pub inst_type: String,
    pub inst_id: String,
    pub uly: String,
    pub category: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub settle_ccy: String,
    pub ct_val: String,
    pub ct_mult: String,
    pub ct_val_ccy: String,
    pub opt_type: String,
    pub stk: String,
    pub list_time: String,
    pub exp_time: String,
    pub tick_sz: String,
    pub lot_sz: String,
    pub min_sz: String,
    pub ct_type: String,
    pub alias: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerEvent {
    pub arg: Arg,
    pub data: Vec<Ticker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub inst_type: String,
    pub inst_id: String,
    #[serde(with = "string_or_float")]
    pub last: f64,
    #[serde(rename = "lastSz", with = "string_or_float")]
    pub last_qty: f64,
    #[serde(rename = "askPx", with = "string_or_float")]
    pub best_ask: f64,
    #[serde(rename = "askSz", with = "string_or_float")]
    pub best_ask_qty: f64,
    #[serde(rename = "bidPx", with = "string_or_float")]
    pub best_bid: f64,
    #[serde(rename = "bidSz", with = "string_or_float")]
    pub best_bid_qty: f64,
    #[serde(rename = "open24h", with = "string_or_float")]
    pub open24_h: f64,
    #[serde(rename = "high24h", with = "string_or_float")]
    pub high24_h: f64,
    #[serde(rename = "low24h", with = "string_or_float")]
    pub low24_h: f64,
    #[serde(with = "string_or_float")]
    pub sod_utc0: f64,
    #[serde(with = "string_or_float")]
    pub sod_utc8: f64,
    #[serde(rename = "volCcy24h", with = "string_or_float")]
    pub vol_ccy24_h: f64,
    #[serde(rename = "vol24h", with = "string_or_float")]
    pub vol24_h: f64,
    #[serde(rename = "ts", with = "string_or_u64")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandiesticksEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradesEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPriceEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceCandlesticksEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookEvent {
    pub arg: Arg,
    pub data: Vec<OrderBook>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,
    pub inst_id: String,
    #[serde(rename = "ts", with = "string_or_u64")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionSummaryEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexCandlesticksEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusEvent {}

// Private Channels Starts from her
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountEvent {
    pub arg: Arg,
    pub data: Vec<Account>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountArg {
//     pub channel: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub ccy: Option<String>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub u_time: String,
    pub total_eq: String,
    pub iso_eq: String,
    pub adj_eq: String,
    pub ord_froz: String,
    pub imr: String,
    pub mmr: String,
    pub notional_usd: String,
    pub mgn_ratio: String,
    pub details: Vec<Detail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(with = "string_or_float")]
    pub avail_bal: f64,
    #[serde(with = "string_or_float")]
    pub avail_eq: f64,
    pub ccy: String,
    #[serde(with = "string_or_float")]
    pub cash_bal: f64,
    #[serde(with = "string_or_u64")]
    pub u_time: u64,
    #[serde(with = "string_or_float")]
    pub dis_eq: f64,
    #[serde(with = "string_or_float")]
    pub eq: f64,
    #[serde(with = "string_or_float")]
    pub eq_usd: f64,
    #[serde(with = "string_or_float")]
    pub frozen_bal: f64,
    #[serde(with = "string_or_float")]
    pub interest: f64,
    #[serde(with = "string_or_float")]
    pub iso_eq: f64,
    #[serde(with = "string_or_float")]
    pub liab: f64,
    #[serde(with = "string_or_float")]
    pub max_loan: f64,
    #[serde(with = "string_or_float")]
    pub mgn_ratio: f64,
    #[serde(with = "string_or_float")]
    pub notional_lever: f64,
    #[serde(with = "string_or_float")]
    pub ord_frozen: f64,
    #[serde(with = "string_or_float")]
    pub upl: f64,
    #[serde(with = "string_or_float")]
    pub upl_liab: f64,
    #[serde(with = "string_or_float")]
    pub cross_liab: f64,
    #[serde(with = "string_or_float")]
    pub iso_liab: f64,
    #[serde(with = "string_or_float")]
    pub coin_usd_price: f64,
    #[serde(with = "string_or_float")]
    pub stgy_eq: f64,
    #[serde(with = "string_or_float")]
    pub iso_upl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsEvent {
    pub arg: Arg,
    pub data: Vec<Positions>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Positions {
    pub adl: String,
    pub avail_pos: String,
    pub avg_px: String,
    pub c_time: String,
    pub ccy: String,
    #[serde(rename = "deltaBS")]
    pub delta_bs: String,
    #[serde(rename = "deltaPA")]
    pub delta_pa: String,
    #[serde(rename = "gammaBS")]
    pub gamma_bs: String,
    #[serde(rename = "gammaPA")]
    pub gamma_pa: String,
    pub imr: String,
    pub inst_id: String,
    pub inst_type: String,
    pub interest: String,
    pub last: String,
    pub lever: String,
    pub liab: String,
    pub liab_ccy: String,
    pub liq_px: String,
    pub margin: String,
    pub mark_px: String,
    pub mgn_mode: String,
    pub mgn_ratio: String,
    pub mmr: String,
    pub notional_usd: String,
    pub opt_val: String,
    pub p_time: String,
    pub pos: String,
    pub pos_ccy: String,
    pub pos_id: String,
    pub pos_side: String,
    #[serde(rename = "thetaBS")]
    pub theta_bs: String,
    #[serde(rename = "thetaPA")]
    pub theta_pa: String,
    pub trade_id: String,
    pub u_time: String,
    pub upl: String,
    pub upl_ratio: String,
    #[serde(rename = "vegaBS")]
    pub vega_bs: String,
    #[serde(rename = "vegaPA")]
    pub vega_pa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancePositionEvent {
    pub arg: Arg,
    pub data: Vec<BalancePosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancePosition {
    pub p_time: String,
    pub event_type: String,
    pub bal_data: Vec<BalDaum>,
    pub pos_data: Vec<PosDaum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalDaum {
    pub ccy: String,
    pub cash_bal: String,
    pub u_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PosDaum {
    pub pos_id: String,
    pub trade_id: String,
    pub inst_id: String,
    pub inst_type: String,
    pub mgn_mode: String,
    pub pos_side: String,
    pub pos: String,
    pub ccy: String,
    pub pos_ccy: String,
    pub avg_px: String,
    #[serde(rename = "uTIme")]
    pub u_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub arg: Arg,
    pub data: Vec<Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub inst_type: String,
    pub inst_id: String,
    pub ccy: String,
    pub ord_id: String,
    pub cl_ord_id: String,
    pub tag: String,
    pub px: String,
    pub sz: String,
    pub notional_usd: String,
    pub ord_type: String,
    pub side: String,
    pub pos_side: String,
    pub td_mode: String,
    pub tgt_ccy: String,
    pub fill_sz: String,
    pub fill_px: String,
    pub trade_id: String,
    pub acc_fill_sz: String,
    pub fill_notional_usd: String,
    pub fill_time: String,
    pub fill_fee: String,
    pub fill_fee_ccy: String,
    pub exec_type: String,
    pub state: String,
    pub avg_px: String,
    pub lever: String,
    pub tp_trigger_px: String,
    pub tp_ord_px: String,
    pub sl_trigger_px: String,
    pub sl_ord_px: String,
    pub fee_ccy: String,
    pub fee: String,
    pub rebate_ccy: String,
    pub rebate: String,
    pub pnl: String,
    pub category: String,
    pub u_time: String,
    pub c_time: String,
    pub req_id: String,
    pub amend_result: String,
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrdersEvent {
    pub arg: Arg,
    pub data: Vec<AlgoOrders>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrders {
    pub inst_type: String,
    pub inst_id: String,
    pub ord_id: String,
    pub ccy: String,
    pub algo_id: String,
    pub px: String,
    pub sz: String,
    pub td_mode: String,
    pub tgt_ccy: String,
    pub notional_usd: String,
    pub ord_type: String,
    pub side: String,
    pub pos_side: String,
    pub state: String,
    pub lever: String,
    pub tp_trigger_px: String,
    pub tp_ord_px: String,
    pub sl_trigger_px: String,
    pub trigger_px: String,
    pub ord_px: String,
    pub actual_sz: String,
    pub actual_px: String,
    pub actual_side: String,
    pub trigger_time: String,
    pub c_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceAlgoOrdersEvent {
    pub arg: Arg,
    pub data: Vec<AdvanceAlgoOrders>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceAlgoOrders {
    pub actual_px: String,
    pub actual_side: String,
    pub actual_sz: String,
    pub algo_id: String,
    pub c_time: String,
    pub ccy: String,
    pub count: String,
    pub inst_id: String,
    pub inst_type: String,
    pub lever: String,
    pub notional_usd: String,
    pub ord_px: String,
    pub ord_type: String,
    pub p_time: String,
    pub pos_side: String,
    pub px_limit: String,
    pub px_spread: String,
    pub px_var: String,
    pub side: String,
    pub sl_ord_px: String,
    pub sl_trigger_px: String,
    pub state: String,
    pub sz: String,
    pub sz_limit: String,
    pub td_mode: String,
    pub time_interval: String,
    pub tp_ord_px: String,
    pub tp_trigger_px: String,
    pub trigger_px: String,
    pub trigger_time: String,
}
