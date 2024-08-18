use crate::utils::{
        board::BoardSimple,
        event_types::{ActionType, RequestMoveType},
};

use crate::utils::types::Command::*;


pub async fn request_moves(event: &RequestMoveType) -> ActionType {
    // let game_state = &event.game_state;

    // let mut board = BoardSimple::new(&game_state.board);
    // board.add_piece(&game_state.current);
    // board.display(&mut std::io::stdout()).unwrap_or(());

    let mut actions = ActionType::new();

    actions.push(SonicDrop);
    actions.push(SonicLeft);
    actions.push(SonicDrop);
    actions.push(SonicRight);
    actions.push(SonicDrop);

    actions
}

