//! The common module containing various common stuff used across modules.

use crate::pieces::*;

pub const MAX_LEGAL_MOVES: usize = 64;

/// `x,y` coordinates. `0,0` is interpreted as the **top-left corner**.
/// 
/// a negative x is considered that the piece is dead.
pub type Pos = [i8; 2];
/// Simply an array that has legal moves in it :).
pub type LegalMoves = [Pos; MAX_LEGAL_MOVES];
