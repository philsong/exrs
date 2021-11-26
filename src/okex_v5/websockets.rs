use crate::okex_v5::util::get_timestamp;

use super::config::*;
use super::errors::*;
use super::rest_model::OrderType;
use super::ws_model::{LoginConfig, LoginRequest, WebsocketResponse};

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
use hmac_sha256::HMAC;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use tokio::sync::mpsc;
use uuid::Uuid;

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
        let wss: String = format!("{}/{}", self.conf.ws_endpoint, endpoint);

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

    pub async fn subscribe_request(&mut self, request: &str) -> Result<()> {
        if let Some((_, ref mut socket)) = self.socket {
            socket.send(Message::Text(request.into())).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to send requests".to_string()))
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

    pub async fn login(
        &mut self,
        api_key: String,
        secret_key: String,
        passphrase: String,
    ) -> Result<()> {
        // {
        //     "op": "login",
        //     "args": [
        //       {
        //         "apiKey": "985d5b66-57ce-40fb-b714-afc0b9787083",
        //         "passphrase": "123456",
        //         "timestamp": "1538054050",
        //         "sign": "7L+zFQ+CEgGu5rzCj4+BdV2/uUHGqddA9pI6ztsRRPs="
        //       }
        //     ]
        // }

        let timestamp = (get_timestamp().unwrap() / 1000).to_string();

        let pre_hash = format!(
            "{}{}{}",
            timestamp,
            Method::GET.as_str(),
            "/users/self/verify"
        );

        let signature = base64::encode(HMAC::mac(pre_hash.as_bytes(), secret_key.as_bytes()));

        let login_cfg = LoginConfig {
            api_key: api_key,
            passphrase: passphrase,
            timestamp: timestamp,
            sign: signature,
        };

        let login_req = LoginRequest {
            op: "login".to_string(),
            args: vec![login_cfg],
        };

        if let Some((_, ref mut socket)) = self.socket {
            socket
                .send(Message::Text(
                    serde_json::to_string(&login_req).unwrap().into(),
                ))
                .await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10000));

        while running.load(Ordering::Relaxed) {
            if let Some((_, ref mut socket)) = self.socket {
                tokio::select! {
                    message = socket.next() => {
                        match message {
                            Some(msg) => {
                                let msg = msg?;
                                match msg {
                                    Frame::Text(text) => {
                                        if text.is_empty() {
                                            return Ok(());
                                        }
                                        if let Ok(event) = from_slice(&text) {
                                            if let Err(e) = self.sender.send(event).await {
                                                return Err(Error::Msg(format!("{:?}", e)));
                                            }
                                        } else if let Ok(response) = from_slice::<WebsocketResponse>(&text) {
                                            println!("WebsocketResponse: {:?}", response);
                                        } else {
                                            return Err(Error::Msg(format!("Websocket Parse failed {:?}", text)));
                                        }
                                    },
                                    Frame::Binary(_) | Frame::Continuation(_) | Frame::Ping(_) => {}
                                    Frame::Pong(pong) => {
                                        debug!("pong： {:?}", pong);
                                    },
                                    Frame::Close(e) => {
                                        return Err(Error::Msg(format!("Disconnected {:?}", e)));
                                    }
                                }
                            },
                            None => return Err(Error::Msg(format!("Option::unwrap()` on a `None` value."))),
                        }
                    }
                    _ = interval.tick() => {
                        socket.send(Message::Ping(Bytes::from_static(b""))).await?;
                        debug!("ping: {:?}", Message::Ping(Bytes::from_static(b"")));
                    }
                }
            }
        }
        Ok(())
    }

    // trade start from here
    pub async fn place_order(&mut self, order: WSOrder) -> Result<()> {
        if let Some((_, ref mut socket)) = self.socket {
            let ws_order = WSOrderRequest {
                id: Uuid::new_v4().to_string(),
                op: "order".to_string(),
                args: vec![order],
            };

            let text = serde_json::to_string(&ws_order)?;
            socket.send(Message::Text(text.into())).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to send requests".to_string()))
        }
    }

    pub async fn place_multipy_order(&mut self, orders: Vec<WSOrder>) -> Result<()> {
        if let Some((_, ref mut socket)) = self.socket {
            let ws_orders = WSOrderRequest {
                id: Uuid::new_v4().to_string(),
                op: "batch-orders".to_string(),
                args: orders,
            };

            let text = serde_json::to_string(&ws_orders)?;
            socket.send(Message::Text(text.into())).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to send requests".to_string()))
        }
    }

    pub async fn limit_buy(
        &mut self,
        symbol: impl Into<String>,
        qty: impl Into<String>,
        price: impl Into<String>,
        order_type: OrderType,
    ) -> Result<()> {
        let order = WSOrder {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: None,
            tag: None,
            side: OrderSide::Buy,
            position_side: None, // None for net mode
            order_type: order_type,
            qty: qty.into(),
            price: Some(price.into()),
            reduce_only: None,
            target_currency: None,
        };
        self.place_order(order).await?;
        Ok(())
    }

    pub async fn limit_sell(
        &mut self,
        symbol: impl Into<String>,
        qty: impl Into<String>,
        price: impl Into<String>,
        order_type: OrderType,
    ) -> Result<()> {
        let order = WSOrder {
            symbol: symbol.into(),
            trade_mode: TradeMode::Cross,
            currency: None,
            client_order_id: None,
            tag: None,
            side: OrderSide::Sell,
            position_side: None, // None for net mode
            order_type: order_type,
            qty: qty.into(),
            price: Some(price.into()),
            reduce_only: None,
            target_currency: None,
        };
        self.place_order(order).await?;
        Ok(())
    }

    pub async fn market_buy() {}

    pub async fn market_sell() {}

    pub async fn cancel_order() {}

    pub async fn amend_order() {}

    pub async fn amend_multiple_order() {}
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionSide {
    Net,
    Long,
    Short,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TradeMode {
    Isolated,
    Cross,
    Cash,
}

/// By default, Cross
impl Default for TradeMode {
    fn default() -> Self {
        Self::Cross
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSOrderRequest {
    pub id: String,
    pub op: String,
    pub args: Vec<WSOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSOrder {
    #[serde(rename = "inst_id")]
    pub symbol: String,
    #[serde(rename = "td_mode")]
    pub trade_mode: TradeMode,
    #[serde(rename = "ccy")]
    pub currency: Option<String>,
    #[serde(rename = "clOrdId")]
    pub client_order_id: Option<String>,
    pub tag: Option<String>,
    pub side: OrderSide,
    #[serde(rename = "posSide")]
    pub position_side: Option<String>,
    #[serde(rename = "ord_type")]
    pub order_type: OrderType,
    #[serde(rename = "sz")]
    pub qty: String,
    #[serde(rename = "px")]
    pub price: Option<String>,
    pub reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy")]
    pub target_currency: Option<String>,
}
