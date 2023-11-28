use crate::load_config::{ExchangeInformation, Exchanges};
use crate::Exchanges::Binance::behavior::BinanceBehavior;
//use crate::Exchanges::Poloniex::behavior::PoloniexBehavior;
use std::sync::Arc;
use async_trait::async_trait;

/// Common trait that all exchanges should implement
/// Defines the processing behavior for the specific exchange
#[async_trait]
pub trait ExchangeBehavior: Send + Sync {
    /// Start the stream for every endpoint that we would like to connect to 
    async fn start_stream(&self, exchange_information: &ExchangeInformation, symbol: String);
}

/// Structure representing an exchange
pub struct Exchange {
    pub exchange_information: ExchangeInformation,
    pub behavior: Arc<dyn ExchangeBehavior>
}

impl Exchange {
    /// Constructor for an exchange
    pub fn new(exchange_information: ExchangeInformation, exchange_type: Exchanges) -> Self {
        println!("creating the exchange");
        // Retrieve the correct behavior for the exchange
        let behavior: Arc<dyn ExchangeBehavior> = match exchange_type {
            Exchanges::Binance => Arc::new(BinanceBehavior),
            // Other exchanges...
        };

        // Construct the instance
        Self {
            exchange_information,
            behavior
        }
    }
}