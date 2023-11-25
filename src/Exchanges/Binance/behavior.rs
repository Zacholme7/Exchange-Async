use async_trait::async_trait;
use crate::exchange::ExchangeBehavior;
use crate::load_config::*;


/// Websocket endpoint
pub static WS_ENDPOINT: &str = "ws";

/// Construct the aggregated stream endpoint
pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{symbol}@aggTrade")
}
pub struct BinanceBehavior;

#[async_trait]
impl ExchangeBehavior for BinanceBehavior {
        fn stream_trade(&self) {
                let agg_trade: String = agg_trade_stream("ethbtc");

                let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
                    logger_tx.send(event.clone()).unwrap();
                    match event {
                        WebsocketEvent::Trade(trade) => {
                            println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
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
            
                web_socket.connect(&agg_trade).await.unwrap(); // check error
                if let Err(e) = web_socket.event_loop(&keep_running).await {
                    println!("Error: {e}");
                }
                web_socket.disconnect().await.unwrap();
                println!("disconnected");
        }

        fn stream_orderbook(&self) {
                todo!()
        }
}