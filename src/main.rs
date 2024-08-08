pub mod utils;
pub mod bot;

use utils::websocket::WebSocket;
use bot::bot::request_moves;
use serde_json::{from_str, json};
use utils::event_types::{ActionType, ServerEvent};

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let mut ws = WebSocket::new();
    ws.connect().await;
    let ws_stream = ws.ws_stream.expect("HUH");

    let (mut write, mut read) = ws_stream.split();

    loop {
        if let Some(message) = read.next().await{
            let message = message.expect("Failed to read the message").to_string();
            println!("{:#}", &message);
            let message: ServerEvent = from_str(&message).unwrap();

            println!("Received a message:\n{:#?}", message);

            match message {
                ServerEvent::RequestMove { payload: event } => {
                    let moves: ActionType = request_moves(&event);
                    let msg = send_moves(moves);
                    write.send(msg).await.unwrap();
                    write.flush().await.unwrap();
                },
                _ => ()
            };

        }
    }
}

fn send_moves(moves: ActionType) -> Message {
    let event = ServerEvent::Action { payload: moves };
    Message::Text(json!(event).to_string())
}
