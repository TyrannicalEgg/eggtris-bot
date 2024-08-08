pub mod utils;
pub mod bot;

use utils::websocket::BotrisWebSocket;
use bot::slow_bot::request_moves;

#[tokio::main]
async fn main() {
    let mut ws = BotrisWebSocket::new().await;

    loop {
        let mv_req = ws.read_next_move_request().await;
        let actions = request_moves(&mv_req);
        ws.send_actions(actions).await;
    }
}

