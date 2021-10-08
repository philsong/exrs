use super::rest_model::string_or_float;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvent {
    BBO(Box<BBOEvent>),
    Kline(Box<KlineEvent>),
    Ticker(Box<TickerEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketResponse {
    Subscription(SubResponse),
    Unsubscription(UnSubResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ping {
    ping: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pong {
    pong: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubRequest {
    sub: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollRequest {
    req: String,
    id: String,
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnSubRequest {
    sub: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubResponse {
    id: String,
    status: String,
    subbed: String,
    #[serde(rename = "ts")]
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnSubResponse {
    id: String,
    status: String,
    unsubbed: String,
    #[serde(rename = "ts")]
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineEvent {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub tick: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub vol: f64,
    #[serde(with = "string_or_float")]
    pub count: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerEvent {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub tick: Ticker,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub vol: f64,
    #[serde(with = "string_or_float")]
    pub count: f64,
    #[serde(with = "string_or_float")]
    pub bid: f64,
    #[serde(with = "string_or_float")]
    pub bid_size: f64,
    #[serde(with = "string_or_float")]
    pub ask: f64,
    #[serde(with = "string_or_float")]
    pub ask_size: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub last_size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BBOEvent {
    // #[serde(rename = "ch")]
    // pub channel: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub tick: BBO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BBO {
    pub symbol: String,
    pub seq_id: u64,
    #[serde(with = "string_or_float")]
    pub ask: f64,
    #[serde(with = "string_or_float")]
    pub ask_size: f64,
    #[serde(with = "string_or_float")]
    pub bid: f64,
    #[serde(with = "string_or_float")]
    pub bid_size: f64,
    pub quote_time: u64,
}
