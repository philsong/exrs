use std::collections::BTreeMap;
use std::time::Duration;

use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use reqwest::Method;
use ring::hmac;
use serde::de;
use serde::de::DeserializeOwned;
use serde_json::from_str;

use super::errors::error_messages;
use super::errors::*;
use super::rest_model::PairQuery;
use super::util::{build_request_p, get_timestamp};

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    passphrase: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    /// Returns a client based on the specified host and credentials
    /// Credentials do not need to be specified when using public endpoints
    /// Host is mandatory
    pub fn new(api_key: Option<String>, secret_key: Option<String>, passphrase: Option<String>, host: String) -> Self {
        let builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        let builder = builder.timeout(Duration::from_secs(2));
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            passphrase: passphrase.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host,
        }
    }

    pub async fn post_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = format!("{}{}?{}", self.host, endpoint, request);
        let response = self.inner
            .post(url.as_str())
            .headers(self.build_signed_headers(true, Method::POST, endpoint, request)?)
            .send()
            .await?;
        self.handler(response).await
    }


    pub async fn post_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self, 
        endpoint: &str, 
        payload: P
    ) -> Result<T> {
        let request = build_request_p(payload)?;
        let string = self.post_signed(endpoint, &request).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    pub async fn get_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = format!("{}{}?{}", self.host, endpoint, request);

        let response = self
            .inner
            .clone()
            .get(url.as_str())
            .headers(self.build_signed_headers(true, Method::GET, endpoint, request)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn get_signed_d<T: de::DeserializeOwned>(
        &self, 
        endpoint: &str,
        request: &str
    ) -> Result<T> {
        let r = self.get_signed(endpoint, request).await?;
        let t = from_str(r.as_str())?;
        Ok(t)
    }

    // pub async fn get_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
    //     &self, 
    //     endpoint: &str, 
    //     payload: Option<P>,
    // ) -> Result<T> {
    //     let req = if let Some(p) = payload {
    //         p
    //         // self.build_signed_headers_p(true, Method::GET, endpoint, p)?
    //     } else {
    //         let option: Option<PairQuery> = None;
    //         // self.build_signed_headers_p(true, Method::GET, endpoint, option)?
    //         option
    //     };
    //     let string = self.get_signed(endpoint, &req).await?;
    //     let data: &str = string.as_str();
    //     let t = from_str(data)?;
    //     Ok(t)
    // }

    pub fn build_headers(&self, content_type: bool) {
        let mut custon_headers = HeaderMap::new();

        custon_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custon_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }
    }

    pub fn build_signed_headers(&self, content_type: bool, method: Method, endpoint: &str, request: &str) -> Result<HeaderMap> {
        let mut custon_headers = HeaderMap::new();

        custon_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custon_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        if let Ok(timestamp) = get_timestamp() {
            let pre_hash = format!("{}{}{}?{}{}", timestamp, method.as_str(), endpoint, request, self.secret_key);

            let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
            let signature = base64::encode(hmac::sign(&signed_key, pre_hash.as_bytes()).as_ref());

            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-KEY"),
                HeaderValue::from_str(self.api_key.as_str())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-SIGN"),
                HeaderValue::from_str(&signature.as_str())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-TIMESTAMP"),
                HeaderValue::from_str(&timestamp.to_string())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-PASSPHRASE"),
                HeaderValue::from_str(self.passphrase.as_str())?,
            );
        } else {
            return Err(Error::Msg("build_headers Failed to get timestamp".to_string()))
        }

        Ok(custon_headers)
    }

    pub fn build_signed_headers_p<S>(&self, content_type: bool, method: Method, endpoint: &str, payload: S) -> Result<HeaderMap> 
    where
        S: serde::Serialize
    {
        let query_string = qs::to_string(&payload)?;
        let mut custon_headers = HeaderMap::new();

        custon_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custon_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        if let Ok(timestamp) = get_timestamp() {
            let pre_hash = format!("{}{}{}?{}{}", timestamp, method.as_str(), endpoint, query_string, self.secret_key);

            let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
            let signature = base64::encode(hmac::sign(&signed_key, pre_hash.as_bytes()).as_ref());

            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-KEY"),
                HeaderValue::from_str(self.api_key.as_str())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-SIGN"),
                HeaderValue::from_str(&signature.as_str())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-TIMESTAMP"),
                HeaderValue::from_str(&timestamp.to_string())?,
            );
            custon_headers.insert(
                HeaderName::from_static("OK-ACCESS-PASSPHRASE"),
                HeaderValue::from_str(self.passphrase.as_str())?,
            );
        } else {
            return Err(Error::Msg("build_headers Failed to get timestamp".to_string()))
        }

        Ok(custon_headers)
    }

    async fn handler(&self, response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                let body = response.bytes().await?;
                let result = std::str::from_utf8(&body);
                Ok(result?.to_string())
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::ServiceUnavailable),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::BAD_REQUEST => {
                let error: OkexContentError = response.json().await?;
                Err(handle_content_error(error))
            }
            s => Err(Error::Msg(format!("Received response: {:?}", s))),
        }
    }
}

// todo! need to match the doc
fn handle_content_error(error: OkexContentError) -> Error {
    match (error.code, error.msg.as_ref()) {
        (51006, error_messages::INVALID_PRICE) => Error::InvalidPrice,
        (59506, msg) => Error::InvalidListenKey(msg.to_string()),
        _ => Error::OkexError { response: error },
    }
}