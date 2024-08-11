//! A board stores the complete state of the game, including the pieces.

use crate::pieces::*;

pub const BOARD_SIZE: usize = 8;

pub struct Cell<'a> {
    /// Stores the piece on a square, if there is one
    pieces: Option<&'a Piece>,
}
