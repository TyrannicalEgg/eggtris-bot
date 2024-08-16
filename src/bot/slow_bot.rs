use crate::utils::{
        board::BoardSimple,
        event_types::{ActionType, RequestMoveType},
};

use tokio::time::{Duration, sleep};

pub async fn request_moves(event: &RequestMoveType) -> ActionType {
    let game_state = &event.game_state;

    let mut board = BoardSimple::new(&game_state.board);
    board.add_piece(&game_state.current);
    board.display(&mut std::io::stdout()).unwrap_or(());

    let actions = ActionType::new();

    sleep(Duration::from_millis(3500)).await;

    actions
}

