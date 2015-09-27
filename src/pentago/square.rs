extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::color;
use pentago::color::Color;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Square {
    pub color: Color
}

impl Square {
    // Generate a new square with assigned coordinates.
    pub fn new() -> Square {
        Square {
            color: Color::Black
        }
    }
}
