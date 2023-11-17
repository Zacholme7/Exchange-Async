use binance::ws_model::{CombinedStreamEvent, WebsocketEvent, WebsocketEventUntag};
use tokio;

#[tokio::main]
async fn main() {
    // event channel
    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<WebsocketEvent>();

    // an unbounded channel to represent if we should close the connection????
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();

    // spawn a new tokio thread to recieve message and close if we need to
    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                event = logger_rx.recv() => println!("{event:?}"),
                _ = close_rx.recv() => break 'hello
            }
        }
    });

    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(market_websocket(logger_tx.clone())),
    ];

    /* 
    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<WebsocketEvent>();
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                event = logger_rx.recv() => println!("{event:?}"),
                _ = close_rx.recv() => break 'hello
            }
        }
    });
    // private api
    //user_stream().await;
    //user_stream_websocket().await;
    // public api
    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(market_websocket(logger_tx.clone())),
        Box::pin(kline_websocket(logger_tx.clone())),
        Box::pin(all_trades_websocket(logger_tx.clone())),
        Box::pin(last_price(logger_tx.clone())),
        Box::pin(book_ticker(logger_tx.clone())),
        Box::pin(combined_orderbook(logger_tx.clone())),
        Box::pin(custom_event_loop(logger_tx)),
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    select! {
        _ = wait_loop => { println!("Finished!") }
        _ = tokio::signal::ctrl_c() => {
            println!("Closing websocket stream...");
            close_tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    */
}

#[allow(dead_code)]
async fn market_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = agg_trade_stream("ethbtc");
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

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}
