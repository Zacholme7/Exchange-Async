use async_trait::async_trait;
use crate::exchange::ExchangeBehavior;
use crate::load_config::*;

pub struct PoloniexBehavior;

#[async_trait]
impl ExchangeBehavior for PoloniexBehavior {

    fn trade(&self) {
        println!("in poloniex trade");
    }

    fn orderbook(&self) {
        println!("in poloniex orderbook");
    }

}