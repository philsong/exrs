use crate::huobi::account::*;
use crate::huobi::client::*;
use crate::huobi::config::Config;
use crate::huobi::margin::Margin;
use crate::huobi::market::*;
use crate::huobi::reference::*;
use crate::huobi::savings::Savings;
use crate::huobi::userstream::*;

pub trait Huobi: Sized {
    fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self::new_with_config(api_key, api_secret, &Config::default())
    }

    fn new_with_env(config: &Config) -> Self {
        let api_key = std::env::var("HUBIO_API_KEY").ok();
        let api_secret = std::env::var("HUBIO_API_SECRET").ok();
        Self::new_with_config(api_key, api_secret, config)
    }

    fn new_with_config(
        api_key: Option<String>,
        api_secret: Option<String>,
        config: &Config,
    ) -> Self;
}

impl Huobi for Reference {
    fn new_with_config(
        api_key: Option<String>,
        api_secret: Option<String>,
        config: &Config,
    ) -> Self {
        Reference {
            client: Client::new(api_key, api_secret, config.rest_api_endpoint.clone()),
        }
    }
}
