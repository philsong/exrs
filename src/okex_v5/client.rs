use std::time::Duration;

use chrono::prelude::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Method;
use reqwest::Response;
use reqwest::StatusCode;
use hmac_sha256::HMAC;
use serde::de;
use serde::de::DeserializeOwned;
use serde_json::from_str;

use super::errors::error_messages;
use super::errors::*;
use super::rest_model::PairQuery;
use super::util::build_request_p;

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
    pub fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        host: String,
    ) -> Self {
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
        request: &str,
    ) -> Result<T> {
        let r = self.get_signed(endpoint, request).await?;
        let t = from_str(r.as_str())?;
        Ok(t)
    }

    pub async fn get_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: Option<P>,
    ) -> Result<T> {
        let req = if let Some(p) = payload {
            build_request_p(p)?
        } else {
            let option: Option<PairQuery> = None;
            build_request_p(option)?
        };
        let string = self.get_signed(endpoint, &req).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    pub async fn post_signed(&self, endpoint: &str, request_body: String) -> Result<String> {
        let url = format!("{}{}", self.host, endpoint);

        println!("post_signed - request_body: {}", request_body);

        let response = self
            .inner
            .clone()
            .post(url.as_str())
            .headers(self.build_signed_headers(true, Method::POST, endpoint, &request_body)?)
            .body(request_body)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn post_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: P,
    ) -> Result<T> {
        let request_body = serde_json::to_string(&payload)?;
        let string = self.post_signed(endpoint, request_body).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    // okex v5 didn't have delete method

    pub async fn get(&self, endpoint: &str, request: &str) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str()).await?;

        self.handler(response).await
    }

    pub async fn get_p<T: DeserializeOwned>(&self, endpoint: &str, request: &str) -> Result<T> {
        let r = self.get(endpoint, request).await?;
        let t = from_str(r.as_str())?;
        Ok(t)
    }

    pub async fn get_d<T: DeserializeOwned, S: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: Option<S>,
    ) -> Result<T> {
        let req = if let Some(p) = payload {
            build_request_p(p)?
        } else {
            String::new()
        };
        self.get_p(endpoint, req.as_str()).await
    }

    pub async fn post(&self, endpoint: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, endpoint);

        let response = self
            .inner
            .clone()
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        Ok(custom_headers)
    }

    pub fn build_signed_headers(
        &self,
        content_type: bool,
        method: Method,
        endpoint: &str,
        request_body: &str,
    ) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let pre_hash = format!(
            "{}{}{}{}",
            timestamp,
            method.as_str(),
            endpoint,
            request_body
        );
        println!("pre_hash: {}", pre_hash);

        let signature = base64::encode(HMAC::mac(pre_hash.as_bytes(), self.secret_key.as_bytes()));

        custom_headers.insert(
            HeaderName::from_static("ok-access-key"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-sign"),
            HeaderValue::from_str(&signature.as_str())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-timestamp"),
            HeaderValue::from_str(&timestamp.to_string())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-passphrase"),
            HeaderValue::from_str(self.passphrase.as_str())?,
        );

        Ok(custom_headers)
    }

    pub fn build_signed_headers_p<S>(
        &self,
        content_type: bool,
        method: Method,
        endpoint: &str,
        payload: S,
    ) -> Result<HeaderMap>
    where
        S: serde::Serialize,
    {
        let query_string = qs::to_string(&payload)?;
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("okex-rs"));
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        println!("timestamp {}", timestamp);

        let pre_hash = format!(
            "{}{}{}?{}",
            timestamp,
            method.as_str(),
            endpoint,
            query_string
        );

        let signature = base64::encode(HMAC::mac(pre_hash.as_bytes(), self.secret_key.as_bytes()));

        custom_headers.insert(
            HeaderName::from_static("ok-access-key"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-sign"),
            HeaderValue::from_str(&signature.as_str())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-timestamp"),
            HeaderValue::from_str(&timestamp.to_string())?,
        );
        custom_headers.insert(
            HeaderName::from_static("ok-access-passphrase"),
            HeaderValue::from_str(self.passphrase.as_str())?,
        );

        Ok(custom_headers)
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
