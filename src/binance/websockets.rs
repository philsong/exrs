use super::config::*;
use super::errors::*;

use awc::ws::Message;
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};

use actix_codec::Framed;
use awc::{
    ws::{Codec, Frame},
    BoxedSocket, Client, ClientResponse,
};
use bytes::Bytes;
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use serde_json::from_slice;
use tokio::sync::mpsc;

pub static STREAM_ENDPOINT: &str = "stream";
pub static WS_ENDPOINT: &str = "ws";
pub static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
pub static OUTBOUND_ACCOUNT_POSITION: &str = "outboundAccountPosition";
pub static EXECUTION_REPORT: &str = "executionReport";
pub static KLINE: &str = "kline";
pub static AGGREGATED_TRADE: &str = "aggTrade";
pub static DEPTH_ORDERBOOK: &str = "depthUpdate";
pub static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
pub static DAYTICKER: &str = "24hrTicker";

pub fn all_ticker_stream() -> &'static str {
    "!ticker@arr"
}

pub fn ticker_stream(symbol: &str) -> String {
    format!("{}@ticker", symbol)
}

pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{}@aggTrade", symbol)
}

pub fn trade_stream(symbol: &str) -> String {
    format!("{}@trade", symbol)
}

pub fn kline_stream(symbol: &str, interval: &str) -> String {
    format!("{}@kline_{}", symbol, interval)
}

pub fn book_ticker_stream(symbol: &str) -> String {
    format!("{}@bookTicker", symbol)
}

pub fn all_book_ticker_stream() -> &'static str {
    "!bookTicker"
}

pub fn all_mini_ticker_stream() -> &'static str {
    "!miniTicker@arr"
}

pub fn mini_ticker_stream(symbol: &str) -> String {
    format!("{}@miniTicker", symbol)
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `levels`: 5, 10 or 20
/// * `update_speed`: 1000 or 100
pub fn partial_book_depth_stream(symbol: &str, levels: u16, update_speed: u16) -> String {
    format!("{}@depth{}@{}ms", symbol, levels, update_speed)
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `update_speed`: 1000 or 100
pub fn diff_book_depth_stream(symbol: &str, update_speed: u16) -> String {
    format!("{}@depth@{}ms", symbol, update_speed)
}

fn combined_stream(streams: Vec<String>) -> String {
    streams.join("/")
}

pub struct WebSockets<WE: serde::de::DeserializeOwned + std::fmt::Debug> {
    pub socket: Option<(ClientResponse, Framed<BoxedSocket, Codec>)>,
    sender: mpsc::Sender<WE>,
    conf: Config,
}

impl<WE: serde::de::DeserializeOwned + std::fmt::Debug> WebSockets<WE> {
    /// New websocket holder with default configuration
    /// # Examples
    /// see examples/binance_WebSockets.rs
    pub fn new(sender: mpsc::Sender<WE>) -> WebSockets<WE> {
        Self::new_with_options(sender, Config::default())
    }

    /// New websocket holder with provided configuration
    /// # Examples
    /// see examples/binance_WebSockets.rs
    pub fn new_with_options(sender: mpsc::Sender<WE>, conf: Config) -> WebSockets<WE> {
        WebSockets {
            socket: None,
            sender: sender,
            conf,
        }
    }

    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.ws_endpoint, WS_ENDPOINT, endpoint);

        let client = Client::builder()
            .max_http_version(awc::http::Version::HTTP_11)
            .finish();

        match client.ws(wss).connect().await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some((_, ref mut socket)) = self.socket {
            socket.close().await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(ClientResponse, Framed<BoxedSocket, Codec>)> {
        &self.socket
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((_, ref mut socket)) = self.socket {
                let message = socket.next().await;
                match message {
                    Some(message) => {
                        let message = message?;
                        debug!("event_loop message - {:?}", message);
                        match message {
                            Frame::Text(msg) => {
                                if msg.is_empty() {
                                    return Ok(());
                                }
                                let event: WE = from_slice(&msg)?;

                                if let Err(e) = self.sender.send(event).await {
                                    return Err(Error::Msg(format!("{:?}", e)));
                                }
                            }
                            Frame::Ping(_) => {
                                socket.send(Message::Pong(Bytes::from_static(b""))).await?;
                            }
                            Frame::Pong(_) | Frame::Binary(_) | Frame::Continuation(_) => {}
                            Frame::Close(e) => {
                                return Err(Error::Msg(format!("Disconnected {:?}", e)));
                            }
                        }
                    }
                    None => {
                        return Err(Error::Msg(format!("Option::unwrap()` on a `None` value.")))
                    }
                }
                actix_rt::task::yield_now().await;
            }
        }
        Ok(())
    }
}
