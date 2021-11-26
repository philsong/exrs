#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use chrono::{DateTime, NaiveDateTime, Utc};
use csv::Writer;
use env_logger::Builder;
use log::{info, warn};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::sync::atomic::AtomicBool;

use exrs::binance_f::api::*;
use exrs::binance_f::market::*;
use exrs::binance_f::rest_model::OrderBookPartial;
use exrs::binance_f::util::get_timestamp;
use exrs::binance_f::websockets::*;
use exrs::binance_f::ws_model::{AggrTradesEvent, DepthOrderBookEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Config {
    pub server_id: String,
    pub log: Log,
    pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub console: bool,
    pub level: String,
    pub path: String,
    pub name: String,
    pub clear: bool,
    pub backup_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub symbol: Vec<String>,
    pub channels: Vec<String>,
    pub silent: bool,
    pub platform: String,
    pub influx_database: bool,
    pub file_format: String,
    pub file_url: String,
}

type Record<'a> = (
    &'a str,
    &'a u64,
    Vec<Decimal>,
    Vec<Decimal>,
    Vec<Decimal>,
    Vec<Decimal>,
);

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Record {
//     pub symbol: String,
//     pub timestamp: u64,
//     pub asks_price: Vec<Decimal>,
//     pub bids_price: Vec<Decimal>,
//     pub asks_qty: Vec<Decimal>,
//     pub bids_qty: Vec<Decimal>,
// }

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
        let now = get_timestamp().unwrap();
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

        Some((
            &self.symbol,
            &self.timestamp,
            asks_price,
            bids_price,
            asks_qty,
            bids_qty,
        ))
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
            if bid.qty == dec!(0) {
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
                    warn!(
                        "best bid {} >= best ask {}",
                        self.best_bid().unwrap().0,
                        self.best_ask().unwrap().0
                    );
                    return false;
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
    pub fn new(local_wrt: Writer<File>) -> Self {
        WebSocketHandler { wrt: local_wrt }
    }

    /// "ap6","ap7","ap8","ap9","ap10","ap11","ap12","ap13","ap14","ap15","ap16","ap17","ap18","ap19","ap20",
    /// "bp6","bp7","bp8","bp9","bp10","bp11","bp12","bp13","bp14","bp15","bp16","bp17","bp18","bp19","bp20",
    /// "az6","az7","az8","az9","az10","az11","az12","az13","az14","az15","az16","az17","az18","az19","az20",
    /// "bz6","bz7","bz8","bz9","bz10","bz11","bz12","bz13","bz14","bz15","bz16","bz17","bz18","bz19","bz20",
    pub fn write_depth_header(&mut self) -> Result<(), Box<dyn Error>> {
        self.wrt.write_record(&[
            "symbol",
            "timestamp",
            "ap1",
            "ap2",
            "ap3",
            "ap4",
            "ap5",
            "bp1",
            "bp2",
            "bp3",
            "bp4",
            "bp5",
            "az1",
            "az2",
            "az3",
            "az4",
            "az5",
            "bz1",
            "bz2",
            "bz3",
            "bz4",
            "bz5",
        ])?;

        Ok(())
    }

    // serialize Depth as CSV records
    pub fn write_depth_to_file(&mut self, event: &Record) -> Result<(), Box<dyn Error>> {
        self.wrt.serialize(event)?;

        Ok(())
    }

    // serialize Depth as CSV records
    pub fn write_partial_depth_to_file(
        &mut self,
        event: &DepthOrderBookEvent,
    ) -> Result<(), Box<dyn Error>> {
        let asks_price: Vec<Decimal> = event.asks.iter().map(|x| x.price).collect();
        let bids_price: Vec<Decimal> = event.bids.iter().map(|x| x.price).collect();
        let asks_qty: Vec<Decimal> = event.asks.iter().map(|x| x.qty).collect();
        let bids_qty: Vec<Decimal> = event.bids.iter().map(|x| x.qty).collect();

        info!("asks_price {:?}", asks_price);
        info!("bids_price {:?}", bids_price);
        info!("asks_qty {:?}", asks_qty);
        info!("bids_qty {:?}", bids_qty);

        let data = Some((
            &event.symbol,
            &event.event_time,
            asks_price,
            bids_price,
            asks_qty,
            bids_qty,
        ));

        self.wrt.serialize(data)?;

        Ok(())
    }

    // serialize Trades as CSV records
    pub fn write_trades_to_file(&mut self, event: &AggrTradesEvent) -> Result<(), Box<dyn Error>> {
        self.wrt.serialize(event)?;

        Ok(())
    }
}

async fn run_partial_depth(file_url: String, symbol: String) {
    let mut tmr_dt = Utc::today().and_hms(23, 59, 59);

    let file_name = format!("{}-{}-{:?}.csv", symbol, "depth5", Utc::today());
    let file_path = std::path::Path::new(&file_url).join(file_name);
    let local_wrt = csv::Writer::from_path(file_path).unwrap();
    let mut web_socket_handler = WebSocketHandler::new(local_wrt);
    web_socket_handler.write_depth_header().unwrap();

    let keep_running = AtomicBool::new(true);
    let depth = format!("{}@depth5@0ms", symbol);
    let (tx, mut rx) = tokio::sync::mpsc::channel(8192);
    let mut web_socket: FuturesWebSockets<DepthOrderBookEvent> = FuturesWebSockets::new(tx);

    web_socket.connect(&depth).await.unwrap();

    actix_rt::spawn(async move {
        loop {
            let msg = rx.recv().await.unwrap();

            if DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp((msg.event_time / 1000) as i64, 0),
                Utc,
            ) > tmr_dt
            {
                tmr_dt = Utc::today().and_hms(23, 59, 59);
                let file_name = format!("{}-{}-{:?}.csv", symbol, "depth5", Utc::today());
                let file_path = std::path::Path::new(&file_url).join(file_name);
                let local_wrt = csv::Writer::from_path(file_path).unwrap();
                web_socket_handler = WebSocketHandler::new(local_wrt);
                web_socket_handler.write_depth_header().unwrap();
            }

            if let Err(error) = web_socket_handler.write_partial_depth_to_file(&msg) {
                warn!("{}", error);
            };
        }
    });

    while let Err(e) = web_socket.event_loop(&keep_running).await {
        warn!(
            "partial_depth web_socket event_loop Error: {}, starting reconnect...",
            e
        );

        while let Err(e) = web_socket.connect(&depth).await {
            warn!(
                "partial_depth web_socket connect Error: {}, try again...",
                e
            );
        }
    }
}

