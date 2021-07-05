use crate::binance_f::config::*;
use crate::binance_f::errors::*;
use crate::binance_f::ws_model::*;
use serde::{Deserialize, Serialize};
use url::Url;

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{connect, Message};

#[allow(clippy::all)]
enum WebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl WebsocketAPI {
    fn params(self, subscription: &str) -> String {
        match self {
            WebsocketAPI::Default => format!("wss://stream.binance.com:9443/ws/{}", subscription),
            WebsocketAPI::MultiStream => format!(
                "wss://stream.binance.com:9443/stream?streams={}",
                subscription
            ),
            WebsocketAPI::Custom(url) => url,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    // Market Data endpoints
    AggrTrades(AggrTradesEvent),
    MarkPrice(MarkPriceEvent),
    AllMarkPrices(MarkPricesEvent),
    Kline(KlineEvent),
    ContinuousKline(ContinuousKlineEvent),
    MiniTicker(MiniTickerEvent),
    AllMiniTicker(AllMiniTickerEvent),
    DayTicker(DayTickerEvent),
    AllDayTickerEvent(AllDayTickerEvent),
    BookTicker(BookTickerEvent),
    AllBookTicker(AllBookTickerEvent),
    LiquidationOrderEvent(LiquidationOrderEvent),
    AllLiquidationOrderEvent(LiquidationOrderEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BLVTInfo(BLVTInfoEvent),
    BLVTNAVKline(BLVTNAVKlineEvent),
    // User Stream endpoints
}
