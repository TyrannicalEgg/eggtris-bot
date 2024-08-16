use crate::utils::{
    event_types::ActionType,
    types::GameState,
    board::BoardSimple,
};

pub struct Actions<'a> {
    pub game_state: &'a GameState,
    pub board: BoardSimple,
    pub actions: ActionType,
}

impl<'a> Actions<'a> {
    pub fn new(game_state: &'a GameState) -> Self{
        Actions {
            game_state,
            board: BoardSimple::new(&game_state.board),
            actions: ActionType::new(),
        }
    }
}
