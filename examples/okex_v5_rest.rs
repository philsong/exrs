#[macro_use]
extern crate log;

use env_logger::Builder;
use exrs::okex_v5::account::*;
use exrs::okex_v5::api::*;
use exrs::okex_v5::config::Config;
use exrs::okex_v5::errors::Error;
use exrs::okex_v5::rest_model::PositionSide;

static API_KEY: &str = "";
static SECRET_KEY: &str = "";
static PASSPHRASE: &str = "";

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    take_order().await;
}

async fn take_order() {
    let account: Account = Okex::new(
        Some(API_KEY.to_string()),
        Some(SECRET_KEY.to_string()),
        Some(PASSPHRASE.to_string()),
    );

    match account
        .limit_buy("DOGE-USDT-SWAP", 100, 0.02, PositionSide::Long, "")
        .await
    {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }

    match account
        .limit_sell("DOGE-USDT-SWAP", 100, 0.05, PositionSide::Long, "")
        .await
    {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("{:?}", e),
    }
}
