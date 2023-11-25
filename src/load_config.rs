use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

/// Represents a configuration for an exchange 
/// as defined in config.json
#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeConfig {
        pub binance: ExchangeInformation,
        pub poloniex: ExchangeInformation,
}

/// The configuation information for each exchange
#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeInformation {
        pub websocket_url: String,
        pub rest_url: String,
        pub public_key: String,
        pub private_key: String,
        pub connections: WebsocketEndpoints,
}

/// Represents that endpoints that we would like to
/// connect to for the specific excahnge
#[derive(Serialize, Deserialize, Debug)] 
pub struct WebsocketEndpoints {
        pub trade: bool,
        pub orderbook: bool
}

/// Load the configuration file and turn it into a hashmap with all of our configuration variables
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<ExchangeConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: ExchangeConfig = serde_json::from_str(&contents)?;
        Ok(config)
}

