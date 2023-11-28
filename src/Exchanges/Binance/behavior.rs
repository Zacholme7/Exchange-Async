use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::exchange::ExchangeBehavior;
use crate::load_config::ExchangeInformation;
use crate::ws_model::WebsocketEvent;
use crate::websocket::WebSockets;
use async_trait::async_trait;


/// Websocket endpoint
pub static WS_ENDPOINT: &str = "ws";

/// Construct the aggregated stream endpoint
pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{symbol}@aggTrade")
}
pub struct BinanceBehavior;

#[async_trait]
impl ExchangeBehavior for BinanceBehavior {
    /// Start the stream for every endpoint that we would like to connec tto
    async fn start_stream(&self, exchange_information: &ExchangeInformation, symbol: String) {
        let connections = &exchange_information.connections;
        println!("starting the stream");
        if connections.trade {
            tokio::spawn(BinanceBehavior::stream_trade(symbol));
        }
    }

}

impl BinanceBehavior {
    /// Start the trade stream
    async fn stream_trade(symbol: String) {
        println!("starting the trade stream");
        let keep_running = AtomicBool::new(true);
        let agg_trade: String = agg_trade_stream("ethbtc");
        let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
            //logger_tx.send(event.clone()).unwrap();
            match event {
                WebsocketEvent::Trade(trade) => {
                    println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
                }
                _ => (),
            };
    
            Ok(())
        });
    

        web_socket.connect(symbol).await.unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }
        web_socket.disconnect().await.unwrap();
        println!("disconnected");
    }
}