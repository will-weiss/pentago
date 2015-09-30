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
    pub cfg: Rc<GameConfiguration>,
    pub squares: Vec<Rc<Square>>
}

impl Quadrant {
    // Generate a new quadrant with some number of squares.
    pub fn new(cfg: &Rc<GameConfiguration>) -> Quadrant {
        Quadrant {
            cfg: cfg.clone(),
            squares: cfg.squares.points.iter().map(|point| {
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
            cfg: self.cfg.clone(),
            squares: self.squares.iter().enumerate().map(|(ix, square)| {
                if (ix == square_ix) {
                    Rc::new(square.fill(&color))
                } else {
                    square.clone()
                }
            }).collect()
        }
    }

    pub fn rotate(&self, rotation: usize) -> Quadrant {
        let q_cfg = self.cfg.quadrants;

        let rotated_square_indexes = q_cfg.rotations[rotation];

        for (ix, square) in Product::new(rotated_square_indexes.iter(), (&self.squares).iter()) {

        }

        Quadrant {
            cfg: self.cfg.clone(),
            squares:
        }


    }

}

