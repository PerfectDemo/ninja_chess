
use std::cmp::{min, max};
use crate::board::Board;
use crate::parser::Command;
use crate::{PIECES, STAGE, COLOUR};
use super::{Operate, Location};
use crate::piece::Piece;


struct RookOperate {}



impl Operate for RookOperate {
    fn final_location(board: &mut Board, command: &Command, colour: &COLOUR) -> Result<(Location, Location), String> {
        let mut x = command.from;
        let mut y = 11_usize;
        
        for (i, piece) in board.boards[x].iter().enumerate() {
            if let PIECES::ROOK = piece.identity {
               if *colour == piece.colour { // can
                   y = i;
               }
            }
        }

        if y == 11_usize {
            return Err("找不到车".to_string());
        }

        let origin = Location {
            x, y
        };

        match command.stage {
            STAGE::FORWARD => {
                if let COLOUR::BLACK = colour {
                    if y >= command.to {
                        y -= command.to;
                    } else {
                        return Err("进超出范围".to_string());
                    }
                } else {
                    y += command.to;
                    if y > 9 {
                        return Err("进超出范围".to_string());
                    }
                }
               
            }, 
            STAGE::BACKWARD => {
                if let COLOUR::BLACK = colour {
                    y += command.to;
                    if y > 9 {
                        return Err("退超出范围".to_string());
                    }
                } else {
                    if y >= command.to {
                        y -= command.to;
                    } else {
                        return Err("退超出范围".to_string());
                    }
                }
              
            },
            STAGE::TRANSECT => x = command.to - 1
        }


        Ok((origin, Location {x, y }))
    }

    fn check(board: &mut Board, from_to: &(Location, Location)) -> bool {
        let ( from, to ) = from_to;

        // 之间是否有其他棋子
        if from.x == to.x {
            let start = min(from.y, to.y);
            let end = max(from.y, to.y);

            for i in (start + 1)..end {
                if let PIECES::EMPTY = board.boards[from.x][i].identity {
                    todo!()
                } else {
                    return false;
                }
            }
        }

        if from.y == to.y {
            let start = min(from.x, to.x);
            let end = max(from.x, to.x);

            for i in (start + 1)..end {
                if let PIECES::EMPTY = board.boards[from.x][i].identity {
                    todo!()
                } else {
                    return false;
                }
            }
        }

        let from_piece = &board.boards[from.x][from.y];
        let to_piece = &board.boards[to.x][to.y];

        // 起点终点是否颜色相同
        if from_piece.colour == to_piece.colour {
            return false;
        }

        return true;
    }
    fn move_location(board: &mut Board, location: &(Location, Location)) {
        let (from, to ) = location;
        board.boards[to.x][to.y] = board.boards[from.x][from.y];
        board.boards[from.x][from.y] = Piece::new(COLOUR::BLACK, PIECES::EMPTY);
    }
    fn after_run(board: &mut Board) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_board() {
        RookOperate::final_location(
            &mut Board::init(), 
            &Command { from: 0, to: 1, stage: STAGE::BACKWARD, identity: PIECES::ROOK },
            &COLOUR::BLACK
        ).unwrap();
    }
}