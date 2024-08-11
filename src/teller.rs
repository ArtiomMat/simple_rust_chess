//! Contains all logic to write a story of a piece

use crate::pieces::*;

pub const BAD_TONE: i8 = -1;
pub const NIL_TONE: i8 = 0;
pub const GOOD_TONE: i8 = 1;

/// A teller can be considered a sort of "builder", but is designed with the fact that it will need to be used quite a bit for many pieces and not just for one.
pub struct Teller<'a> {
    pieces: &'a [Piece],
    /// Last tone, used, to add ands or buts. cehck out `*_TONE`
    tone: i8,
    /// Last mark that was put, like `','` or `'.'`. Null terminator(`\0`) means that there wasn't a last mark, this helps identify if there was already a mark put and ignore this new mark call. There is also `'b'` For noting that we just called `begin()`.
    last_mark: u8,
    /// Current index, cached for convinience of calling `sentence/mark()`.
    i: u8,
    /// String where the backstory or info is written. Same as i, for convinience.
    str: String,
}

impl<'a> Teller<'a> {
    pub fn new(pieces: &'a [Piece]) -> Teller {
        Teller {
            pieces,
            tone: 0,
            i: 0,
            last_mark: '.' as u8,
            str: String::new()
        }
    }

    /// Sets up the struct for writing backstory/info for a new piece.
    fn begin(&mut self, i: u8) {
        self.last_mark = b'b';
        self.tone = 0;

        self.i = i;
        
        self.str.clear();
    }

    /// Should be called if a mark symbol is detected.
    /// If `self.last_mark` is a valid mark, then the function does nothing, to allow for more flexibility when constructing sentences.
    /// Also if the function detects that we 
    fn mark(&mut self, m: char) {
        // Ignore multiple marks, and ignore if we just called begin()
        if self.last_mark != b'\0' || self.last_mark == b'b' {
            return;
        }

        self.str.push(m);

        self.last_mark = m as u8;
    }

    
    /// Automatically detects marks, and if it does, then it 
    fn sentence(&mut self, fmt: &str, _tone: i8) {
        let mut last_c = '\0';
        for c in fmt.chars() {
            match c {
                '?' => {
                    // ?? -> ?
                    if last_c == '?' {
                        self.str.push('?');
                    }
                },
                'n' => {

                },
                // '.' | ',' | '!' |
                _ => self.str.push(c)
            }

            last_c = c;
        }
    }

    /// Tells the story into str, using the index `i` from pieces
    /// This should be cached because it's randomized each call.
    pub fn backstory(&mut self, i: u8) -> &String {
        self.begin(i);

        self.sentence("?n the ?s ?c", 0);
        self.mark('.');

        &self.str
    }

    pub fn info(&mut self, i: u8) -> &String {
        self.begin(i);

        self.str.push_str("Todo stuff");
        
        &self.str
    }
}
