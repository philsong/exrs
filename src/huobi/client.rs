use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use ring::hmac;
use serde::de;
use serde_json::from_str;
use std::time::Duration;

// use crate::errors::error_messages;
// use crate::errors::*;
// use crate::util::{build_request_p, build_signed_request_p};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    api_secret: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(api_key: Option<String>, api_secret: Option<String>, host: String) -> Self {
        let builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        let builder = builder.timeout(Duration::from_secs(2));
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            api_secret: api_secret.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host,
        }
    }
}
