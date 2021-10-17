#[macro_use]
extern crate log;

use env_logger::Builder;
use exrs::okex_v5::account::*;
use exrs::okex_v5::api::*;
use exrs::okex_v5::config::Config;
use exrs::okex_v5::errors::Error;

static API_KEY: &str = "1f2ee2ab-9a1f-4946-b496-a444fdf11c4a";
static SECRET_KEY: &str = "8187D5156E0979C385B11263C2E247A3";
static PASSPHRASE: &str = "zpxx2021";

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    take_order().await;
}

async fn take_order() {
    let account: Account = Okex::new(Some(API_KEY.to_string()), Some(SECRET_KEY.to_string()), Some(PASSPHRASE.to_string()));

    match account.limit_buy("DOGE-USDT-SWAP", 100, 0.02).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }

    match account.limit_sell("DOGE-USDT-SWAP", 100, 0.05).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }
}
