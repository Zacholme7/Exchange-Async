


pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{symbol}@aggTrade")
}

// struct generic over two parameters
// 'a the lifetime, indicates how long the references within the struct are valid
// WE websocket event, must be type that can be deserialized
pub struct WebSockets<'a, WE> {
    pub socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>, Response>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>, // a box allocated function, has type WE and returns result, hold ref with lifetime a and send betwteen threads
    conf: Config,
}


impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    // callback is literally a callback function
    // returna new websocket instance
    pub fn new<Callback>(handler: Callback) -> Websocket<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            conf: Config::default()
        }
    }

    // connect to a websocket endpoint
    // takes a mutable referene to self and a &str representing the ending for the websocket stream
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        // construct the wss that we want to connect to
        let wss: String = format!("{}/{}/{}", self.conf.ws_endpoint, WS_ENDPOINT, endpoint);
        // parse it into a url
        let url = Url::parse(&wss)?;

        // do the actual connecting
        self.handle_connect(url).await
    }


    async fn handle_connection(&mut self, url: Url) -> Result<()> {
        // asyncronously connect
        // ahndle the result
        match connect_async(url).await {
            Ok(answer) => {
                // we were able to connect
                self.socket = Some(answer); // set the socket in the struct
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}")))
        }
    }

    // disconnect the websocket
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("not able to close the connection".to_string()))
        }
    }

    // the main event loop
    // this will recieve the messages and parse htem based on our callback
    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            // not sure why doing it like this? why dont we just take the socket
            // maybe it has to do with the lifetimes and the issues that I had adding a socket to a struct
            // regardless get mutable reference to socket
            // see what ref means
            if let Some((ref mut socket, _)) = self.socket {
                // toto return error instead of panic? from the code and i can implement

                // get the next message
                let message = socket.next().await.unwrap()?;

                // once we get a message, process it
                match message {
                    Message::Text(msg) => {
                        // check if the message is empty
                        if msg.is_empty() {
                            return Ok(());
                        }

                        // deserialize the message, must be in a format we are expecting in the other file
                        let event: WE = from_str(msg.as_str())?;

                        // this is interesting syntax, this should bhe looked into
                        (self.handler)(event)?; // process the message with our callback function
                    }
                    // the other messages that we might recieve
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {e:?}")));
                    }
                }
            }
        }
        Ok(())
    }
























}

