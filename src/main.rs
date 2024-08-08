mod common;
mod pieces;
mod board;
mod teller;

use crate::pieces::*;
use crate::common::*;

fn main() {
    Piece::generate_classic();
    println!();
    Piece::generate_classic();
    println!();
    Piece::generate_classic();
    println!();
}
