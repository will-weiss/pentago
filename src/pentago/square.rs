extern crate num;
extern crate itertools;

use self::num::bigint::BigUint;
use self::itertools::Product;

use pentago::math_utils::{three_raised_to, mult2};
use pentago::configuration::Configuration;


#[derive(Debug, Clone)]
pub struct Square {
    pub b_ix: usize,
    pub q_ix: usize,
    pub s_ix: usize,
    pub if_white: BigUint,
    pub if_black: BigUint,
}

pub type Squares = Vec<Square>;

impl Square {

    pub fn all(cfg: &Configuration) -> Squares {
        (0..cfg.whole_board.len()).zip(Product::new(
            0..cfg.quadrants.len(),
            0..cfg.single_quadrant.len()
        )).map(|(b_ix, (q_ix, s_ix))| {

            let if_white = three_raised_to(b_ix);
            let if_black = mult2(if_white.clone());

            Square {
                b_ix: b_ix,
                q_ix: q_ix,
                s_ix: s_ix,
                if_white: if_white,
                if_black: if_black
            }
        }).collect()
    }

}
