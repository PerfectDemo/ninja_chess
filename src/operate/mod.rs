
use super::board::Board;
use super::parser::Command;
use super::COLOUR;


mod rook;
mod knight;

pub struct Location {
    x: usize,
    y: usize,
}

pub trait Operate {
    fn final_location(board: &mut Board, command: &Command, colour: &COLOUR) -> Result<(Location, Location), String>;
    fn check(board: &mut Board, location: &(Location, Location)) -> bool;
    fn move_location(board: &mut Board, location: &(Location, Location));
    fn after_run(board: &mut Board, location: &(Location, Location));

}

pub fn run() {

}