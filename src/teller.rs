//! Contains all logic to write a story of a piece

use crate::pieces::*;

pub const BAD_TONE: i8 = -1;
pub const NIL_TONE: i8 = 0;
pub const GOOD_TONE: i8 = 1;

/// A teller can be considered a sort of "builder", but is designed with the fact that it will need to be used quite a bit for many pieces and not just for one.
pub struct Teller<'a> {
    pieces: &'a [Piece],
    /// Last tone, used, to add ands or buts.
    tone: i8,
    /// Last mark that was put, like ',' or '.'.
    last_mark: u8,
}

impl<'a> Teller<'a> {
    pub fn new(pieces: &'a [Piece]) -> Teller {
        Teller {
            pieces,
            tone: 0,
            last_mark: '.' as u8
        }
    }

    /// Tells the story into str, using the index `i` from pieces
    /// This should be cached because it's randomized each call.
    pub fn backstory(&mut self, i: usize, str: &mut String) {
        self.tone = 0;
        self.last_mark = '.' as u8;

        str.clear();
        str.push_str("Todo backstory");
    }

    pub fn info(&mut self, i: usize, str: &mut String) {
        self.tone = 0;
        self.last_mark = '.' as u8;
        
        str.clear();
        str.push_str("Todo stuff");
    }
}
