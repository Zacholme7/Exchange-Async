#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub recv_window: u64,

    pub binance_us_api: bool,

    pub timeout: Option<u64>,
}

impl Config {

    /// Default config implementation
    fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com".into(),

            recv_window: 5000,
            binance_us_api: false,

            timeout: None,
        }
    }

    // Testnet config implementation
    pub fn testnet() -> Config {
        Config {
            rest_api_endpoint: "https://testnet.binance.vision".into(),
            ws_endpoint: "wss://testnet.binance.vision".into(),

            futures_rest_api_endpoint: "https://testnet.binancefuture.com".into(),
            futures_ws_endpoint: "wss://testnet.binancefuture.com".into(),

            recv_window: 5000,
            binance_us_api: false,

            timeout: None,
        }
    }
}

