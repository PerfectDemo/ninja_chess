


use std::cmp::{min, max};
use crate::board::Board;
use crate::parser::Command;
use crate::{PIECES, STAGE, COLOUR};
use super::{Operate, Location};
use crate::piece::Piece;

struct KnightOperate {}


impl Operate for KnightOperate {
    fn final_location(board: &mut Board, command: &Command, colour: &COLOUR) -> Result<(Location, Location), String> {
        let mut from_location;

        for i in 0..10_usize {
            let current: &Piece = &board.boards[command.from][i];
            if let PIECES::KNIGHT = current.identity {
                if &current.colour == colour {
                    from_location = Location {
                        x: command.from,
                        y: i
                    }
                }
            }
        }
        
        todo!()
    }

    fn check(board: &mut Board, location: &(Location, Location)) -> bool {
        todo!()
    }
    fn move_location(board: &mut Board, location: &(Location, Location)) {
        todo!()
    }
    fn after_run(board: &mut Board, location: &(Location, Location)) {
        todo!()
    }

}