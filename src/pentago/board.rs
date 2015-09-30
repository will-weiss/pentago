extern crate num;

use std::rc::{Rc, Weak};
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::quadrant;
use pentago::quadrant::Quadrant;
use pentago::color::Color;
use pentago::game_configuration::GameConfiguration;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Board {
    pub cfg: Rc<GameConfiguration>,
    pub quadrants: Vec<Rc<Quadrant>>
}

impl Board {

    // Generate a new board with some number of quadrants of a given size.
    pub fn new(cfg: Rc<GameConfiguration>) -> Board {
        Board {
            cfg: cfg.clone(),
            quadrants: cfg.quadrants.points.iter().map(|_| {
                Rc::new(Quadrant::new(&cfg)).clone()
            }).collect()
        }
    }

    // Place a stone of a given color at a square given by its indexes.
    pub fn place(&self, q_ix: usize, s_ix: usize, color: &Color) -> Board {
        Board {
            cfg: self.cfg.clone(),
            quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
                if (ix == q_ix) {
                    Rc::new(quadrant.place(s_ix, color))
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

}
