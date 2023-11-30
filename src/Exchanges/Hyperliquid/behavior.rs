use crate::Exchanges::Hyperliquid::data_models::*;
use std::sync::atomic::AtomicBool;
use crate::exchange::ExchangeBehavior;
use crate::load_config::ExchangeInformation;
use crate::Exchanges::Hyperliquid::data_models::*;
use crate::websocket::WebSockets;
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
    }
}

impl HyperliquidBehavior {
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

        // connect to the websocket
        web_socket.connect(WS_URL.to_string()).await.unwrap();

        // construt and send the subscription
        let subscription = WebsocketSubscription {
            type_: "trades".to_string(),
            coin: symbol.to_string(),
        };
        let message = SubscribeMessage {
            method: "subscribe".to_string(),
            subscription,
        };
        let subscription_string = serde_json::to_string(&message).unwrap();
        //println!("{subscription_string}");
    
        web_socket.subscribe(subscription_string).await.unwrap();

        // start the event loop
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }

        // disconnect from the websocket
        web_socket.disconnect().await.unwrap();
    }
}