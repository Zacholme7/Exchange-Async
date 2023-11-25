use crate::load_config::{ExchangeInformation, Exchanges};
use crate::Exchanges::Binance::behavior::BinanceBehavior;
//use crate::Exchanges::Poloniex::behavior::PoloniexBehavior;
use async_trait::async_trait;

/// Common trait that all exchanges should implement
/// Defines the processing behavior for the specific exchange
#[async_trait]
pub trait ExchangeBehavior {
    /// Start the stream for every endpoint that we would like to connect to 
    async fn start_stream(&self, exchange_information: &ExchangeInformation) {
        // From the config, get what endpoints we should connect to
        let connections = &exchange_information.connections;

        // Connect to the trade endpoint
        if connections.trade {
            tokio::spawn(
                self.stream_trade()
            );
        }

        // Connect to the orderbook stream
        if connections.orderbook {
            self.stream_orderbook();
        }
    }
    /// Connect to a trade stream
    fn stream_trade(&self);
    /// Connect to the orderbook stream
    fn stream_orderbook(&self);
}

/// Structure representing an exchange
pub struct Exchange<'a> {
    /// Configuration information about the exchange
    pub exchange_information: &'a ExchangeInformation,
    pub behavior: Box<dyn ExchangeBehavior>
}

impl<'a> Exchange<'a> {
    /// Constructor for an exchange
    pub fn new(exchange_information: &'a ExchangeInformation, exchange_type: Exchanges) -> Self {
        // Retrieve the correct behavior for the exchange
        let behavior: Box<dyn ExchangeBehavior> = match exchange_type {
            Exchanges::Binance => Box::new(BinanceBehavior),
            //Exchanges::Poloniex => Box::new(PoloniexBehavior),
        };

        // Construct the instance
        Self {
            exchange_information,
            behavior
        }
    }
}