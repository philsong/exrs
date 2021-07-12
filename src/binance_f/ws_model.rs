use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Empty {}

/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// Stream Name: \<symbol\>@aggTrade
///
/// Update Speed: Real-time
///
/// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggrTradesEvent {
    #[serde(rename = "e")]
    pub event_type: String,

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub mark_price: Decimal,

    #[serde(rename = "i")]
    pub index_price: Decimal,

    #[serde(rename = "P")]
    pub estimated_settle_price: Decimal,

    #[serde(rename = "r")]
    pub funding_rate: Decimal,

    #[serde(rename = "T")]
    pub next_funding_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPricesEvent {
    AllMarkPricesEvent(Vec<MarkPriceEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "c")]
    pub close: Decimal,

    #[serde(rename = "v")]
    pub volume: Decimal,

    #[serde(rename = "n")]
    pub number_of_trades: u64,

    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: Decimal,

    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: Decimal,

    #[serde(rename = "B", skip_serializing)]
    pub ignore: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
    #[serde(rename = "t")]
    pub kline_start_time: u64,

    #[serde(rename = "T")]
    pub kline_close_time: u64,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "c")]
    pub close: Decimal,

    #[serde(rename = "v")]
    pub volume: Decimal,

    #[serde(rename = "n")]
    pub number_of_trades: u64,

    #[serde(rename = "x")]
    pub kline_closed: bool,

    #[serde(rename = "q")]
    pub quote_asset_volume: Decimal,

    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: Decimal,

    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: Decimal,

    #[serde(rename = "B", skip_serializing)]
    pub ignore: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "ps")]
    pub symbol: String,

    #[serde(rename = "ct")]
    pub contract_type: String,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "c")]
    pub close: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "c")]
    pub close: Decimal,

    #[serde(rename = "v")]
    pub total_traded_base_asset_volume: Decimal,

    #[serde(rename = "q")]
    pub total_traded_quote_asset_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AllMiniTickerEvent {
    AllMiniTickerEvent(Vec<MiniTickerEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: Decimal,

    #[serde(rename = "P")]
    pub price_change_percent: Decimal,

    #[serde(rename = "w")]
    pub average_price: Decimal,

    #[serde(rename = "c")]
    pub current_close: Decimal,

    #[serde(rename = "Q")]
    pub current_close_qty: Decimal,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "v")]
    pub total_traded_base_asset_volume: Decimal,

    #[serde(rename = "q")]
    pub total_traded_quote_asset_volume: Decimal,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AllDayTickerEvent {
    AllDayTickerEvent(Vec<DayTickerEvent>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub best_bid: Decimal,

    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,

    #[serde(rename = "a")]
    pub best_ask: Decimal,

    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AllBookTickerEvent {
    AllBookTickerEvent(Vec<BookTickerEvent>),
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
    pub original_qty: Decimal,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "ap")]
    pub average_price: Decimal,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "l")]
    pub order_last_filled_qty: Decimal,

    #[serde(rename = "l")]
    pub order_last_filled_accumulated_qty: Decimal,

    #[serde(rename = "T")]
    pub order_trade_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrderEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "o")]
    pub liquidation_order: Vec<LiquidationOrder>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "pu")]
    #[serde(default)]
    pub previous_final_update_id: Option<u64>,

    #[serde(rename = "b")]
    pub bids: Vec<Bids>,

    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Busket {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "s")]
    pub position: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BLVTInfoEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub blvt_name: String,

    #[serde(rename = "m")]
    pub token_issued: Decimal,

    #[serde(rename = "b")]
    pub baskets: Vec<Busket>,

    #[serde(rename = "n")]
    pub blvt_nav: Decimal,

    #[serde(rename = "l")]
    pub real_leverage: Decimal,

    #[serde(rename = "t")]
    pub traget_leverage: Decimal,

    #[serde(rename = "f")]
    pub funding_rate: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BLVTNAVKline {
    #[serde(rename = "t")]
    pub kline_start_time: u64,

    #[serde(rename = "T")]
    pub kline_close_time: u64,

    #[serde(rename = "s")]
    pub blvt_name: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_nav_update_time: u64,

    #[serde(rename = "L")]
    pub last_nav_update_time: u64,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "c")]
    pub close: Decimal,

    #[serde(rename = "v")]
    pub volume: Decimal,

    #[serde(rename = "n")]
    pub number_of_nav_update: u64,

    #[serde(rename = "x", skip_serializing)]
    pub kline_closed: bool,

    #[serde(rename = "q", skip_serializing)]
    pub quote_asset_volume: Decimal,

    #[serde(rename = "V", skip_serializing)]
    pub taker_buy_base_asset_volume: Decimal,

    #[serde(rename = "Q", skip_serializing)]
    pub taker_buy_quote_asset_volume: Decimal,

    #[serde(rename = "B", skip_serializing)]
    pub ignore: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BLVTNAVKlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub blvt_name: String,

    #[serde(rename = "k")]
    pub blvt_kline: BLVTNAVKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Composition {
    #[serde(rename = "b")]
    pub base_asset: String,

    #[serde(rename = "q")]
    pub quote_asset: String,

    #[serde(rename = "w")]
    pub weight_in_quantity: Decimal,

    #[serde(rename = "W")]
    pub weight_in_percentage: Decimal,

    #[serde(rename = "i")]
    pub index_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompositeIndexEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "c")]
    pub compositions: Vec<Composition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "a")]
    pub data: AccountData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    #[serde(rename = "m")]
    pub event_reason_type: String,

    #[serde(rename = "B")]
    pub balances: Vec<Balance>,

    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,

    #[serde(rename = "wb")]
    pub wallet_balance: Decimal,

    #[serde(rename = "cw")]
    pub cross_wallet_balance: Decimal,

    #[serde(rename = "bc")]
    pub balances_change: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "pa")]
    pub position_amount: Decimal,

    #[serde(rename = "ep")]
    pub entry_price: Decimal,

    #[serde(rename = "cr")]
    pub accumulated_realized: Decimal,

    #[serde(rename = "up")]
    pub unrealized_pnl: Decimal,

    #[serde(rename = "mt")]
    pub margin_type: String,

    #[serde(rename = "iw")]
    pub isolated_wallet: Decimal,

    #[serde(rename = "ps")]
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {}

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
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
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
            Some(v) => crate::binance_f::ws_model::string_or_float::serialize(v, serializer),
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

        Ok(Some(
            crate::binance_f::ws_model::string_or_float::deserialize(deserializer)?,
        ))
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