async fn run_depth(file_url: String, symbol: String) {
    let mut tmr_dt = Utc::today().and_hms(23, 59, 59);

    let file_name = format!("{}-{}-{:?}.csv", symbol, "depth5", Utc::today());
    let file_path = std::path::Path::new(&file_url).join(file_name);
    let local_wrt = csv::Writer::from_path(file_path).unwrap();
    let mut web_socket_handler = WebSocketHandler::new(local_wrt);
    web_socket_handler.write_depth_header().unwrap();

    let api_key_user = Some("YOUR_KEY".into());
    let market: FuturesMarket = BinanceF::new(api_key_user, None);

    let keep_running = AtomicBool::new(true);
    let depth = format!("{}@depth@0ms", symbol);
    let (tx, mut rx) = tokio::sync::mpsc::channel(8192);
    let mut web_socket: FuturesWebSockets<DepthOrderBookEvent> = FuturesWebSockets::new(tx);
    let mut orderbook = Orderbook::new(symbol.clone());

    web_socket.connect(&depth).await.unwrap();

    actix_rt::spawn(async move {
        let partial_init: OrderBookPartial =
            market.get_custom_depth(symbol.clone(), 1000).await.unwrap();
        orderbook.partial(&partial_init);

        loop {
            let msg = rx.recv().await.unwrap();

            if msg.final_update_id < partial_init.last_update_id {
                continue;
            } else if msg.first_update_id <= partial_init.last_update_id
                && msg.final_update_id >= partial_init.last_update_id
            {
                orderbook.update(&msg)
            } else if orderbook.verify(msg.previous_final_update_id, false) {
                info!("verfiy passed");
                orderbook.update(&msg)
            } else {
                warn!("verfiy failed");
                let partial_init: OrderBookPartial =
                    market.get_custom_depth(symbol.clone(), 1000).await.unwrap();
                orderbook.partial(&partial_init);
            }

            let event = orderbook.get_depth(5).unwrap();

            if DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp((msg.event_time / 1000) as i64, 0),
                Utc,
            ) > tmr_dt
            {
                tmr_dt = Utc::today().and_hms(23, 59, 59);
                let file_name = format!("{}-{}-{:?}.csv", symbol, "depth5", Utc::today());
                let file_path = std::path::Path::new(&file_url).join(file_name);
                let local_wrt = csv::Writer::from_path(file_path).unwrap();
                web_socket_handler = WebSocketHandler::new(local_wrt);
                web_socket_handler.write_depth_header().unwrap();
            }

            if let Err(error) = web_socket_handler.write_depth_to_file(&event) {
                warn!("{}", error);
            };
        }
    });

    while let Err(e) = web_socket.event_loop(&keep_running).await {
        warn!(
            "depth web_socket event_loop Error: {}, starting reconnect...",
            e
        );

        while let Err(e) = web_socket.connect(&depth).await {
            warn!("depth web_socket connect Error: {}, try again...", e);
        }
    }
}

