use binance::ws_model::{CombinedStreamEvent, WebsocketEvent, WebsocketEventUntag};
use tokio;

#[tokio::main]
async fn main() {
    // event channel
    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<WebsocketEvent>();

    // close signaler
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();

    // spawn a new tokio thread to recieve message and close if we need to
    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                event = logger_rx.recv() => println!("{event:?}"), // this will give us all the logging messages
                _ = close_rx.recv() => break 'hello // this will break us out if we have some closing logic
            }
        }
    });

    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(market_websocket(logger_tx.clone())), // clone the sender
    ];

    for stream in streams{
        tokio::spawn(stream);
    }

    select! {
        _ = wait_loop => ( println!("finished"))
        _ = tokio::signal::ctrl_c() {
            println!("Closing websocket stream...;")
            close_tx.send(true).unwrap(); // send close message to bdR
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // not sure why we are sleeping here
        }
    }

}

async fn market_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = agg_trade_stream("ethbtc"); // this will configure the api ending for ethbtc for the aggredated trades
    
    // create websocket with handling event
    let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
        logger_tx.send(event.clone()).unwrap();
        match event {
            WebsocketEvent::Trade(trade) => {
                println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            }
            _ => (),
        };

        Ok(())
    });

    // connect to the websocket
    web_socket.connect(&agg_trade).await.unwrap(); // check error

    // main event loop
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}
