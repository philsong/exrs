use super::config::*;
use super::errors::*;
use super::ws_model::*;
use futures::Future;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use url::Url;

use async_recursion::async_recursion;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use tungstenite::handshake::client::Response;

pub static WS_BASE: &str = "wss://api.huobi.pro/";
pub static WS_ENDPOINT: &str = "ws";
pub static MBP_ENDPOINT: &str = "feed";
pub static ACCOUNT_ENDPOINT: &str = "ws/v2";

enum WebsocketAPI {
    Default,
    MBP,
    Account,
    Custom(String),
}

impl WebsocketAPI {
    fn params(self, subscription: &str) -> String {
        match self {
            WebsocketAPI::Default => {
                format!("{}/{}/{}", WS_BASE, WS_ENDPOINT, subscription)
            }
            WebsocketAPI::MBP => {
                format!("{}/{}/{}", WS_BASE, MBP_ENDPOINT, subscription)
            }
            WebsocketAPI::Account => {
                format!("{}/{}/{}", WS_BASE, ACCOUNT_ENDPOINT, subscription)
            }
            WebsocketAPI::Custom(url) => url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    Ping(Box<Ping>),
    SubResponse(Box<SubResponseEvent>),
    UnSubResponse(Box<UnSubResponseEvent>),
    Kline(Box<KlineEvent>),
    Ticker(Box<TickerEvent>),
}

pub struct WebSockets {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    pub sender: Sender<WebsocketEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Events {
    KlineEvent(KlineEvent),
    TickerEvent(TickerEvent),
}

impl WebSockets {
    pub fn new(sender: Sender<WebsocketEvent>) -> Self {
        WebSockets {
            socket: None,
            sender: sender,
        }
    }

    pub async fn connect(&mut self, subscription: &str) -> Result<()> {
        self.connect_wss(WebsocketAPI::Default.params(subscription))
            .await
    }

    pub async fn connect_with_config(&mut self, subscription: &str, config: Config) -> Result<()> {
        self.connect_wss(WebsocketAPI::Custom(config.ws_endpoint.clone()).params(subscription))
            .await
    }

    async fn connect_wss(&mut self, wss: String) -> Result<()> {
        let url = Url::parse(&wss)?;
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            return Ok(());
        }
        Err(Error::Msg("Not able to close the connection".to_string()))
    }

    pub async fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg).await
    }

    #[async_recursion]
    async fn handle_msg<'a>(&mut self, msg: &'a str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("tick") {
            self.handle_msg(&data.to_string()).await?;
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<Events>(value) {
            let action = match events {
                Events::KlineEvent(v) => WebsocketEvent::Kline(Box::new(v)),
                Events::TickerEvent(v) => WebsocketEvent::Ticker(Box::new(v)),
            };
            self.sender.send(action).await.unwrap();
        }
        Ok(())
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.next().await.unwrap()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg).await {
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
