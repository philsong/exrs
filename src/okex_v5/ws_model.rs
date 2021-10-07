use super::rest_model::{string_or_float, string_or_uint};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub channel: String,
    pub inst_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvent {
    Instruments(Box<InstrumentsEvent>),
    Ticker(Box<TickerEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub op: String,
    pub args: Vec<LoginConfig>
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
    pub args: Vec<Arg>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionResponse {
    pub event: String,
    pub arg: Arg
}

#[derive(Debug,Serialize, Deserialize, Clone)]
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
    pub args: Vec<Arg>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnsubscriptionResponse {
    pub event: String,
    pub arg: Arg
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
    #[serde(rename = "ts", with = "string_or_uint")]
    pub timestamp: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandiesticksEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradesEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPriceEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceCandlesticksEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionSummaryEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexCandlesticksEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusEvent {
    
}

// Private Channels Starts from here
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionsEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAndPositionEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrdersEvent {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceAlgoOrdersEvent {
    
}
