pub mod websocket;
pub mod types;
pub mod game_info;

use serde_json::{from_str, json};
use types::types::Command;
use websocket::event_types::{ActionType, ServerEvent};

use dotenv::dotenv;
use std::{env::var, fmt::write};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = var("TOKEN").expect("Token was not found");
    let room_key = var("ROOMKEY").expect("Room Key was not found");

    let url = format!("wss://botrisbattle.com/ws?token={}&roomKey={}", token, room_key);

    println!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();

    let drop_move = ServerEvent::Action { payload: ActionType { commands: vec![Command::SonicDrop] } };
    let drop_message = Message::Text(json!(drop_move).to_string());

    loop {
        if let Some(message) = read.next().await{
            let message = message.expect("Failed to read the message").to_string();
            println!("{:#}", &message);
            let message: ServerEvent = from_str(&message).unwrap();

            match message {
                ServerEvent::RequestMove { payload: _ } => {
                    println!("Sending Move?");
                    write.send(drop_message.clone()).await.unwrap();
                    write.flush().await.unwrap();
                },
                _ => println!("Did not send move :3")
            };

            println!("Received a message:\n{:#?}", message);
        }
    }
}
