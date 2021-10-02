use super::config;
use super::config::*;
use super::errors::*;

use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};

use actix_codec::Framed;
use awc::{
    ws::{Codec, Frame},
    BoxedSocket, Client, ClientResponse,
};
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use serde_json::from_slice;
use tokio::sync::mpsc;

pub enum WebsocketAPI {
    Public,
    Private,
    Custom(String),
}

impl WebsocketAPI {
    fn params(self, config: Config) -> String {
        match self {
            WebsocketAPI::Public => config.ws_public,
            WebsocketAPI::Private => config.ws_private,
            WebsocketAPI::Custom(url) => url,
        }
    }
}

pub struct WebSockets<WE: serde::de::DeserializeOwned> {
    pub socket: Option<(ClientResponse, Framed<BoxedSocket, Codec>)>,
    sender: mpsc::Sender<WE>,
    conf: Config,
}

impl<WE: serde::de::DeserializeOwned> WebSockets<WE> {
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

    pub async fn connect_public(&mut self, config: Config) -> Result<()> {
        self.connect_wss(WebsocketAPI::Public.params(config)).await
    }

    pub async fn connect_private(&mut self, config: Config) -> Result<()> {
        self.connect_wss(WebsocketAPI::Private.params(config)).await
    }

    pub async fn connect_wss(&mut self, wss: String) -> Result<()> {

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
                let message = socket.next().await.unwrap()?;
                debug!("event_loop message - {:?}", message);
                match message {
                    Frame::Text(msg) => {
                        if msg.is_empty() {
                            return Ok(());
                        }
                        let event: WE = from_slice(&msg)?;

                        if let Err(_e) = self.sender.send(event).await {
                            println!("SendError<WE>");
                        }
                    }
                    Frame::Ping(_) | Frame::Pong(_) | Frame::Binary(_) | Frame::Continuation(_) => {
                    }
                    Frame::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {:?}", e)));
                    }
                }
                actix_rt::task::yield_now().await;
            }
        }
        Ok(())
    }

    // trade start from here
    pub async fn place_order() {

    }

    pub async fn place_multiple_orders() {

    }

    pub async fn cancel_order() {

    }

    pub async fn amend_order() {

    }

    pub async fn amend_multiple_order() {

    }
}