async fn run_trades(file_url: String, symbol: String) {
    let mut tmr_dt = Utc::today().and_hms(23, 59, 59);

    let file_name = format!("{}-{}-{:?}.csv", symbol, "trades", Utc::today());
    let file_path = std::path::Path::new(&file_url).join(file_name);
    let local_wrt = csv::Writer::from_path(file_path).unwrap();
    let mut web_socket_handler = WebSocketHandler::new(local_wrt);

    let keep_running = AtomicBool::new(true);
    let agg_trade = format!("{}@aggTrade", symbol);
    let (tx, mut rx) = tokio::sync::mpsc::channel(8192);
    let mut web_socket: FuturesWebSockets<AggrTradesEvent> = FuturesWebSockets::new(tx);

    web_socket.connect(&agg_trade).await.unwrap();

    actix_rt::spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();

            if DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp((event.event_time / 1000) as i64, 0),
                Utc,
            ) > tmr_dt
            {
                tmr_dt = Utc::today().and_hms(23, 59, 59);
                let file_name = format!("{}-{}-{:?}.csv", symbol, "trades", Utc::today());
                let file_path = std::path::Path::new(&file_url).join(file_name);
                let local_wrt = csv::Writer::from_path(file_path).unwrap();
                web_socket_handler = WebSocketHandler::new(local_wrt);
            }

            if let Err(error) = web_socket_handler.write_trades_to_file(&event) {
                warn!("{}", error);
            };
        }
    });

    while let Err(e) = web_socket.event_loop(&keep_running).await {
        warn!(
            "trades web_socket event_loop Error: {}, starting reconnect...",
            e
        );

        while let Err(e) = web_socket.connect(&agg_trade).await {
            warn!("trades web_socket connect Error: {}, try again...", e);
        }
    }
}

#[actix_rt::main]
async fn main() {
    Builder::new().parse_default_env().init();

    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("file should open read only");
    let c: Config = serde_json::from_reader(file).expect("file shoud be proper json");

    let mut tasks = Vec::new();
    for symbol in c.data.symbol.iter() {
        for ch in c.data.channels.iter() {
            match ch.as_str() {
                "depth5@0ms" => {
                    let symbol = symbol.clone();
                    let file_url = c.data.file_url.clone();
                    let task =
                        actix_rt::spawn(async move { run_partial_depth(file_url, symbol).await });
                    tasks.push(task);
                }
                "depth@0ms" => {
                    let symbol = symbol.clone();
                    let file_url = c.data.file_url.clone();
                    let task = actix_rt::spawn(async move { run_depth(file_url, symbol).await });
                    tasks.push(task);
                }
                "aggTrade" => {
                    let symbol = symbol.clone();
                    let file_url = c.data.file_url.clone();
                    let task = actix_rt::spawn(async move { run_trades(file_url, symbol).await });
                    tasks.push(task);
                }
                _ => {
                    warn!("Error: channel type not support!")
                }
            }
        }
    }

    for task in tasks {
        task.await.unwrap();
    }
}
