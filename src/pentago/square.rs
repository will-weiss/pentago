extern crate num;

use std::rc::Rc;
use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::point::Point;
use pentago::color::Color;
use pentago::color::Color::{Black, White};
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Square {
    pub point: Point,
    pub color: Option<Color>
}

impl Square {
    // Generate a new square with assigned coordinates.
    pub fn new(point: Point) -> Square {
        Square {
            point: point,
            color: None
        }
    }

    pub fn rotate(&self, direction: usize) -> Square {
        Square {
            point: self.point.rotate(direction),
            color: self.color.clone()
        }
    }

    pub fn fill(&self, color: &Color) -> Square {
        Square {
            point: self.point.clone(),
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
            Some(Black) => true,
            _ => false
        }
    }

    pub fn val(&self) -> BigUint {
        match self.color {
            None => BigUint::zero(),
            Some(White) => three_raised_to(self.point.ix),
            Some(Black) => mult2(three_raised_to(self.point.ix))
        }
    }

}
