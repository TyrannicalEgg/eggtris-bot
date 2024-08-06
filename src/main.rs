use std::env::var;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// const CONNECTION: &'static str = "wss://botrisbattle.com/ws?token={8e347b5c-1cfd-4722-85ec-7d24323417e3}&roomKey={vndn5ta5e7kdddnxlkzsploe}";

#[tokio::main]
async fn main() {
    let token = var("TOKEN").expect("Token was not found");
    let room_key = var("ROOMKEY").expect("Room Key was not found");

    let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}", token, room_key);

    println!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();
    
    let msg = Message::Text("Hello echo server".into());
    
    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read the message");
        println!("Received a message: {}", message);
    }
    
    // println!("sending message {}", msg);
    // write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read the message");
        println!("Received a message: {}", message);
    }
}
