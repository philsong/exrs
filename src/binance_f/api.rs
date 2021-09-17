use super::account::*;
use super::client::*;
use super::config::Config;
use super::general::*;
use super::market::*;
use super::userstream::*;

pub trait BinanceF: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    /// Create a binance API using environment variables for credentials
    /// BINANCE_API_KEY=<your api key>
    /// BINANCE_API_SECRET_KEY=<your secret key>
    fn new_with_env(config: &Config) -> Self {
        let api_key = std::env::var("BINANCE_API_KEY").ok();
        let secret = std::env::var("BINANCE_API_SECRET_KEY").ok();
        Self::new_with_config(api_key, secret, config)
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> Self;
}

impl BinanceF for FuturesGeneral {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
        }
    }
}

impl BinanceF for FuturesMarket {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl BinanceF for FuturesAccount {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> FuturesAccount {
        FuturesAccount {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}

impl BinanceF for FuturesUserStream {
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> FuturesUserStream {
        FuturesUserStream {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }
}
