use super::account::*;
use super::client::*;
use super::config::Config;
use super::general::*;
use super::margin::Margin;
use super::market::*;
use super::userstream::*;

pub trait Okex: Sized {
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

    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

impl Okex for General {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Okex for Account {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Okex for Market {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Okex for UserStream {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Okex for Margin {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Margin {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}
