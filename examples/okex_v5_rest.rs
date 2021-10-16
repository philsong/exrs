#[macro_use]
extern crate log;

use exrs::okex_v5::account::*;
use exrs::okex_v5::api::*;
use exrs::okex_v5::config::Config;
use exrs::okex_v5::errors::Error as BinanceLibError;
use env_logger::Builder;

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    take_order().await;
}

async fn take_order() {
    let api_key = "".to_string();
    let secret_key = "".to_string();
    let passphrase = "".to_string();
    let account: Account = Okex::new(Some(api_key), Some(secret_key), Some(passphrase));
    match account.limit_buy("DOGE-USDT-PERP", 100, 0.02).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }
}