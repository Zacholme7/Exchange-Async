use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::exchange::ExchangeBehavior;
use crate::load_config::ExchangeInformation;
use crate::ws_model::WebsocketEvent;
use crate::websocket::WebSockets;
use async_trait::async_trait;

/// The websocket url
static WS_URL: &str = "wss://stream.binance.com:9443/";

pub fn construct_trade_stream(symbols: &Vec<String>) -> String {
    let stream_names: Vec<String> = symbols
        .iter()
        .map(|symbol| format!("{}@trade", symbol))
        .collect();

    // Join the stream names with '/'
    let stream_names_combined = stream_names.join("/");

    // Construct the full WebSocket URL
    format!("{}stream?streams={}", WS_URL, stream_names_combined)
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
        if connections.trade.should_connect {
            // construct the url

            let url = construct_trade_stream(&connections.trade.symbols);
            println!("{:?}", url);

            tokio::spawn(BinanceBehavior::stream_trade(url));
        }

        // should we connect to the orderbook stream
        /* 
        if connections.orderbook.should_connect {
            tokio::spawn(BinanceBehavior::stream_orderbook(symbol.clone()));
        }
        */
    }
}

impl BinanceBehavior {
    /// Start the trade stream
    async fn stream_trade(url: String) {
        let keep_running = AtomicBool::new(true);
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