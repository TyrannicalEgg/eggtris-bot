pub mod utils;
pub mod bot;

use crossterm::{
    cursor, style::Stylize, terminal::{self, ClearType::*}, ExecutableCommand
};

use utils::websocket::BotrisWebSocket;
use bot::slow_bot::request_moves;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();

        std::io::stdout().execute(cursor::Show).unwrap();
        std::process::exit(0);
    });

    let mut stdout = std::io::stdout();

    stdout
        .execute(terminal::Clear(All)).unwrap()
        .execute(cursor::MoveTo(0,0)).unwrap()
        .execute(cursor::Hide).unwrap();
    
    
    println!("{}", "Running Eggtris Bot\n\n".dark_red().bold());
    let mut ws = BotrisWebSocket::new().await;

    stdout
        .execute(cursor::MoveToNextLine(3)).unwrap()
        .execute(cursor::SavePosition).unwrap();


    loop {
        let mv_req = ws.read_next_move_request().await;
        let actions = request_moves(&mv_req).await;
        ws.send_actions(actions).await;
    }
}

