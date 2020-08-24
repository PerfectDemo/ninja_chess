
use crate::piece::*;
use crate::PIECES;
use crate::COLOUR;




pub struct Board<'a> {
    pub boards: Vec<Vec<Piece<'a>>>
}

impl <'a>Board<'a> {
    pub fn init() -> Board<'a> {
        let all = vec![
            vec![
               Piece::new(COLOUR::BLACK, PIECES::ROOK),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::PAWN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::PAWN),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::ROOK),
            ],

            vec![
               Piece::new(COLOUR::BLACK, PIECES::KNIGHT),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::CANNON),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::CANNON),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::KNIGHT),
            ],

            vec![
               Piece::new(COLOUR::BLACK, PIECES::ELEPHANT),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::PAWN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::PAWN),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::ELEPHANT),
            ],

            vec![
               Piece::new(COLOUR::BLACK, PIECES::MANDARIN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::MANDARIN),
            ],

            vec![
               Piece::new(COLOUR::BLACK, PIECES::KING),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::PAWN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::PAWN),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::KING),
            ],

            vec![
                Piece::new(COLOUR::BLACK, PIECES::MANDARIN),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::MANDARIN),
             ],

             vec![
               Piece::new(COLOUR::BLACK, PIECES::ELEPHANT),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::PAWN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::PAWN),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::ELEPHANT),
            ],

            vec![
                Piece::new(COLOUR::BLACK, PIECES::KNIGHT),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::BLACK, PIECES::CANNON),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::BLACK, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::CANNON),
                Piece::new(COLOUR::RED, PIECES::EMPTY),
                Piece::new(COLOUR::RED, PIECES::KNIGHT),
             ],

             vec![
               Piece::new(COLOUR::BLACK, PIECES::ROOK),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::BLACK, PIECES::PAWN),
               Piece::new(COLOUR::BLACK, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::PAWN),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::EMPTY),
               Piece::new(COLOUR::RED, PIECES::ROOK),
            ],

        ];

        Board {
            boards: all
        }
    }

    fn print(&self) {
       for i in 0..10 {
           for j in 0..9 {
               self.boards[j][i].show();
           }
           println!("");
       }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_board() {
        let board = Board::init();
        board.print();
    }
}

