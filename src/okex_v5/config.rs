#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,
}

impl Config {
    /// Configure binance with default production endpoints
    /// # Examples            .set_rest_api_endpoint("https://testnet.binance.vision")

    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// ```
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://www.okex.com".into(),
            ws_endpoint: "wss://ws.okex.com:8443/ws/v5".into(),
        }
    }

    /// Configure binance with all testnet endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::testnet();
    /// ```
    pub fn testnet() -> Config {
        Config::default()
            .set_rest_api_endpoint("https://www.okex.com")
            .set_ws_public("wss://wspap.okex.com:8443/ws/v5/public?brokerId=9999")
            .set_ws_private("wss://wspap.okex.com:8443/ws/v5/private?brokerId=9999")
    }

    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_public<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    pub fn set_ws_private<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }
}
