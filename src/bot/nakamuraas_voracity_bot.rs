use crate::utils::{
    // board::BoardSimple,
    event_types::{ActionType, RequestMoveType},
    piece::Piece::{self, *},
    types::{Command::{self, *}, GameState}
};

pub struct Bot {
    bag: u32,
    piece: u32,
}

impl Bot {
    pub fn new () -> Self {
        Self::default()
    }

    pub fn default() -> Self {
        Bot {
            bag: 0,
            piece: 0,
        }
    }

    pub async fn request_moves(&mut self, event: &RequestMoveType) -> ActionType {
        let game_state = &event.game_state;

        // let mut board = BoardSimple::new(&game_state.board);
        // board.add_piece(&game_state.current);
        // board.display(&mut std::io::stdout()).unwrap_or(());

        let mut actions = ActionType::new();
        self.piece += 1;

        match self.bag {
            0 => {actions.append(&mut bag_0(self.piece, &game_state));},
            _ => {self.bag = 0; self.piece = 0}
        }
        if self.piece == 7 {
            self.bag += 1;
            self.piece = 0;
        }
        
        actions
    }
}

fn bag_0(piece_num: u32, game_state: &GameState) -> Vec<Command> {
    let piece_cur = game_state.current.piece;
    let piece_held = game_state.held;
    let piece_next = game_state.queue[0];

    let mut commands: Vec<Command> = Vec::new();

    let mut piece: Piece = 
        if (piece_num >= 6) ^ matches!(piece_cur, T) { 
            commands.push(Hold);
            hold_piece(piece_held, piece_next) 
        } else {
            piece_cur 
        };


    // let piece = if self.piece != 7 {
    //     match piece {
    //         T => {
    //             commands.push(Hold);
    //             match held {
    //                 Some(piece) => piece,
    //                 None => next
    //             }
    //         },
    //         S => {
    //             if self.piece < 6 {
    //                 match held {
    //                     Some(_piece) => {
    //                         println!("Fuck!!!");
    //                         S
    //                     },
    //                     None => {
    //                         commands.push(Hold);
    //                         next
    //                     }
    //                 }
    //             } else {
    //                 S
    //             }
    //         }
    //         _ => {piece}
    //     }
    // } else {
    //     match piece {
    //         T => piece,
    //         _ => {
    //             commands.push(Hold);
    //             T
    //         }
    //     }
    // };


    match piece {
        I => {commands.append(&mut vec!(RotateCw, SonicLeft));},
        J => {commands.append(&mut vec!(RotateCw, MoveLeft, MoveLeft, MoveLeft));},
        L => {commands.append(&mut vec!(RotateCw, MoveLeft, SonicDrop, RotateCw));},
        Z => {commands.append(&mut vec!(MoveRight, MoveRight));},
        O => {commands.append(&mut vec!(SonicRight));},
        S => {commands.append(&mut vec!(RotateCw, SonicRight));},
        T => {commands.append(&mut 
            vec!(RotateCcw, MoveRight, MoveRight, MoveRight, SonicDrop, RotateCcw)
        );},
    };

    commands
}

fn hold_piece(piece_held: Option<Piece>, piece_next: Piece) -> Piece {
    match piece_held {
        Some(piece) => piece,
        None => piece_next
    } 
}
