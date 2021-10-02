use std::time::Duration;

use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use ring::hmac;
use serde::de;
use serde::de::DeserializeOwned;
use serde_json::from_str;

use super::errors::error_messages;
use super::errors::*;
// use super::rest_model::PairQuery;
use super::util::{build_request_p, build_signed_request_p};

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner: reqwest::Client,
    host: String,
}


impl Client {
    /// Returns a client based on the specified host and credentials
    /// Credentials do not need to be specified when using public endpoints
    /// Host is mandatory
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        let builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        let builder = builder.timeout(Duration::from_secs(2));
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host,
        }
    }
}