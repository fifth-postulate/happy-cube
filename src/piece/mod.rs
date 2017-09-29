//! The piece module gather functionality around a single piece.
//!
//! A piece is a connected part of the puzzle, forming one of the sides of the
//! cube, or that which will be placed in the receptacle.
//!
//! We will address sub-cubes by index. Use the following chart to relate an
//! index to a sub-cube, and vice versa.
//!
//! ```text
//! +----+----+----+----+----+
//! | 00 | 01 | 02 | 03 | 04 |
//! +----+----+----+----+----+
//! | 15 |              | 05 |
//! +----+              +----+
//! | 14 |              | 06 |
//! +----+              +----+
//! | 13 |              | 07 |
//! +----+----+----+----+----+
//! | 12 | 11 | 10 | 09 | 08 |
//! +----+----+----+----+----+
//! ```
//!
//! This tells us that there are a maximum of `2^16 = 65536` number of pieces.
//! In reality this number is lower due to symmetry. When taking this into
//! account there are only 8420 pieces.
//!
//! Even then, not every piece could be made into a puzzle. For example some of
//! these piece have multiple components.

/// A representation of a piece.
#[derive(PartialEq, Debug)]
pub struct Piece {
    index: u16,
}

impl Piece {
    /// Create a `Piece` from an index. The piece will have a corresponding
    /// sub-cube for each bit set in the `u16` index.
    pub fn from_index(index: u16) -> Piece {
        Piece { index : index }
    }

    /// Return a `Piece` which is similar as this piece, but rotated 90 degrees clockwise.
    pub fn rotate_clockwise(&self) -> Piece {
        Piece::from_index(0b10000)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pieces_should_rotate_clockwise() {
        let start = Piece::from_index(0b10);

        let transformed = start.rotate_clockwise();

        assert_eq!(transformed, Piece::from_index(0b10000))
    }
}