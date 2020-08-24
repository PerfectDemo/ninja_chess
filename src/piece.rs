use crate::COLOUR;
use crate::PIECES;
use colored::Colorize;

pub trait Show {
    fn show(&self);
}


#[derive(Copy, Clone)]
pub struct Piece<'a> {
    pub colour: COLOUR,
    pub identity: PIECES,
    pub title: &'a str
}

impl <'a> Piece<'a> {
    pub fn new(colour: COLOUR, identity: PIECES) -> Piece<'a> {
        let title;
        match colour {
            COLOUR::RED => {
                match identity {
                    PIECES::CANNON => title = "炮",
                    PIECES::ROOK => title = "俥",
                    PIECES::KNIGHT => title = "马",
                    PIECES::KING => title = "帅",
                    PIECES::MANDARIN => title = "仕",
                    PIECES::ELEPHANT => title = "相",
                    PIECES::PAWN => title = "兵",
                    PIECES::EMPTY => title = "・",
                }
            },
            COLOUR::BLACK => {
                match identity {
                    PIECES::CANNON => title = "炮",
                    PIECES::ROOK => title = "车",
                    PIECES::KNIGHT => title = "馬",
                    PIECES::KING => title = "将",
                    PIECES::MANDARIN => title = "士",
                    PIECES::ELEPHANT => title = "象",
                    PIECES::PAWN => title = "卒",
                    PIECES::EMPTY => title = "・",
                }
            },
        }

        Piece {
            colour,
            identity,
            title
        }
    }
}

impl <'a>Show for Piece<'a> {
    fn show(&self) {
        if let PIECES::EMPTY = self.identity {
            print!("{}", self.title);
        } else {
            match self.colour {
                COLOUR::BLACK => print!("{}", self.title.black()),
                COLOUR::RED => print!("{}", self.title.red()),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show() {
        let rook = Piece::new(COLOUR::RED, PIECES::ROOK);
        rook.show();
        let empty = Piece::new(COLOUR::RED, PIECES::EMPTY);
        empty.show();
    }
}
