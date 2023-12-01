use std::sync::atomic::AtomicBool;
use crate::exchange::ExchangeBehavior;
use crate::load_config::ExchangeInformation;
use crate::exchanges::cex::binance::data_models::*;
use crate::websocket::WebSockets;
use async_trait::async_trait;

// URLs and endpoints
static WS_URL: &str = "wss://stream.binance.com:9443/ws/";
static TRADE_STREAM: &str = "@trade";
static ORDERBOOK_STREAM: &str = "@depth";

pub struct BinanceBehavior;

#[async_trait]
impl ExchangeBehavior for BinanceBehavior {
    /// Start the stream for every endpoint that we would like to connec tto
    async fn start_stream(&self, exchange_information: &ExchangeInformation) {
        // streams that we want to connect to
        let connections = &exchange_information.connections;

        // connect to the trade stream
        if connections.trade.should_connect {
            self.start_streams(&connections.trade.symbols, TRADE_STREAM).await;
        }

        // connect to the orderbook stream
        if connections.orderbook.should_connect {
            self.start_streams(&connections.orderbook.symbols, ORDERBOOK_STREAM).await;
        }
    }
}

impl BinanceBehavior {
    /// Helper function to delegate the stream connection to the correct function
    async fn start_streams(&self, symbols: &[String], stream_type: &str) {
        for symbol in symbols {
            let url = format!("{}{}{}", WS_URL, symbol, stream_type);
            if stream_type == TRADE_STREAM {
                tokio::spawn(BinanceBehavior::stream_trade(url));
            } else if stream_type == ORDERBOOK_STREAM {
                tokio::spawn(BinanceBehavior::stream_orderbook(url));
            }
        }
    }

    /// Start the trade stream
    async fn stream_trade(url: String) {
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

        web_socket.connect_and_stream(url).await;
     }

    /// Start the orderbook stream
    async fn stream_orderbook(url: String) {
        println!("Connecting to orderbook stream");
        let keep_running = AtomicBool::new(true); // Used to control the event loop
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
    
        web_socket.connect_and_stream(url).await;
    }
}

