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
        // streams that we want to connect to
        let connections = &exchange_information.connections;
        let symbol = Arc::new(symbol);

        // should we connect to the trade stream
        if connections.trade {
            tokio::spawn(BinanceBehavior::stream_trade(symbol.clone()));
        }

        // should we connect to the orderbook stream
        if connections.orderbook {
            tokio::spawn(BinanceBehavior::stream_orderbook(symbol.clone()));
        }
    }
}

impl BinanceBehavior {
    /// Start the trade stream
    async fn stream_trade(symbol: Arc<String>) {
        println!("starting the trade stream");
        let keep_running = AtomicBool::new(true);
        let url = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol);
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
    

        web_socket.connect(url).await.unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }
        web_socket.disconnect().await.unwrap();
        println!("disconnected");
    }

    async fn stream_orderbook(symbol: Arc<String>) {
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let url = format!("wss://stream.binance.com:9443/ws/{}@depth@100ms", symbol);
        let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
        //    logger_tx.send(event.clone()).unwrap();
            match event {
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
    
        web_socket.connect(url).await.unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }
        web_socket.disconnect().await.unwrap();
        println!("disconnected");
    }
}