extern crate num;

use std::rc::Rc;
use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::game_configuration::GameConfiguration;
use pentago::square::Square;
use pentago::color::Color;
use pentago::color::Color::{Black, White};
use pentago::math_utils::{three_raised_to, mult2, mult3};


#[derive(Debug, Clone)]
pub struct Quadrant {
    pub squares: Vec<Square>
}

impl Quadrant {
    // Generate a new quadrant with some number of squares.
    pub fn new(cfg: &Rc<GameConfiguration>) -> Quadrant {
        Quadrant {
            squares: (0..cfg.quadrant_size).map(|_| {
                Square::new()
            }).collect()
        }
    }

    // Get a (big) integer representing the value of this square.
    pub fn val(&self) -> BigUint {
        self.squares.iter().enumerate().fold(BigUint::zero(), |val, (ix, square)| {
            match square.color {
                None => val,
                Some(White) => val + three_raised_to(ix),
                Some(Black) => val + mult2(three_raised_to(ix))
            }
        })
    }

    pub fn place(&self, square_ix: usize, color: &Color) -> Quadrant {
        Quadrant {
            squares: self.squares.iter().enumerate().map(|(ix, square)| {
                if (ix == square_ix) {
                    Square::fill(color)
                } else {
                    square.clone()
                }
            }).collect()
        }
    }

}

