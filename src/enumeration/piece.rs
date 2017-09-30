//! Iterators for `Piece`.
//!
//! There are a number of iterators possible.

use std::option::Option;
use std::iter::Iterator;
use super::super::piece::Piece;

/// Returns all `Piece`s, regardless whether they be used in a puzzle.
pub fn all() -> AllPieces {
    AllPieces::new()
}

/// Iterator structure for all pieces.
pub struct AllPieces {
    next_index: Option<u16>,
}

impl AllPieces {
    fn new() -> AllPieces {
        AllPieces { next_index: Some(0) }
    }
}

impl Iterator for AllPieces {
    type Item = Piece;

    fn next(&mut self) -> Option<Piece> {
        match self.next_index {
            Some(index) => {
                self.next_index = if index != <u16>::max_value() {
                    Some(index + 1)
                } else {
                    None
                };

                Some(Piece::from_index(index))
            }

            None => None
        }
    }
}
