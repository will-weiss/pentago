extern crate num;
extern crate itertools;

use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use self::itertools::Product;

use pentago::configuration::Configuration;
use pentago::lattice::{Lattice, Point, Coordinates};

use pentago::math_utils::{three_raised_to, mult2, mult3};


#[derive(Debug, Clone)]
pub struct Square {
    pub b_pt: Point,
    pub q_pt: Point,
    pub s_pt: Point,

    pub if_white: BigUint,
    pub if_black: BigUint,
}

pub type Squares = Vec<Square>;
