use exrs::binance_f::api::*;
use exrs::binance_f::userstream::*;
use exrs::binance_f::websockets::*;
use exrs::binance_f::websockets::FuturesWebsocketEvent;
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    //user_stream();
    //user_stream_websocket();
    //market_websocket();
    bookticker_websocket();
    //all_trades_websocket();
}

#[allow(dead_code)]
async fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: FuturesUserStream = BinanceF::new(api_key_user.clone(), None);

    if let Ok(answer) = user_stream.start().await {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(&listen_key).await {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match user_stream.close(&listen_key).await {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

#[allow(dead_code)]
async fn user_stream_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: FuturesUserStream = BinanceF::new(api_key_user, None);

    if let Ok(answer) = user_stream.start().await {
        let listen_key = answer.listen_key;

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
            if let FuturesWebsocketEvent::OrderTrade(trade) = event {
                println!(
                    "Symbol: {}, Side: {:?}, Price: {}, Execution Type: {:?}",
                    trade.symbol, trade.side, trade.price, trade.execution_type
                );
            };

            Ok(())
        });

        web_socket.connect(&listen_key).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            println!("Error: {}", e);
        }
        user_stream.close(&listen_key).await.unwrap();
        web_socket.disconnect().unwrap();
        println!("Userstrem closed and disconnected");
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

#[allow(dead_code)]
fn market_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = format!("{}@aggTrade", "ethbtc");
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
        match event {
            FuturesWebsocketEvent::Trade(trade) => {
                println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
            }
            FuturesWebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            }
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
fn bookticker_websocket() {
    debug!("hi");
    let keep_running = AtomicBool::new(true);
    let kline: String = "ethusdt@bookTicker".to_string();
    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
        if let FuturesWebsocketEvent::BookTicker(kline_event) = event {
            println!(
                "{:?}", kline_event
            );
        }

        Ok(())
    });

    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}