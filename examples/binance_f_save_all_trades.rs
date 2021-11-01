use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::sync::atomic::AtomicBool;

use exrs::binance_f::websockets::*;
use exrs::binance_f::ws_model::FuturesWebsocketEvent;

#[actix_rt::main]
async fn main() {
    save_all_trades_websockets().await;
}

async fn save_all_trades_websockets() {
    struct WebSocketHandler {
        wrt: Writer<File>,
    }

    impl WebSocketHandler {
        pub fn new(local_wrt: Writer<File>) -> Self {
            WebSocketHandler { wrt: local_wrt }
        }

        // serialize DayTickerEvent as CSV records
        pub fn write_to_file(
            &mut self,
            events: Vec<FuturesWebsocketEvent>,
        ) -> Result<(), Box<dyn Error>> {
            for event in events {
                self.wrt.serialize(event)?;
            }
            Ok(())
        }
    }

    let keep_running = AtomicBool::new(true);
    let file_path = std::path::Path::new("test.csv");
    let local_wrt = csv::Writer::from_path(file_path).unwrap();

    let mut web_socket_handler = WebSocketHandler::new(local_wrt);
    let agg_trade: String = "!ticker@arr".to_string();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut web_socket: FuturesWebSockets<Vec<FuturesWebsocketEvent>> = FuturesWebSockets::new(tx);

    actix_rt::spawn(async move {
        loop {
            let events = rx.recv().await.unwrap();

            // You can break the event_loop if some condition is met be setting keep_running to false
            // keep_running.store(false, Ordering::Relaxed);
            if let Err(error) = web_socket_handler.write_to_file(events) {
                println!("{}", error);
            };
        }
    });

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }
}
