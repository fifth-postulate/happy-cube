#![deny(missing_docs)]
//! hcube: A Happy Cube generator and solver.
//!
//! A [Happy Cube](http://www.happy.be) is a foam puzzle. There are actually two
//! objectives
//! 1. Take the six pieces and form them into a cube.
//! 2. Take the six pieces and put them back in there receptacle.
//!
//! hcube offers both style of solvers. Furthermore, it allows one to search the
//! space for other types puzzles.

extern crate bit_field;

pub mod piece;
pub mod enumeration;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true)
    }
}
