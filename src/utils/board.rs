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
}

const LEFT_WALL: u16 = 0;
const RIGHT_WALL: u16 = BOARD_WIDTH * 2 + 1 + LEFT_WALL;

impl BoardSimple {
    pub fn display<W>(&self, write: &mut W) -> std::io::Result<()> 
    where W: std::io::Write {
        let board = self.board;

        queue!(write, 
            cursor::RestorePosition,
            terminal::Clear(FromCursorDown),
        )?;

        queue!(write, PrintStyledContent("▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁".white()))?;
        for _i in 0..BOARD_HIGHT {
            queue!(write,
                cursor::MoveDown(1),
                cursor::MoveToColumn(LEFT_WALL),
                PrintStyledContent("▎".white()),
                cursor::MoveToColumn(RIGHT_WALL),
                PrintStyledContent("🮇".white())
            )?;
        }
        queue!(write,
            cursor::MoveDown(1),
            cursor::MoveToColumn(LEFT_WALL),
            PrintStyledContent( "▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔".white()),
            cursor::SavePosition,
        )?;

        for line in board {
            queue!(write,
                cursor::MoveUp(1),
                cursor::MoveToColumn(1),
            )?;
            for i in 0..BOARD_WIDTH {
                if(line >> (12 - i)) & 0b1 == 0 {
                    queue!(write, PrintStyledContent("  ".dark_red()))?;
                } else {
                    queue!(write, PrintStyledContent("██".dark_red()))?;
                }
            }
        };

        queue!(write, cursor::RestorePosition)?;

        for line in board {
            queue!(write,
                cursor::MoveToPreviousLine(1),
                cursor::MoveToColumn(RIGHT_WALL + 3),
                PrintStyledContent(format!("{:#018b}", line).stylize())
            )?;
        };

        queue!(write,
            cursor::MoveToPreviousLine(1),
            cursor::SavePosition
        )?;

        write.flush()?;

        Ok(())
    }
}

