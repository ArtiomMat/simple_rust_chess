//! Defines various piece specific logic, the home of the Piece struct.

use rand::{self, Rng, RngCore};
use std::{any::Any, fmt, ops::{Range, RangeInclusive}};

use crate::common::*;

const CLASSIC_PIECES_N: usize = 32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Black,
    White,
}
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Side::Black => "black",
            Side::White => "white",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Class {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}
impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Class::Pawn => "pawn",
            Class::Knight => "knight",
            Class::Rook => "rook",
            Class::Bishop => "bishop",
            Class::Queen => "Queen",
            Class::King => "King",
        };
        write!(f, "{}", s)
    }
}

const VOWELS: [&str; 13] = [
    "a", "aa", "ae", "oe", "e", "ea", "i", "ie", "o", "oi", "ou", "u", "ui",
];
const CONSONANTS: [&str; 31] = [
    "b", "c", "d", "f", "ff", "g", "cg", "h", "j", "k", "l", "ll", "m", "n", "nn", "p", "pp", "q",
    "r", "s", "ss", "t", "v", "w", "x", "y", "vy", "dy", "by", "ly", "z",
];

/// The moment the other side is winning by a high margin, will switch sides.\
/// Incompatible with `King` class of course.
const NAT_TRAITOR: u32 = 0b1;
/// Naturally fearful, fear is doubled.\
/// Incompatible with `NAT_WARRIOR`
const NAT_COWARD: u32 = 0b10;
/// Naturally fearful, fear is halved.
/// Incompatible with `NAT_COWARD`
const NAT_WARRIOR: u32 = 0b100;

pub struct Piece {
    class: Class,
    side: Side,
    sex: Sex,
    pos: Pos,
    /// A bitmask of various constant booleanic mindsets and states, check out `NAT_` constants.
    nature: u32,

    name: String,
    /// The index of the friend within an array of other pieces. `-1` for no friend. Index used because Rust has a period when we use circular references.
    friend: i8,
    /// The current piece that this one will eat the moment it gets the chance. Obviously everyone is an enemy but this one is especially hated, for instance if it eats a friend.
    enemy: i8,
    /// Fear of advancing, if high enough will even retreat.\
    /// Raw fear is the fear before calculations.
    raw_fear: f32,
}

/// Locates a friendless piece within the range, will not return a king or a queen.\
/// EXCEPT `except`, this index will not be returned as well, can be set to a big value and ignored.
fn find_friendless_except(pieces: &[Piece; CLASSIC_PIECES_N], range: &RangeInclusive<usize>, except: usize) -> Option<usize> {
    let mut rng = rand::thread_rng();
    // I know 5 is arbitrary but it makes sense here so shut up
    for _ in 0..=5 {
        let i = rng.gen_range(range.clone());

        if pieces[i].friend == -1
            && pieces[i].class != Class::King
            && pieces[i].class != Class::Queen
            && i != except
        {
            return Some(i);
        }
    }

    None
}

fn find_friendless(pieces: &[Piece; CLASSIC_PIECES_N], range: &RangeInclusive<usize>) -> Option<usize> {
    find_friendless_except(pieces, range, usize::MAX)
}

fn find_friendless_pair(pieces: &[Piece; CLASSIC_PIECES_N], range: &RangeInclusive<usize>) -> Option<[usize; 2]> {
    find_friendless(pieces, range).and_then(|a| {
        find_friendless_except(pieces, range, a).map(|b| [a, b])
    })
}

impl Piece {
    pub fn get_class(&self) -> Class {
        Class::Pawn
    }

