use exchange_async::load_config::{load_config, Exchanges};
use exchange_async::exchange::Exchange;
use std::thread;
use futures::StreamExt;
use exchange_async::websocket::WebSockets;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use std::sync::Arc;
use futures::future::join_all;
use tokio;
use futures::future::join;

#[tokio::main]
async fn main() {

    // Load configuration
    let config = load_config("src/config.json").unwrap();

    let binance_config = config.binance.clone();

    // Create a vector of exchanges wrapped in Arc
    let exchanges: Vec<Arc<Exchange>> = vec![
        Arc::new(Exchange::new(binance_config, Exchanges::Binance)),
    ];

    // Iterate over the exchanges
    //let mut futures = Vec::new();
    for exchange in exchanges {
        let exchange_clone = exchange.clone();
        exchange_clone.behavior.start_stream(&exchange_clone.exchange_information, "btcusdt".to_string()).await;
        exchange_clone.behavior.start_stream(&exchange_clone.exchange_information, "ethusdt".to_string()).await;
    }

    while true {
        continue;
    }
    // Wait for all futures to complete
    //join_all(futures).await;
}