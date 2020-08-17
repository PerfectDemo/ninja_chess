
mod piece;
mod board;
mod parser;

pub enum COLOUR {
    RED,
    BLACK,
}


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