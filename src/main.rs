pub mod websocket;
pub mod types;
pub mod game_info;

use websocket::event_types::ServerEvent;

use dotenv::dotenv;
use std::env::var;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use serde_json::from_str;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = var("TOKEN").expect("Token was not found");
    let room_key = var("ROOMKEY").expect("Room Key was not found");

    let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}", token, room_key);

    println!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Agent Network");

    let (mut _write, mut read) = ws_stream.split();
    
    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read the message").to_string();
        println!("{}", message);
        let message: ServerEvent = from_str(&message).expect("Received malformed message");
        println!("Received a message: {:?}", &message);
    }
    
    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read the message").to_string();
        let message: ServerEvent = from_str(&message).expect("Received malformed message");
        println!("Received a message: {:?}", message);
    }
}
