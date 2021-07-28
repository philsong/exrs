use exrs::binance_f::api::*;
use exrs::binance_f::userstream::*;
use exrs::binance_f::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    //user_stream();
    //user_stream_websocket();
    //market_websocket();
    //kline_websocket();
    //all_trades_websocket();
    //last_price_for_one_symbol();
    multiple_streams();
}

fn market_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = String::from("ethbtc@aggTrade");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::AggrTrades(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
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

fn all_trades_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = String::from("!ticker@arr");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTickerAll(ticker_events) = event {
            for tick_event in ticker_events {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    tick_event.symbol, tick_event.current_close, tick_event.current_close_qty
                );
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn kline_websocket() {
    let keep_running = AtomicBool::new(true);
    let kline: String = String::from("ethbtc@kline_1m");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::Kline(kline_event) = event {
            println!(
                "Symbol: {}, high: {}, low: {}",
                kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
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

fn last_price_for_one_symbol() {
    let keep_running = AtomicBool::new(true);
    let agg_trade: String = String::from("btcusdt@ticker");
    let mut btcusdt: f32 = "0".parse().unwrap();

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            println!("dayticker - {:?}", ticker_event);
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn multiple_streams() {
    let symbols: Vec<_> = vec!["ethusdt", "btcusdt"]
        .into_iter()
        .map(String::from)
        .collect();
    let mut endpoints: Vec<String> = Vec::new();

    for symbol in symbols.iter() {
        endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()));
    }

    let keep_running = AtomicBool::new(true);
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
            println!("{:?}", depth_order_book);
        } else {
            println!("error");
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
}
