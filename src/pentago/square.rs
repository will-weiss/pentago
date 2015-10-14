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
        (cfg.whole_board.iter().enumerate()).zip(Product::new(
            cfg.quadrants.iter().enumerate(),
            cfg.single_quadrant.iter().enumerate()
        )).map(|((b_ix, b_pt), ((q_ix, q_pt), (s_ix, s_pt)))| {

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
