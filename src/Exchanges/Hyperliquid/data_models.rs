use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "channel")]
pub enum WebsocketEvent {
    #[serde(alias = "trades")]
    Trades(Box<TradeEvent>),
    #[serde(alias = "error")]
    Error(Box<ErrorEvent>),
    #[serde(alias = "subscriptionResponse")]
    SubscriptionResponse(Box<SubscriptionResponseEvent>),
}

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


#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorEvent {
    pub data: String,
}

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
    pub type_: String,  // Use an underscore or another name in Rust code
    pub coin: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebsocketSubscription {
    #[serde(rename = "type")]
    pub type_: String,
    pub coin: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeMessage {
    pub method: String,
    pub subscription: WebsocketSubscription,
}