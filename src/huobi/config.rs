#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,
    pub mbp_endpoint: String,
    pub account_enddpoint: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.huobi.pro".into(),
            ws_endpoint: "wss://api.huobi.pro/ws".into(),
            mbp_endpoint: "wss://api.huobi.pro/feed".into(),
            account_enddpoint: "wss://api.huobi.pro/ws/v2".into(),
        }
    }
}
