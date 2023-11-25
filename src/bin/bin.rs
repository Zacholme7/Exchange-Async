use exchange_async::load_config::{load_config, Exchanges};
use exchange_async::exchange::Exchange;

use tokio;


#[tokio::main]
async fn main() {
    // read in all of the configuration information
    let config = load_config("src/config.json").unwrap();

    // vector that will hold the exchanges
    let exchanges: Vec<Exchange> = vec![
        Exchange::new(&config.binance, Exchanges::Binance),
        //Exchange::new(config.poloniex, Exchanges::Poloniex)
    ];

    for exchange in exchanges {
        /* 
        tokio::spawn(async move {
            exchange.behavior.start_stream(&config.binance).await;
        });
        */
        //exchange.behavior.start_stream();
        // start each of these in their own thread????
        // something liek std::thread(exchange.start_stream())
        // start stream will then call tokio::spawn
    }

    // main funciton can then poll, restart connecions, etc
}







