


pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{symbol}@aggTrade")
}

pub struct WebSockets<'a, WE> {
    pub socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>, Response>,
}

