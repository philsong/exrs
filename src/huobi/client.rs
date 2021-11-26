use base64;
use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use hmac_sha256::HMAC;
use serde::de;
use serde_json::from_str;
use std::time::Duration;

use crate::huobi::errors::error_messages;
use crate::huobi::errors::*;
use crate::huobi::util::{build_request_p, build_signed_request_p};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    api_secret: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    /// Returns a client based on the specified host and credentials
    /// Credentials do not need to be specified when using public endpoints
    /// Host is mandatory
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

    pub async fn get_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let response = self
            .inner
            .clone()
            .get(url.as_str())
            .headers(self.build_headers(true)?)
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
            String::new()
        };
        let string = self.get_signed(endpoint, &req).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

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

    pub async fn delete(&self, endpoint: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, endpoint);

        let response = self
            .inner
            .clone()
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signature = base64::encode(HMAC::mac(request.as_bytes(), self.api_secret.as_bytes()));

        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", self.host, endpoint, request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        // custom_headers.insert(USER_AGENT, HeaderValue::from_static("huobi-rs"));
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

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
                let error: HuobiContentError = response.json().await?;
                Err(handle_content_error(error))
            }
            s => Err(Error::Msg(format!("Received response: {:?}", s))),
        }
    }
}

fn handle_content_error(error: HuobiContentError) -> crate::huobi::errors::Error {
    match (error.code, error.msg.as_ref()) {
        (-1, error_messages::INVALID_PRICE) => Error::InvalidPrice,
        _ => Error::HuobiError { response: error },
    }
}
