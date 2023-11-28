use exchange_async::load_config::{load_config, Exchanges};
use exchange_async::exchange::Exchange;
use tokio::time::{sleep, Duration};
use tokio;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = load_config("src/config.json").unwrap();

    // Create all of the exchanges
    let exchanges: Vec<Exchange> = vec![
        Exchange::new(Exchanges::Binance),
    ];

    // Start the stream for each of the exchanges
    for exchange in exchanges {
        exchange.behavior.start_stream(&config.binance).await;
    }

    // Spin loop
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}