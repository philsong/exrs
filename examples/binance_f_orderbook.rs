use csv::Writer;
use chrono::{DateTime, NaiveDateTime, Utc};
use env_logger::Builder;
use log::{info, warn};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use serde::{Deserialize, Serialize};

use exrs::binance_f::api::*;
use exrs::binance_f::market::*;
use exrs::binance_f::websockets::*;
use exrs::binance_f::rest_model::OrderBookPartial;
use exrs::binance_f::ws_model::DepthOrderBookEvent;
use exrs::binance_f::util::get_timestamp;

type Record<'a> = (&'a str, &'a u64, Vec<Decimal>, Vec<Decimal>, Vec<Decimal>, Vec<Decimal>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orderbook {
    pub symbol: String,
    pub timestamp: u64,
    pub final_update_id: u64,
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}

impl Orderbook {
    pub fn new(symbol: String) -> Orderbook {
        let now =  get_timestamp().unwrap();
        Orderbook {
            symbol,
            timestamp: now,
            final_update_id: 0,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn get_depth(&mut self, depth: usize) -> Option<Record> {
        // let asks: Vec<(Decimal, Decimal)> = self.asks.iter().take(depth).rev().collect();
        // let bids: Vec<(Decimal, Decimal)> = self.bids.iter().rev().take(depth).collect();
        let asks_price = self.asks.keys().cloned().take(depth).collect();
        let bids_price = self.bids.keys().cloned().rev().take(depth).collect();
        let asks_qty = self.asks.values().cloned().take(depth).collect();
        let bids_qty = self.bids.values().cloned().rev().take(depth).collect();

        info!("asks_price {:?}", asks_price);
        info!("bids_price {:?}", bids_price);
        info!("asks_qty {:?}", asks_qty);
        info!("bids_qty {:?}", bids_qty);
        
        Some((&self.symbol, &self.timestamp, asks_price, bids_price, asks_qty, bids_qty))
    }

    pub fn partial(&mut self, data: &OrderBookPartial) {
        self.bids.clear();
        self.asks.clear();
        self.final_update_id = data.last_update_id;
        self.timestamp = data.event_time;
        for bid in &data.bids {
            self.bids.insert(bid.price, bid.qty);
        }
        for ask in &data.asks {
            self.asks.insert(ask.price, ask.qty);
        }
    }

    pub fn update(&mut self, data: &DepthOrderBookEvent) {
        self.final_update_id = data.final_update_id;
        self.timestamp = data.event_time;
        for bid in &data.bids {
            if bid.qty == dec!(0){
                self.bids.remove(&bid.price);
            } else {
                self.bids.insert(bid.price, bid.qty);
            }
        }
        for ask in &data.asks {
            if ask.qty == dec!(0) {
                self.asks.remove(&ask.price);
            } else {
                self.asks.insert(ask.price, ask.qty);
            }
        }
    }

    pub fn verify(&mut self, pu_id: u64, check_bid_ask_overlapping: bool) -> bool {
        if check_bid_ask_overlapping {
            if self.bids.len() > 0 && self.asks.len() > 0 {
                if self.best_bid().unwrap().0 >= self.best_ask().unwrap().0 {
                    warn!("best bid {} >= best ask {}", self.best_bid().unwrap().0, self.best_ask().unwrap().0);
                    return false
                }
            }
        }

        self.final_update_id == pu_id
    }

    /// Returns the price of the best bid
    pub fn bid_price(&self) -> Option<Decimal> {
        self.bids.keys().rev().next().cloned()
    }

    /// Returns the price of the best ask
    pub fn ask_price(&mut self) -> Option<Decimal> {
        self.asks.keys().next().cloned()
    }

    /// Returns the midpoint between the best bid price and best ask price.
    /// Output is not rounded to the smallest price increment.
    pub fn mid_price(&mut self) -> Option<Decimal> {
        Some((self.bid_price()? + self.ask_price()?) / dec!(2))
    }

    /// Returns the price and quantity of the best bid
    /// (bid_price, bid_quantity)
    pub fn best_bid(&mut self) -> Option<(Decimal, Decimal)> {
        let (price, qty) = self.bids.iter().rev().next()?;

        Some((*price, *qty))
    }

    /// Returns the price and quantity of the best ask
    /// (ask_price, ask_quantity)
    pub fn best_ask(&mut self) -> Option<(Decimal, Decimal)> {
        let (price, qty) = self.asks.iter().next()?;

        Some((*price, *qty))
    }

    /// Returns the price and quantity of the best bid and best ask
    /// ((bid_price, bid_quantity), (ask_price, ask_quantity))
    pub fn best_bid_and_ask(&mut self) -> Option<((Decimal, Decimal), (Decimal, Decimal))> {
        Some((self.best_bid()?, self.best_ask()?))
    }
}

struct WebSocketHandler {
    wrt: Writer<File>,
}

impl WebSocketHandler {
    pub fn new(local_wrt: Writer<File>) -> Self { WebSocketHandler { wrt: local_wrt } }

    // serialize DayTickerEvent as CSV records
    pub fn write_to_file(&mut self, event: &Record) -> Result<(), Box<dyn Error>> {

        self.wrt.serialize(event)?;
        
        Ok(())
    }
}


#[actix_rt::main]
async fn main() {
    Builder::new().parse_default_env().init();

    let symbol = "ethusdt";
    let mut tmr_dt = Utc::today().and_hms(23,59,59);

    let file_name = format!("{}-{}-{:?}.csv", symbol, "depth20", Utc::today());
    let file_path = std::path::Path::new(&file_name);
    let local_wrt = csv::Writer::from_path(file_path).unwrap();
    let mut web_socket_handler = WebSocketHandler::new(local_wrt);

    let api_key_user = Some("YOUR_KEY".into());
    let market: FuturesMarket = BinanceF::new(api_key_user, None);

    let keep_running = AtomicBool::new(true);

    let depth = format!("{}@depth@100ms", symbol);
    let (tx, mut rx) = tokio::sync::mpsc::channel(1000);
    let mut web_socket: FuturesWebSockets<DepthOrderBookEvent> = FuturesWebSockets::new(tx);
    let mut orderbook = Orderbook::new("ethusdt".to_string());

    web_socket.connect(&depth).await.unwrap();

    actix_rt::spawn(async move {
        let partial_init: OrderBookPartial = market.get_custom_depth(symbol, 1000).await.unwrap();
        orderbook.partial(&partial_init);

        loop {
            let msg = rx.recv().await.unwrap();

            if msg.final_update_id < partial_init.last_update_id {
                continue
            } else if msg.first_update_id <= partial_init.last_update_id && msg.final_update_id >= partial_init.last_update_id {
                orderbook.update(&msg)
            } else if orderbook.verify(msg.previous_final_update_id, false) {
                info!("verfiy passed");
                orderbook.update(&msg)
            } else {
                warn!("verfiy failed");
                let partial_init: OrderBookPartial = market.get_custom_depth(symbol, 1000).await.unwrap();
                orderbook.partial(&partial_init);
            }

            let event = orderbook.get_depth(20).unwrap();

            if DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp((msg.event_time / 1000) as i64, 0), Utc) > tmr_dt {
                tmr_dt = Utc::today().and_hms(23,59,59);
                let file_name = format!("{}-{}-{:?}.csv", symbol, "depth20", Utc::today());
                let file_path = std::path::Path::new(&file_name);
                let local_wrt = csv::Writer::from_path(file_path).unwrap();
                web_socket_handler = WebSocketHandler::new(local_wrt);
            }
            
            if let Err(error) = web_socket_handler.write_to_file(&event) {
                warn!("{}", error);
            };
        }
    });

    while let Err(e) = web_socket.event_loop(&keep_running).await {
        warn!("web_socket event_loop Error: {}, starting reconnect...", e);

        while let Err(e) = web_socket.connect(&depth).await {
            warn!("web_socket connect Error: {}, try again...", e);
        }
    }
}