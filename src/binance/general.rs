use super::client::*;
use super::errors::*;
use super::rest_model::*;

use serde_json::from_str;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    /// Test connectivity
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let general: General = Binance::new_with_env(&Config::default());
    /// let pong = tokio_test::block_on(general.ping());
    /// assert!(pong.is_ok(), "{:?}", pong);
    /// assert_eq!(pong.unwrap(), "pong");
    /// ```
    pub async fn ping(&self) -> Result<String> {
        self.client.get("/api/v3/ping", "").await?;

        Ok("pong".into())
    }

    /// Check server time
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let general: General = Binance::new_with_env(&Config::default());
    /// let server_time = tokio_test::block_on(general.get_server_time());
    /// assert!(server_time.is_ok(), "{:?}", server_time);
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        let data: String = self.client.get("/api/v3/time", "").await?;

        let server_time: ServerTime = from_str(data.as_str())?;

        Ok(server_time)
    }

    /// Obtain exchange information (rate limits, symbol metadata etc)
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let general: General = Binance::new_with_env(&Config::default());
    /// let excyahge_info = tokio_test::block_on(general.exchange_info());
    /// assert!(excyahge_info.is_ok(), "{:?}", excyahge_info);
    /// ```
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        let data: String = self.client.get("/api/v3/exchangeInfo", "").await?;

        let info: ExchangeInformation = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let symbol_string = symbol.into();
        let upper_symbol = symbol_string.to_uppercase();

        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                Err(Error::UnknownSymbol(symbol_string.clone()))
            }
            Err(e) => Err(e),
        }
    }
}
