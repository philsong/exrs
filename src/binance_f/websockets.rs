use super::config::*;
use super::errors::*;
use super::ws_model::*;
use serde::{Deserialize, Serialize};
use url::Url;

use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};

pub static FUTURES_WS_BASE: &str = "wss://fstream.binance.com";

#[allow(clippy::all)]
enum FuturesWebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl FuturesWebsocketAPI {
    fn params(self, subscription: &str) -> String {
        match self {
            FuturesWebsocketAPI::Default => {
                format!("{}/ws/{}", FUTURES_WS_BASE, subscription)
            }
            FuturesWebsocketAPI::MultiStream => {
                format!("{}/stream?streams={}", FUTURES_WS_BASE, subscription)
            }
            FuturesWebsocketAPI::Custom(url) => url,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FuturesWebsocketEvent {
    AccountUpdate(Box<AccountUpdateEvent>),
    OrderTradeUpdate(Box<OrderTradeUpdateEvent>),
    AggrTrades(Box<AggrTradesEvent>),
    Trade(Box<TradeEvent>),
    OrderBook(Box<OrderBook>),
    DayTicker(Box<DayTickerEvent>),
    MiniTicker(Box<MiniTickerEvent>),
    MiniTickerAll(Box<Vec<MiniTickerEvent>>),
    IndexPrice(Box<IndexPriceEvent>),
    MarkPrice(Box<MarkPriceEvent>),
    MarkPriceAll(Box<Vec<MarkPriceEvent>>),
    DayTickerAll(Box<Vec<DayTickerEvent>>),
    Kline(Box<KlineEvent>),
    ContinuousKline(Box<ContinuousKlineEvent>),
    IndexKline(Box<IndexKlineEvent>),
    Liquidation(Box<LiquidationEvent>),
    DepthOrderBook(Box<DepthOrderBookEvent>),
    BookTicker(Box<BookTickerEvent>),
}

pub struct FuturesWebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(FuturesWebsocketEvent) -> Result<()> + 'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FuturesEvents {
    Vec(Vec<DayTickerEvent>),
    DayTickerEvent(DayTickerEvent),
    BookTickerEvent(BookTickerEvent),
    MiniTickerEvent(MiniTickerEvent),
    VecMiniTickerEvent(Vec<MiniTickerEvent>),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeUpdateEvent(OrderTradeUpdateEvent),
    AggrTradesEvent(AggrTradesEvent),
    IndexPriceEvent(IndexPriceEvent),
    MarkPriceEvent(MarkPriceEvent),
    VecMarkPriceEvent(Vec<MarkPriceEvent>),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    ContinuousKlineEvent(ContinuousKlineEvent),
    IndexKlineEvent(IndexKlineEvent),
    LiquidationEvent(LiquidationEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
}

impl<'a> FuturesWebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, subscription: &'a str) -> Result<()> {
        self.connect_wss(FuturesWebsocketAPI::Default.params(subscription))
    }

    pub fn connect_with_config(&mut self, subscription: &'a str, config: &'a Config) -> Result<()> {
        self.connect_wss(
            FuturesWebsocketAPI::Custom(config.futures_ws_endpoint.clone()).params(subscription),
        )
    }

    pub fn connect_multiple_streams(&mut self, endpoints: &[String]) -> Result<()> {
        self.connect_wss(FuturesWebsocketAPI::MultiStream.params(&endpoints.join("/")))
    }

    fn connect_wss(&mut self, wss: String) -> Result<()> {
        let url = Url::parse(&wss)?;
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        Err(Error::Msg("Not able to close the connection".to_string()))
    }

    pub fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg)
    }

    fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<FuturesEvents>(value) {
            let action = match events {
                FuturesEvents::Vec(v) => FuturesWebsocketEvent::DayTickerAll(Box::new(v)),
                FuturesEvents::DayTickerEvent(v) => FuturesWebsocketEvent::DayTicker(Box::new(v)),
                FuturesEvents::BookTickerEvent(v) => FuturesWebsocketEvent::BookTicker(Box::new(v)),
                FuturesEvents::MiniTickerEvent(v) => FuturesWebsocketEvent::MiniTicker(Box::new(v)),
                FuturesEvents::VecMiniTickerEvent(v) => {
                    FuturesWebsocketEvent::MiniTickerAll(Box::new(v))
                }
                FuturesEvents::AccountUpdateEvent(v) => {
                    FuturesWebsocketEvent::AccountUpdate(Box::new(v))
                }
                FuturesEvents::OrderTradeUpdateEvent(v) => {
                    FuturesWebsocketEvent::OrderTradeUpdate(Box::new(v))
                }
                FuturesEvents::IndexPriceEvent(v) => FuturesWebsocketEvent::IndexPrice(Box::new(v)),
                FuturesEvents::MarkPriceEvent(v) => FuturesWebsocketEvent::MarkPrice(Box::new(v)),
                FuturesEvents::VecMarkPriceEvent(v) => {
                    FuturesWebsocketEvent::MarkPriceAll(Box::new(v))
                }
                FuturesEvents::TradeEvent(v) => FuturesWebsocketEvent::Trade(Box::new(v)),
                FuturesEvents::ContinuousKlineEvent(v) => {
                    FuturesWebsocketEvent::ContinuousKline(Box::new(v))
                }
                FuturesEvents::IndexKlineEvent(v) => FuturesWebsocketEvent::IndexKline(Box::new(v)),
                FuturesEvents::LiquidationEvent(v) => {
                    FuturesWebsocketEvent::Liquidation(Box::new(v))
                }
                FuturesEvents::KlineEvent(v) => FuturesWebsocketEvent::Kline(Box::new(v)),
                FuturesEvents::OrderBook(v) => FuturesWebsocketEvent::OrderBook(Box::new(v)),
                FuturesEvents::DepthOrderBookEvent(v) => {
                    FuturesWebsocketEvent::DepthOrderBook(Box::new(v))
                }
                FuturesEvents::AggrTradesEvent(v) => FuturesWebsocketEvent::AggrTrades(Box::new(v)),
            };
            (self.handler)(action)?;
        }
        Ok(())
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg) {
                            return Err(Error::Msg(format!(
                                "Error on handling stream message: {}",
                                e
                            )));
                        }
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => (),
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {:?}", e)));
                    }
                }
            }
        }
        Ok(())
    }
}
