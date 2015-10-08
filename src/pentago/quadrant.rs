extern crate num;
extern crate itertools;

use self::itertools::Product;
use std::rc::Rc;
use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::game_configuration::GameConfiguration;
use pentago::square::Square;
use pentago::color::Color;
use pentago::color::Color::{Black, White};
use pentago::math_utils::{three_raised_to, mult2, mult3, factorial};


#[derive(Debug, Clone)]
pub struct Quadrant {
    pub squares: Vec<Rc<Square>>
}

impl Quadrant {
    // Generate a new quadrant with some number of squares.
    pub fn new(cfg: &Rc<GameConfiguration>) -> Quadrant {
        Quadrant {
            squares: cfg.squares.iter().map(|point| {
                Rc::new(Square::new(point.clone()))
            }).collect()
        }
    }

    // Get a (big) integer representing the value of this square.
    pub fn val(&self) -> BigUint {
        self.squares.iter().fold(BigUint::zero(), |val, square| {
            val + square.val()
        })
    }

    // Place a stone of a given color at a square given by its index.
    pub fn place(&self, square_ix: usize, color: &Color) -> Quadrant {
        Quadrant {
            squares: self.squares.iter().enumerate().map(|(ix, square)| {
                if (ix == square_ix) {
                    Rc::new(square.place(&color))
                } else {
                    square.clone()
                }
            }).collect()
        }
    }

    pub fn rotate(&self, direction: usize) -> Quadrant {
        Quadrant {
            squares: (&self.squares).iter().map(|square| {
                Rc::new(Square {
                    point: square.point.clone(),
                    color: self.squares[square.point.rotate(direction)].color.clone()
                })
            }).collect()
        }
    }

}

