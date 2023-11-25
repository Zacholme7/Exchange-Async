use exchange_async::load_config::load_config;
use exchange_async::exchange::Exchange;
use tokio;


#[tokio::main]
async fn main() {
    // read in all of the configuration information
    let config = load_config("src/config.json").unwrap();

    // vector that will hold the exchanges
    let exchanges: Vec<Exchange> = vec![
        Exchange::new(config.binance),
        Exchange::new(config.poloniex)
    ];

    for exchange in exchanges {
        // start each of these in their own thread????
        // something liek std::thread(exchange.start_stream())
        // start stream will then call tokio::spawn
    }

    // main funciton can then poll, restart connecions, etc
}







