use async_trait::async_trait;
use crate::exchange::ExchangeBehavior;
use crate::load_config::*;
use crate::websocket::*;

// URLs and endpoints
static WS_URL: &str = "wss://ws.poloniex.com/ws/public";

pub struct PoloniexBehavior;

#[async_trait]
impl ExchangeBehavior for PoloniexBehavior {
    /// Start the stream for every endpoint that we would like to connec tto
    async fn start_stream(&self, exchange_information: &ExchangeInformation) {
        // streams that we want to connect to
        let connections = &exchange_information.connections;

        // connect to the trade stream
        /* 
        if connections.trade.should_connect {
            self.start_streams(&connections.trade.symbols, TRADE_STREAM).await;
        }

        // connect to the orderbook stream
        if connections.orderbook.should_connect {
            self.start_streams(&connections.orderbook.symbols, ORDERBOOK_STREAM).await;
        }
        */
    }
}

impl PoloniexBehavior {
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
        println!("Connecting to trade stream");
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

        // send the subscription message
        
        websocket.subscribe("")


        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }
        web_socket.disconnect().await.unwrap();
        println!("disconnected");
    }
}