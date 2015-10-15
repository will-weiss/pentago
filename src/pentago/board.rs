extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::bigint::BigUint;
use self::itertools::Product;
use pentago::math_utils::{three_raised_to, mult2};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White
}

pub type Space = Option<Color>;
pub type Quadrant = Vec<Space>;
pub type QuadrantRef = Rc<Quadrant>;
pub type Board = Vec<QuadrantRef>;
pub type SquareIxs = (usize, usize);
pub type Line = Vec<SquareIxs>;

#[derive(Debug, Clone)]
pub struct Square {
    pub ixs: SquareIxs,
    pub if_white: BigUint,
    pub if_black: BigUint,
}

pub type Squares = Vec<Square>;

impl Square {

    pub fn all(num_qs: usize, num_sq: usize) -> Squares {
        Product::new(0..num_qs, 0..num_sq).enumerate().map(|(b_ix, ixs)| {
            Square {
                ixs: ixs,
                if_white: three_raised_to(b_ix),
                if_black: mult2(three_raised_to(b_ix))
            }
        }).collect()
    }

    pub fn of(&self, board: &Board) -> Space {
        let (q_ix, s_ix) = self.ixs;
        board[q_ix][s_ix]
    }

}

fn init_quadrant(sq_len: usize) -> Quadrant {
    vec![None; sq_len]
}

pub fn init(qs_len: usize, sq_len: usize) -> Board {
    vec![Rc::new(init_quadrant(sq_len)); qs_len]
}
