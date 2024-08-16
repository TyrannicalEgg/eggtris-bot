use std::env::var;
use dotenv::dotenv;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream, tungstenite::Message};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use serde_json::{from_str, json};
use crossterm::style::Stylize;

use crate::utils::event_types::{ServerEvent, RequestMoveType, ActionType};

/// Prints errors in the format of
/// Error: Description
/// $args
///
/// $args can be any number of string literals
/// which will each be printed on a seperate line
///
/// # Example
/// ```
/// error!("Description",
///     "You got an error"
/// );
/// ```
///
/// Output:
/// """
/// Error: Descriptions
///     You got an error
/// """
macro_rules! error {
    ($err:literal$(, )?$($args:literal$(,)?)*) => {
        eprintln!("{}: {}", "Error".red().bold(), $err);
        $(eprintln!("    {}", $args);)*
    };
}

/// Represents an unconnected websocket
/// holds url when ready for connecting
pub struct WebSocket {
    pub url: String,
}

/// Represents a websocket that is connected
/// to the botrisbattle room
pub struct BotrisWebSocket {
    pub ws: WebSocket,
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocket {
    /// Creates a new websocket prepared to connect to a 
    /// botrisbattle room using envronment variables TOKEN 
    /// and ROOMKEY to create the url.
    /// The variables must be set before calling this function
    pub fn new() -> Self {
        dotenv().ok();
        let token = match var("TOKEN") {
            Ok(token) => token,
            Err(..) => {
                error!("Token was not found",
                    "The Token can be found in the dashboard at",
                    "https://botrisbattle.com/dashboard",
                    "Please set it in .env file as TOKEN={token}",
                );
                std::process::exit(1);
                
            }
        };
        let room_key = match var("ROOMKEY") {
            Ok(room_key) => room_key,
            Err(..) => {
                error!("Room key was not found.", 
                    "The room key can be found in the room menu",
                    "Please set it in the .env file as ROOMKEY={roomKey}"
                );
                std::process::exit(1);
            }
        };
        let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}",token, room_key);
        WebSocket { url }
    }

    /// Creates a new websocket prepared to connect to a botrisbattle room
    /// using the given token and room_key
    /// ### parameters
    /// token: Generated api token from Dashboard 
    /// can be found at https://botrisbattle.com/dashboard
    /// room_key: Provided roomKey from the room host
    pub fn config_new(token: String, room_key:String) -> Self {
        let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}",token, room_key);
        WebSocket { url}
    }

    /// Connects to the botrisbattle room and returns a connected websocket
    /// 
    /// # Example
    /// ```
    /// let ws = WebSocket::new().connect().async;
    /// ```
    pub async fn connect(self) -> BotrisWebSocket {
        println!("Connecting to {}", &self.url);
        let ws_stream = match connect_async(&self.url).await {
            Ok(ws_stream) => ws_stream.0,
            Err(..) => {
                error!("Failed to connect :(",
                    "Make sure TOKEN and ROOMKEY are set correctly in .env"
                );
                std::process::exit(1);
            }
        };
        println!("{}", "Connected!!! :3".green().bold());
        let (write, read) = ws_stream.split();
        BotrisWebSocket { ws: self, write, read }
    }
}

impl BotrisWebSocket {
    /// Creates a new websocket connected to a botrisbattle room using
    /// the envronment variables TOKEN and ROOMKEY to form the url
    ///
    /// # Example
    /// ```
    /// let ws = BotrisWebSocket::new().await;
    /// ```
    pub async fn new() -> Self {
        WebSocket::new().connect().await
    }

    /// Reads a message from the botrisbattle server and returns 
    /// a parsed struct ServerEvent representing its contents.
    /// It will return None on an error.
    async fn read (&mut self) -> Option<ServerEvent> {
        let message = self.read.next().await?;
        let message = match message {
            Ok(msg) => msg.to_string(),
            Err(err) => {
                error!("Failed to read message");
                eprintln!("{}", err);
                return None
            }
        };
        // println!("{:#}", message); // Uncomment this from debugging
        match from_str(&message) {
            Ok(msg) => {
                // println!("{:#?}", msg); // Uncomment this for debugging
                msg
            },
            Err(err) => {
                error!("Failed to parse message");
                eprintln!("{}\n{}", message, err);
                None
            }
        }
    }

    /// Reads messages from the botrisbattle server and returns the next
    /// move request parsed into a RequestMoveType struct representing
    /// its contents. It will skip past any non request move type messages
    pub async fn read_next_move_request(&mut self) -> RequestMoveType {
        loop {
            if let Some(message) = self.read().await {
                match message {
                    ServerEvent::RequestMove { payload } => return payload,
                    _ => continue
                }
            }
        }
    }

    /// Sends the server an action type message with the moves you want to 
    /// make.
    ///
    /// # Example
    /// ```
    /// let mut actions = ActionType::new();
    /// actions.push(Command::MoveLeft);
    /// actions.push(Command::MoveRight);
    /// ws.send_actions(actions).await;
    /// ```
    pub async fn send_actions(&mut self, actions: ActionType) {
        let event = ServerEvent::Action { payload: actions };
        let msg = Message::Text(json!(event).to_string());
        match self.write.send(msg).await {
            Ok(..) => (),
            Err(err) => {
                error!("Failed to send message T_T");
                eprintln!("{}", err);
            }
        };
        match self.write.flush().await {
            Ok(..) => (),
            Err(err) => {
                error!("Failed to send message T_T");
                eprintln!("{}", err);
            }
        }
    }
}
