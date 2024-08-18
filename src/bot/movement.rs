use crate::utils::{
    board::BoardSimple, event_types::ActionType, piece::Piece, types::{ Command::{self, *}, GameState, PieceData}
};

pub struct Actions<'a> {
    pub game_state: &'a mut GameState,
    pub board: BoardSimple,
    pub actions: ActionType,
}

impl Actions<'_> {
    pub fn take_action(mut self, command: Command) {
        match command {
            Hold => { },
            MoveLeft => {},
            MoveRight => {},
            SonicLeft => {},
            SonicRight => {},
            RotateCw => {},
            RotateCcw => {},
            Drop => {},
            SonicDrop => {},
            HardDrop => {},
        }
        self.actions.push(command);
    }
}

impl<'a> Actions<'a> {
    pub fn new(game_state: &'a mut GameState) -> Self{
        Actions {
            board: BoardSimple::new(&game_state.board),
            game_state,
            actions: ActionType::new(),
        }
    }
    pub fn display_board(&self) -> std::io::Result<()>{
        self.board.display(&mut std::io::stdout())
    }
}

