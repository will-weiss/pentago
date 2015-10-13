extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use self::itertools::Product;

use pentago::game_state::GameState;
use pentago::lattice::Lattice;
use pentago::board::Board;
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color::{Black, White};

#[derive(Debug, Clone)]
pub struct Point {
    pub pt_ix: usize,
    pub q_ix: usize,
    pub s_ix: usize,
    pub if_white: BigUint,
    pub if_black: BigUint
}

impl Point {
    pub fn gen(num_q: usize, num_s: usize) -> Vec<Point> {
        Product::new(0..num_q, 0..num_s)
            .enumerate()
            .map(|(pt_ix, (q_ix, s_ix))| {
                let if_white = three_raised_to(pt_ix);
                let if_black = mult2(if_white.clone());
                Point {
                    pt_ix: pt_ix,
                    q_ix: q_ix,
                    s_ix: s_ix,
                    if_white: if_white,
                    if_black: if_black,
                }
            }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub squares: Lattice,
    pub quadrants: Lattice,
    pub points: Vec<Point>,
}

impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        let squares = Lattice::new(dim, length);
        let quadrants = Lattice::new(dim, 2);
        let points = Point::gen(quadrants.len(), squares.len());

        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            squares: squares,
            quadrants: quadrants,
            points: points,
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

    // Calculate the numeric representation of a given board.
    pub fn val(&self, board: &Board) -> BigUint {
        (self.points).iter().fold(BigUint::zero(), |val, pt| {
            let square = board.quadrants[pt.q_ix].squares[pt.s_ix];
            match square {
                None => val,
                Some(White) => val + &pt.if_white,
                Some(Black) => val + &pt.if_black
            }
        })
    }

}
