use dotenv::dotenv;
use tokio::net::TcpStream;
use std::env::var;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct WebSocket {
    pub url: String,
    pub ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Default for WebSocket {
    fn default() -> Self {
        Self { url: String::new(), ws_stream: None }
    }
}

impl WebSocket {
    pub fn new() -> Self {
        dotenv().ok();
        let token = var("TOKEN").expect("Token was not found. It can found in the dashboard at botris.com\nPlease set it in the .env file as TOKEN={{token}}");
        let room_key = var("ROOMKEY").expect("Room key was not found. It can found in the menu of the room\nPlease set it in the .env file as ROOMKEY={{room_key}}");
        let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}",token, room_key);
        WebSocket { url, ..Default::default() }
    }

    pub fn new_with_config(token: String, room_key:String) -> Self {
        let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}",token, room_key);
        WebSocket { url, ..Default::default() }
    }

    pub async fn connect(&mut self) {
        println!("Connecting to {}", &self.url);
        let (ws_stream, _) = connect_async(&self.url).await.expect("Failed to connect");
        println!("Connected!!! :3");
        self.ws_stream = Some(ws_stream);
    }
}
