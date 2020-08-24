
mod piece;
mod board;
mod parser;
mod operate;

#[derive(Copy, Clone)]
pub enum COLOUR {
    RED,
    BLACK,
}



impl PartialEq for COLOUR {
    fn eq(&self, other: &Self) -> bool {
        let left;
        let right;
        match self {
            COLOUR::RED => { left = true; },
            COLOUR::BLACK => { left = false; }
        }

        match other {
            COLOUR::RED => { right = true; },
            COLOUR::BLACK => { right = false; }
        }

        if left == right {
            true
        } else {
            false
        }
    }  
}

#[derive(Copy, Clone)]
pub enum PIECES {
    ROOK,
    KNIGHT,
    CANNON,
    PAWN,

    KING,
    MANDARIN,
    ELEPHANT,

    EMPTY,
}

pub enum STAGE {
    FORWARD,
    BACKWARD,
    TRANSECT,
}