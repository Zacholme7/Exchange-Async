use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

/// Represents a configuration for an exchange 
/// as defined in config.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeConfig {
        pub binance: ExchangeInformation,
        pub poloniex: ExchangeInformation,
        pub hyperliquid: ExchangeInformation,
}

/// The configuation information for each exchange
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeInformation {
        pub public_key: String,
        pub private_key: String,
        pub connections: WebsocketEndpoints,
}

/// Represents information regarding the endpoints that we want to connect to
#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct WebsocketEndpoints {
        pub trade: ConnectionInformation,
        pub orderbook: ConnectionInformation,
}

/// Holds connection information for the endpoint that we want to connect to
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionInformation {
        pub should_connect: bool,
        pub symbols: Vec<String>,
}

/// Load the configuration file and turn it into a hashmap with all of our configuration variables
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<ExchangeConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: ExchangeConfig = serde_json::from_str(&contents)?;
        Ok(config)
}

/// Represents the exchanges that we have added
pub enum Exchanges {
        Binance,
        Hyperliquid
        //Poloniex,
}


