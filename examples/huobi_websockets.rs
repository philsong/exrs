use env_logger::Builder;
use exrs::huobi::api::*;
use exrs::huobi::userstream::*;
use exrs::huobi::websockets::*;
use exrs::huobi::ws_model::BBOEvent;
use exrs::huobi::ws_model::WebsocketEvent;
use std::sync::atomic::{AtomicBool, Ordering};

#[actix_rt::main]
async fn main() {
    Builder::new().parse_default_env().init();
    //user_stream().await;
    //user_stream_websocket().await;
    //market_websocket().await;
    bbo_websocket().await;
    //all_trades_websocket().await;
}

async fn bbo_websocket() {
    let keep_running = AtomicBool::new(true);
    let bbo_req = r#"{"sub": "market.btcusdt.bbo","id": "id1"}"#;
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut web_socket: WebSockets<WebsocketEvent> = WebSockets::new(tx);

    actix_rt::spawn(async move {
        loop {
            let msg = rx.recv().await.unwrap();
            println!("{:?}", msg);
            actix_rt::task::yield_now().await;
        }
    });

    web_socket.connect("ws").await.unwrap();
    web_socket.subscribe_request(bbo_req).await.unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {}", e);
    }

    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}
