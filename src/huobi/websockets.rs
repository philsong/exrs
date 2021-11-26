use super::config::*;
use super::errors::*;
use super::ws_model::WebsocketResponse;

use log::debug;
use std::str::from_utf8;
use std::sync::atomic::{AtomicBool, Ordering};

use actix_codec::Framed;
use awc::{
    ws::{Codec, Frame, Message},
    BoxedSocket, Client, ClientResponse,
};
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use libdeflater::Decompressor;
use serde_json::from_slice;
use tokio::sync::mpsc;

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

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((_, ref mut socket)) = self.socket {
                let message = socket.next().await;
                match message {
                    Some(message) => {
                        let message = message?;
                        debug!("event_loop message - {:?}", message);
                        match message {
                            Frame::Binary(msg) => {
                                if msg.is_empty() {
                                    return Ok(());
                                }

                                let msg = huobi_decompress(msg.to_vec()).unwrap();

                                if let Ok(event) = from_slice(&msg) {
                                    if let Err(e) = self.sender.send(event).await {
                                        return Err(Error::Msg(format!("{:?}", e)));
                                    }
                                } else if let Ok(response) = from_slice::<WebsocketResponse>(&msg) {
                                    println!("WebsocketResponse: {:?}", response);
                                } else if from_utf8(&msg)?.starts_with(r#"{"pi"#) {
                                    socket
                                        .send(Message::Text(
                                            from_utf8(&msg)?.replace("i", "o").into(),
                                        ))
                                        .await?;
                                } else {
                                    return Err(Error::Msg(format!(
                                        "Websocket Parse failed {:?}",
                                        msg
                                    )));
                                }
                            }
                            Frame::Ping(_)
                            | Frame::Pong(_)
                            | Frame::Text(_)
                            | Frame::Continuation(_) => {}
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

fn huobi_decompress(msg: Vec<u8>) -> Result<Vec<u8>> {
    let isize_start = msg.len() - 4;
    let isize_bytes = &msg[isize_start..];
    let mut ret: u32 = isize_bytes[0] as u32;
    ret |= (isize_bytes[1] as u32) << 8;
    ret |= (isize_bytes[2] as u32) << 16;
    ret |= (isize_bytes[3] as u32) << 26;

    let mut decompressor = Decompressor::new();
    let mut outbuf = vec![0; ret as usize];
    decompressor.gzip_decompress(&msg, &mut outbuf).unwrap();
    Ok(outbuf)
}
