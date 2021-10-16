use env_logger::Builder;
use exrs::binance_f::api::*;
use exrs::binance_f::userstream::*;
use exrs::binance_f::websockets::*;
use exrs::binance_f::ws_model::{BookTickerEvent, FuturesWebsocketEvent};
use std::sync::atomic::{AtomicBool, Ordering};

#[actix_rt::main]
async fn main() {
    Builder::new().parse_default_env().init();
    //user_stream().await;
    //user_stream_websocket().await;
    //market_websocket().await;
    bookticker_websocket().await;
    //all_trades_websocket().await;
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
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let tx = tx.clone();
    if let Ok(answer) = user_stream.start().await {
        let listen_key = answer.listen_key;

        let mut web_socket: FuturesWebSockets<FuturesWebsocketEvent> = FuturesWebSockets::new(tx);

        actix_rt::spawn(async move {
            loop {
                let msg = rx.recv().await.unwrap();
                println!("msg - {:?}", msg);
                actix_rt::task::yield_now().await;
            }
        });

        web_socket.connect(&listen_key).await.unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {}", e);
        }
        user_stream.close(&listen_key).await.unwrap();
        web_socket.disconnect().await.unwrap();
        println!("Userstrem closed and disconnected");
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

#[allow(dead_code)]
async fn market_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = format!("{}@aggTrade", "ethusdt");
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut web_socket: FuturesWebSockets<FuturesWebsocketEvent> = FuturesWebSockets::new(tx);

    actix_rt::spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            match event {
                FuturesWebsocketEvent::AggrTrades(trade) => {
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        trade.symbol, trade.price, trade.qty
                    );
                }
                FuturesWebsocketEvent::DepthOrderBook(depth_order_book) => {
                    println!(
                        "Symbol: {}, Bids: {:?}, Ask: {:?}",
                        depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                    );
                }
                _ => (),
            };
            actix_rt::task::yield_now().await;
        }
    });

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn bookticker_websocket() {
    let keep_running = AtomicBool::new(true);
    let bookticker: String = "ethusdt@bookTicker".to_string();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut web_socket: FuturesWebSockets<BookTickerEvent> = FuturesWebSockets::new(tx);

    actix_rt::spawn(async move {
        let mut count: usize = 0;
        let mut len: usize = 0;
        let mut pre = 0;

        loop {
            let msg = rx.recv().await.unwrap();
            count += 1;

            if msg.transaction_time / 1000 > pre {
                pre = msg.transaction_time / 1000;
                len += 1;
                println!("mean: {}", count as f64 / len as f64)
            }

            actix_rt::task::yield_now().await;
        }
    });

    web_socket.connect(&bookticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn all_trades_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = all_ticker_stream();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut web_socket: FuturesWebSockets<Vec<FuturesWebsocketEvent>> = FuturesWebSockets::new(tx);

    actix_rt::spawn(async move {
        loop {
            let events = rx.recv().await.unwrap();

            for tick_events in events {
                if let FuturesWebsocketEvent::DayTicker(tick_event) = tick_events {
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        tick_event.symbol, tick_event.current_close, tick_event.current_close_qty
                    );
                }
            }
        }
    });

    web_socket.connect(agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}
