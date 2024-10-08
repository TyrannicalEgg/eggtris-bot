pub mod utils;
pub mod bot;

use std::io::Write;

use crossterm::{
    cursor::{self, Hide, Show, MoveTo}, queue, style::{Stylize, PrintStyledContent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use utils::websocket::BotrisWebSocket;
use utils::event_types::ServerEvent;
use bot::nakamuraas_voracity_bot::Bot;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        queue!(
            std::io::stdout(),
            Show,
            LeaveAlternateScreen,
        ).unwrap();
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    });

    let mut stdout = std::io::stdout();

    queue!(
        stdout,
        EnterAlternateScreen,
        Hide,
        MoveTo(0,0),
        PrintStyledContent("Running Eggtris Bot\n\n".dark_red().bold()),
    ).unwrap();

    stdout.flush().unwrap();
    
    let mut ws = BotrisWebSocket::new().await;

    queue!(stdout,
        cursor::MoveToNextLine(3),
        cursor::SavePosition
    ).unwrap();

    let mut bot = Bot::new();

    loop {
        if let Some(message) = ws.read().await {
            match message {
                ServerEvent::RequestMove { payload } => {
                    let actions = bot.request_moves(&payload).await;
                    ws.send_actions(actions).await;
                },
                ServerEvent::GameReset { payload: _ } => {
                    println!("Game Reset!");
                    bot = Bot::new();
                }
                ServerEvent::RoundOver { payload: _ } => {
                    println!("Round Over!");
                    bot = Bot::new();
                }
                ServerEvent::GameOver { payload: _ } => {
                    println!("Game Over!");
                    bot = Bot::new();
                }
                _ => {}
            }
        }
    }

    // loop {
    //     let mv_req = ws.read_next_move_request().await;
    //     let actions = bot.request_moves(&mv_req).await;
    //     ws.send_actions(actions).await;
    // }
}

