extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::color;
use pentago::color::Color;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Square {
    pub color: Option<Color>
}

impl Square {
    // Generate a new square with assigned coordinates.
    pub fn new() -> Square {
        Square {
            color: Some(Color::Black)
        }
    }

    pub fn fill(color: &Color) -> Square {
        Square {
            color: Some(color.clone())
        }
    }
}
