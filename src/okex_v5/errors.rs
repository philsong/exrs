use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

use super::rest_model::string_or_u16;
use super::ws_model::WebsocketEvent;

#[derive(Debug, Clone, Deserialize, Error)]
#[error("code: {code}, msg: {msg}")]
pub struct OkexContentError {
    #[serde(with = "string_or_u16")]
    pub code: u16,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// First errors are technical errors
/// All unhandled Okex content errors are OkexError
/// The rest are Okex content errors that are properly handled
/// Unhandled Okex errors are Msg
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Qs(#[from] serde_qs::Error),
    #[error(transparent)]
    WsProtocolError(#[from] awc::error::WsProtocolError),
    #[error(transparent)]
    SendError(tokio::sync::mpsc::error::SendError<WebsocketEvent>),
    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    UTF8Err(#[from] std::str::Utf8Error),
    #[error("{response}")]
    OkexError {
        #[from]
        response: OkexContentError,
    },
    #[error("invalid listen key : {0}")]
    InvalidListenKey(String),
    #[error("unknown symbol {0}")]
    UnknownSymbol(String),
    #[error("{msg}")]
    InvalidOrderError { msg: String },
    #[error("invalid price")]
    InvalidPrice,
    #[error("invalid period {0}")]
    InvalidPeriod(String),
    #[error("internal server error")]
    InternalServerError,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Msg(String),
}

/// Custom error messages
pub mod error_messages {
    pub const INVALID_PRICE: &str = "Invalid price.";
}

pub type Result<T> = core::result::Result<T, Error>;
