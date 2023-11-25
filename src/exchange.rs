//use crate::websocket::WebSockets;
//use crate::error::*;
use crate::load_config::ExchangeInformation;

/// Structure representing an exchange
pub struct Exchange {
    /// Configuration information about the exchange
    pub exchange_information: ExchangeInformation,
}

impl Exchange {
    pub fn new(exchange_information: ExchangeInformation) -> Self {
        Self {
            exchange_information
        }
    }
}

/* 
impl<'a, WE: serde::de::DeserializeOwned> Exchange<'a, WE> {

    pub fn new<Callback>(
    ) -> Exchange<'a, WE> 
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        Exchange {
            rest_url: rest_url,
            websocket_url: websocket_url,
            public_key: public_key,
            private_key: private_key,
            websocket: WebSockets::new(callback, websocket_url),
        }
    } 
}
*/