    /// Generates the classic set of pieces, utilizes regular `new()`.
    ///
    /// The array is flat, it has an order that just makes it easier to add friends.
    pub fn generate_classic() -> [Piece; CLASSIC_PIECES_N] {
        let mut pieces = [
            Piece::new(Class::Rook, Side::Black, &[0, 0]),
            Piece::new(Class::Knight, Side::Black, &[1, 0]),
            Piece::new(Class::Bishop, Side::Black, &[2, 0]),
            Piece::new(Class::Queen, Side::Black, &[3, 0]),
            Piece::new(Class::King, Side::Black, &[4, 0]),
            Piece::new(Class::Bishop, Side::Black, &[5, 0]),
            Piece::new(Class::Knight, Side::Black, &[6, 0]),
            Piece::new(Class::Rook, Side::Black, &[7, 0]),
            ////////////////////////////////////////////////////
            Piece::new(Class::Pawn, Side::Black, &[0, 1]),
            Piece::new(Class::Pawn, Side::Black, &[1, 1]),
            Piece::new(Class::Pawn, Side::Black, &[2, 1]),
            Piece::new(Class::Pawn, Side::Black, &[3, 1]),
            Piece::new(Class::Pawn, Side::Black, &[4, 1]),
            Piece::new(Class::Pawn, Side::Black, &[5, 1]),
            Piece::new(Class::Pawn, Side::Black, &[6, 1]),
            Piece::new(Class::Pawn, Side::Black, &[7, 1]),
            ////////////////////////////////////////////////////
            Piece::new(Class::Pawn, Side::White, &[0, 6]),
            Piece::new(Class::Pawn, Side::White, &[1, 6]),
            Piece::new(Class::Pawn, Side::White, &[2, 6]),
            Piece::new(Class::Pawn, Side::White, &[3, 6]),
            Piece::new(Class::Pawn, Side::White, &[4, 6]),
            Piece::new(Class::Pawn, Side::White, &[5, 6]),
            Piece::new(Class::Pawn, Side::White, &[6, 6]),
            Piece::new(Class::Pawn, Side::White, &[7, 6]),
            ////////////////////////////////////////////////////
            Piece::new(Class::Rook, Side::White, &[0, 6]),
            Piece::new(Class::Knight, Side::White, &[1, 7]),
            Piece::new(Class::Bishop, Side::White, &[2, 7]),
            Piece::new(Class::Queen, Side::White, &[3, 7]),
            Piece::new(Class::King, Side::White, &[4, 7]),
            Piece::new(Class::Bishop, Side::White, &[5, 7]),
            Piece::new(Class::Knight, Side::White, &[6, 7]),
            Piece::new(Class::Rook, Side::White, &[7, 7]),
        ];

        // Add a friend
        let mut rng = rand::thread_rng();

        // Same friendship generation for both sides
        for side_i in 0..=1 {
            let pawns_start = 8 + (side_i as usize) * 8; // Pawns start index for this side
            let specs_start = (side_i as usize) * 24; // Special pieces start index

            // The pawn friendships that are to be generated, at most 4.
            for i in 0..rng.gen_range(2..=4) {
                let friendless = find_friendless_pair(&pieces, &(pawns_start..=(pawns_start+7)));
                if let Some(pair) = friendless {
                    pieces[pair[0]].friend = pair[1] as i8;
                    pieces[pair[1]].friend = pair[0] as i8;
                }
            }

            // The specs friendships, at most 3, which would mean everyone except the king and queen, they have no friends!
            // Duplicate code...
            for i in 0..rng.gen_range(0..=3) {
                let friendless = find_friendless_pair(&pieces, &(specs_start..=(specs_start+7)));
                if let Some(pair) = friendless {
                    pieces[pair[0]].friend = pair[1] as i8;
                    pieces[pair[1]].friend = pair[0] as i8;
                    println!("Friends: {}({:?} {:?}) + {}({:?} {:?})", pieces[pair[0]].name, pieces[pair[0]].side, pieces[pair[0]].class, pieces[pair[1]].name, pieces[pair[1]].side, pieces[pair[1]].class);
                }
            }
        }

        pieces
    }

    pub fn new(class: Class, side: Side, pos: &Pos) -> Piece {
        let mut rng = rand::thread_rng();

        let mut name = String::new();
        let mut last_was_vowel = rand::random();
        let name_len = rng.gen_range(3..=8);
        for _ in 0..name_len {
            if last_was_vowel {
                name.push_str(CONSONANTS[rng.gen_range(0..CONSONANTS.len())]);
            } else {
                name.push_str(VOWELS[rng.gen_range(0..VOWELS.len())]);
            }
            last_was_vowel = !last_was_vowel;
        }
        if let Some(first_char) = name.get(0..1) {
            name.replace_range(0..1, &(first_char.to_uppercase()));
        }

        let sex = match class {
            Class::King => Sex::Male,
            Class::Queen => Sex::Female,
            _ => {
                if rand::random() {
                    Sex::Male
                } else {
                    Sex::Female
                }
            },
        };

        let nature: u32 = 0;

        Piece {
            class,
            side,
            sex,
            nature,
            name,
            pos: *pos,
            raw_fear: 0.0,
            enemy: -1, // No enemies at the start
            friend: -1, // Handled when actually initializing the entire piece set
        }
    }

    /// Finds a friend in a classic pieces array, but may also not find a friend.
    /// `i` is the piece's own index.
    /// mut rng = rand::thread_rng();

    //     // May not have a friend :(
    //     if self.class == Class::King
    //         || self.class == Class::Queen
    //         || (0..=3).contains(&rng.gen_range(0..=10))
    //     {
    //         return;
    //     }

    //     // If a pawn then a special piece as a friend, and vice versa. Because it's not common for a pawn to have a fucking bishop as a friend is it?
    //     let special_friend = (0..=3).contains(&rng.gen_range(0..=10));

    //     if self.class == Class::Pawn {
    //         let 
    //     } else {
            
    //     }
    // }

    fn legal_moves(&self, legals: &mut LegalMoves) -> usize {
        0
    }

    pub fn is_legal_move(&self, pos: &Pos) -> bool {
        false
    }

    pub fn is_dead(&self) -> bool {
        self.pos[0] < 0
    }

    pub fn try_move(&self, pos: &Pos) -> bool {
        self.is_legal_move(pos)
    }
}
