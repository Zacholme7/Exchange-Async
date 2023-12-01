use serde::{Deserialize, Serialize};

/// Represents the high level websocket events that we can recieve 
/// from the hyperliquid websocket streams
//---------------------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "channel")]
pub enum WebsocketEvent {
    #[serde(alias = "trades")]
    Trades(Box<TradeEvent>),
    #[serde(alias = "error")]
    Error(Box<ErrorEvent>),
    #[serde(alias = "subscriptionResponse")]
    SubscriptionResponse(Box<SubscriptionResponseEvent>),
    #[serde(alias = "l2Book")]
    Orderbook(Box<OrderbookEvent>),
}
//---------------------------------------

/// Response subscription message after subscribing to stream
//---------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionResponseEvent {
    pub data: SubscriptionInfo,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionInfo {
    pub method: String,
    pub subscription: Subscription,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    #[serde(rename = "type")]
    pub type_: String,  
    pub coin: String,
    #[serde(rename = "nSigFigs")]
    pub n_sig_figs: Option<i32>,
}
//---------------------------------------

/// ----------- Orderbook stream -------------
/// Represents the data structure for the 'l2Book' channel
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookEvent {
    pub coin: String,
    pub time: u64,
    pub levels: Vec<Vec<Level>>,  // Nested array of Level objects
}
/// Represents an individual level in the orderbook
#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub px: String,
    pub sz: String,
    pub n: i32,
}
//---------------------------------------


/// Trade stream
//---------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeEvent {
    pub data: Vec<Trade>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
}
//---------------------------------------




/// Error upon connecting to stream
//---------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorEvent {
    pub data: String,
}
//---------------------------------------