use tokio::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use futures::StreamExt;
use serde_json::from_str;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;
use crate::error::*;

/// Websocket struct representing a connection to an exchange
pub struct WebSockets<'a, WE> {
    /// Websoccket connection
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    /// Callback function to handle incomming messages
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>,
    /// Websocket url
    ws_endpoint: &'a str,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    /// Constructor to create a new websocket
    pub fn new<Callback>(handler: Callback, ws_endpoint: &str) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            ws_endpoint: ws_endpoint,
        }
    }

    /// Connect to a specified endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.ws_endpoint, WS_ENDPOINT, endpoint);
        let url = Url::parse(&wss)?;
        self.handle_connection(url).await
    }


    /// Helper function to do the actual connecting
    async fn handle_connection(&mut self, url: Url) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer); 
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}")))
        }
    }

    /// Disconnect from the websocket
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("not able to close the connection".to_string()))
        }
    }

    /// Event loop that will recieve the incomming messages and utilize the callback to parse them
    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((ref mut socket, _)) = self.socket {
                let message = socket.next().await.unwrap()?;
                match message {
                    Message::Text(msg) => {
                        if msg.is_empty() {
                            return Ok(());
                        }
                        let event: WE = from_str(msg.as_str())?;
                        (self.handler)(event)?; // process the message with our callback function
                    }
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

