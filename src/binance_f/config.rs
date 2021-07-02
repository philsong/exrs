#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,
    pub futures_testnet_rest_api_endpoint: String,
    pub futures_testnet_ws_endpoint: String,

    pub recv_window: u64,
}

impl Config {
    pub fn default() -> Config {
        Config {
            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com".into(),
            futures_testnet_rest_api_endpoint: "https://testnet.binancefuture.com".into(),
            futures_testnet_ws_endpoint: "wss://stream.binancefuture.com".into(),
            recv_window: 5000,
        }
    }

    pub fn testnet() -> Config {
        Config::default()
            .set_futures_rest_api_endpoint("https://fapi.binance.com")
            .set_futures_ws_endpoint("wss://fstream.binance.com")
            .set_futures_testnet_rest_api_endpoint("https://testnet.binancefuture.com")
            .set_futures_testnet_ws_endpoint("wss://stream.binancefuture.com")
    }

    pub fn set_futures_rest_api_endpoint<T: Into<String>>(
        mut self,
        futures_rest_api_endpoint: T,
    ) -> Self {
        self.futures_rest_api_endpoint = futures_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_ws_endpoint<T: Into<String>>(mut self, futures_ws_endpoint: T) -> Self {
        self.futures_ws_endpoint = futures_ws_endpoint.into();
        self
    }

    pub fn set_futures_testnet_rest_api_endpoint<T: Into<String>>(
        mut self,
        futures_testnet_rest_api_endpoint: T,
    ) -> Self {
        self.futures_testnet_rest_api_endpoint = futures_testnet_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_testnet_ws_endpoint<T: Into<String>>(
        mut self,
        futures_testnet_ws_endpoint: T,
    ) -> Self {
        self.futures_testnet_ws_endpoint = futures_testnet_ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}
