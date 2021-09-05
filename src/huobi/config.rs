#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_feed_except_mbp: String,
    pub ws_feed_only_mbp: String,
    pub ws_feed_account: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.huobi.pro".into(),
            ws_feed_except_mbp: "wss://api.huobi.pro/ws".into(),
            ws_feed_only_mbp: "wss://api.huobi.pro/feed".into(),
            ws_feed_account: "wss://api.huobi.pro/ws/v2".into(),
        }
    }
}
