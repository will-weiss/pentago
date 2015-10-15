extern crate num;
extern crate itertools;

use self::num::bigint::BigUint;
use self::itertools::Product;

use pentago::math_utils::{three_raised_to, mult2};
use pentago::configuration::Configuration;
use pentago::board::{Board, Space};

pub type SquareIxs = (usize, usize);

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
