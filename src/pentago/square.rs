extern crate num;

use std::rc::Rc;
use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::coordinates::Coordinates;
use pentago::color::Color;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Square {
    // pub coordinates: Rc<Coordinates>,
    pub color: Option<Color>
}

impl Square {
    // Generate a new square with assigned coordinates.
    pub fn new() -> Square {
        Square {
            color: None
        }
    }

    pub fn fill(color: &Color) -> Square {
        Square {
            color: Some(color.clone())
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.color {
            None => true,
            _ => false
        }
    }

    pub fn is_black(&self) -> bool {
        match self.color {
            Some(Color::Black) => true,
            _ => false
        }
    }

}
