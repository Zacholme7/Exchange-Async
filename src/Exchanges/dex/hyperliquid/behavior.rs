use std::sync::atomic::AtomicBool;
use crate::exchange::ExchangeBehavior;
use crate::load_config::ExchangeInformation;
use crate::websocket::WebSockets;
use crate::exchanges::dex::hyperliquid::data_models::*;
use async_trait::async_trait;
use serde_json::json;


// URLs and endpoints
static WS_URL: &str = "wss://api.hyperliquid.xyz/ws";

pub struct HyperliquidBehavior;

#[async_trait]
impl ExchangeBehavior for  HyperliquidBehavior {
    /// Start the stream for every endpoint that we would like to connec tto
    async fn start_stream(&self, exchange_information: &ExchangeInformation) {
        // streams that we want to connect to
        let connections = &exchange_information.connections;

        // start a trade stream for each of the symbols that we want to connec to
        if connections.trade.should_connect {
            for symbol in &connections.trade.symbols {
                tokio::spawn(HyperliquidBehavior::stream_trade(symbol.to_string()));
            }
        }

        if connections.orderbook.should_connect {
            for symbol in &connections.orderbook.symbols {
                tokio::spawn(HyperliquidBehavior::stream_orderbook(symbol.to_string()));
            }
        }
    }
}

impl HyperliquidBehavior {
    // Stream the trades for a specific coin
    async fn stream_trade(symbol: String) {
        let keep_running = AtomicBool::new(true);
        // construct the websocket with our callback function
        let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
            //logger_tx.send(event.clone()).unwrap();
            match event {
                WebsocketEvent::Trades(trade) => {
                    println!("{:?}", trade.data);
                }
                _ => (),
            };
            Ok(())
        });

        // construt and send the subscription
        let subscription = json!({
            "type": "trade",
            "coin": symbol
        });
    
        // Creating the message object
        let message = json!({
            "method": "subscribe",
            "subscription": subscription
        });
    
        // construt and send the subscription
        let subscription_string = serde_json::to_string(&message).unwrap();
        web_socket.connect_subscribe_and_stream(WS_URL.to_string(), subscription_string).await;
    }

    /// Stream the level two order book
    async fn stream_orderbook(symbol: String) {
        // construct the websocket with our callback function
        let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
            //logger_tx.send(event.clone()).unwrap();
            println!("{:?}", event);
            match event {
                WebsocketEvent::Orderbook(ob) => {
                    println!("{:?}", ob);
                }
                WebsocketEvent::SubscriptionResponse(resp) => {
                    println!("{:?}", resp);

                }
                WebsocketEvent::Error(resp) => {
                    println!("{:?}", resp);
                }
                _ => (),
            };
            Ok(())
        });

        // construt and send the subscription
        let subscription = json!({
            "type": "l2Book",
            "coin": symbol
        });
    
        // Creating the message object
        let message = json!({
            "method": "subscribe",
            "subscription": subscription
        });
    
        // Converting the message object to a JSON string
        let subscription_string = serde_json::to_string(&message).unwrap();
        web_socket.connect_subscribe_and_stream(WS_URL.to_string(), subscription_string).await;
    }

}