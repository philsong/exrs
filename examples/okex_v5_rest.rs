#[macro_use]
extern crate log;

use env_logger::Builder;
use exrs::okex_v5::account::*;
use exrs::okex_v5::api::*;
use exrs::okex_v5::config::Config;
use exrs::okex_v5::errors::Error;

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    take_order().await;
}

async fn take_order() {
    let api_key = "1f2ee2ab-9a1f-4946-b496-a444fdf11c4a".to_string();
    let secret_key = "8187D5156E0979C385B11263C2E247A3".to_string();
    let passphrase = "zpxx2021".to_string();
    let account: Account = Okex::new(Some(api_key), Some(secret_key), Some(passphrase));
    match account.limit_buy("DOGE-USDT-SWAP", 100, 0.02).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }
}
