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

use bit_field::BitField;

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
        Piece::from_index(r1(self.index))
    }

    /// Return a `Piece` which is similar as this piece, but rotated 90 degrees counter-clockwise.
    pub fn rotate_counter_clockwise(&self) -> Piece {
        Piece::from_index(r3(self.index))
    }

    /// Return a `Piece` which is similar as this piece, but flipped.
    pub fn flip(&self) -> Piece {
        Piece::from_index(t(self.index))
    }
}

fn r0(n: u16) -> u16 {
    n
}


fn r1(n: u16) -> u16 {
    let mut result = 0;
    let length = result.bit_length();
    for bit_index in 0..length {
        let bit = n.get_bit(bit_index);
        let target_index = (bit_index + 4) % length;
        result.set_bit(target_index, bit);
    }
    result
}

fn r2(n: u16) -> u16 {
    let mut result = 0;
    let length = result.bit_length();
    for bit_index in 0..length {
        let bit = n.get_bit(bit_index);
        let target_index = (bit_index + 8) % length;
        result.set_bit(target_index, bit);
    }
    result
}


fn r3(n: u16) -> u16 {
    let mut result = 0;
    let length = result.bit_length();
    for bit_index in 0..length {
        let bit = n.get_bit(bit_index);
        let target_index = (bit_index + 12) % length;
        result.set_bit(target_index, bit);
    }
    result
}


fn t(n: u16) -> u16 {
    let mut result = 0;
    let length = result.bit_length();
    for bit_index in 0..length {
        let bit = n.get_bit(bit_index);
        let target_index = (length - bit_index) % length;
        result.set_bit(target_index, bit);
    }
    result
}


fn r1t(n: u16) -> u16 {
    t(r1(n))
}

fn r2t(n: u16) -> u16 {
    t(r2(n))
}

fn r3t(n: u16) -> u16 {
    t(r3(n))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pieces_should_rotate_clockwise() {
        let start = Piece::from_index(0b10);
        let mut transformed = start;

        transformed = transformed.rotate_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000));

        transformed = transformed.rotate_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000_0000));

        transformed = transformed.rotate_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000_0000_0000));

        transformed = transformed.rotate_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10));
    }

    #[test]
    fn pieces_should_rotate_counter_clockwise() {
        let start = Piece::from_index(0b10);
        let mut transformed = start;

        transformed = transformed.rotate_counter_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000_0000_0000));

        transformed = transformed.rotate_counter_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000_0000));

        transformed = transformed.rotate_counter_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10_0000));

        transformed = transformed.rotate_counter_clockwise();
        assert_eq!(transformed, Piece::from_index(0b10));
    }

    #[test]
    fn pieces_should_flip_clockwise() {
        let start = Piece::from_index(0b10);
        let mut transformed = start;

        transformed = transformed.flip();
        assert_eq!(transformed, Piece::from_index(0b1000_0000_0000_0000));

        transformed = transformed.flip();
        assert_eq!(transformed, Piece::from_index(0b10));
    }

}
