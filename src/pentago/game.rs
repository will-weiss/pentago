extern crate num;

use std::rc::Rc;
use pentago::board;
use pentago::board::Board;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;

#[derive(Debug, Clone)]
pub struct Game {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub num_quadrants: usize,
    pub quadrant_size: usize,
    pub board: Board
}

impl Game {
    pub fn new(dim: usize, length: usize, victory: usize) -> Game {
        let num_quadrants = 2u32.pow(dim as u32) as usize;
        let quadrant_size = (length as u32).pow(dim as u32) as usize;

        Game {
            dim: dim,
            length: length,
            victory: victory,
            num_quadrants: num_quadrants,
            quadrant_size: quadrant_size,
            board: Board::new(num_quadrants, quadrant_size)
        }
    }

    pub fn val(&self) -> BigUint {
        self.board.val(&self.quadrant_size)
    }

}
