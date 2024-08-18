use crate::utils::{
    types::{Board, PieceData},
    game_info::{BOARD_WIDTH, BOARD_HIGHT},
};


use crossterm::{
    cursor::{self}, queue,
    style::{PrintStyledContent, Stylize},
    terminal::{self, ClearType::*}
};


pub struct BoardSimple {
    pub board: [u16; 21],
}

impl BoardSimple {
    pub fn new(game_board: &Board) -> Self {
        let mut board = [0u16; 21];

        for i in 0..game_board.len() {
            for block in &game_board[i] {
                board[i] |= u16::from(block.is_some());
                board[i] <<= 1;
            }
            board[i] <<= 2;
        }

        BoardSimple {
            board
        }
    }

    // pub fn add_piece(&mut self, piece_data: &PieceData) {
        // let piece = &piece_data.piece;
        // let x = piece_data.x;
        // let y = piece_data.y;
        // let mut piece_matrix = piece.get_matrix();
        // let piece_size = piece_matrix.len().into();

        // for line in &mut piece_matrix {
        //     *line <<= 13 - piece_size - x;
        // }

        // for i in 0..piece_matrix.len() {
        //     *self.board.index_mut(y-i) = piece_matrix[i];
        // }
    // }
}

const LEFT_WALL: u16 = 0;
const RIGHT_WALL: u16 = BOARD_WIDTH * 2 + 1;

impl BoardSimple {
    pub fn display(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        let board = self.board;

        queue!(out, 
            cursor::RestorePosition,
            terminal::Clear(FromCursorDown),
        )?;

        queue!(out, PrintStyledContent("▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁".white()))?;
        for _i in 0..BOARD_HIGHT {
            queue!(out,
                cursor::MoveDown(1),
                cursor::MoveToColumn(LEFT_WALL),
                PrintStyledContent("▎".white()),
                cursor::MoveToColumn(RIGHT_WALL),
                PrintStyledContent("🮇".white())
            )?;
        }
        queue!(out,
            cursor::MoveDown(1),
            cursor::MoveToColumn(LEFT_WALL),
            PrintStyledContent( "▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔".white()),
            cursor::SavePosition,
        )?;

        for line in board {
            queue!(out,
                cursor::MoveUp(1),
                cursor::MoveToColumn(1),
            )?;
            for i in 0..BOARD_WIDTH {
                if(line >> (12 - i)) & 0b1 == 0 {
                    queue!(out, PrintStyledContent("  ".dark_red()))?;
                } else {
                    queue!(out, PrintStyledContent("██".dark_red()))?;
                }
            }
        };

        queue!(out, cursor::RestorePosition)?;

        for line in board {
            queue!(out,
                cursor::MoveToPreviousLine(1),
                cursor::MoveToColumn(RIGHT_WALL + 3),
                PrintStyledContent(format!("{:#018b}", line).stylize())
            )?;
        };

        queue!(out,
            cursor::MoveToPreviousLine(1),
            cursor::SavePosition
        )?;

        out.flush()?;

        Ok(())
    }
}

