extern crate num;

use std::collections::VecDeque;
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
    pub quadrants: VecDeque<Rc<Quadrant>>
}

impl Board {

    // Generate a new board with some number of quadrants of a given size.
    pub fn new(cfg: Rc<GameConfiguration>) -> Board {
        let num_q = (&cfg.quadrants).len();
        let mut quadrants = VecDeque::with_capacity(num_q);

        for _ in 0..num_q {
            quadrants.push_back(Rc::new(Quadrant::new(&cfg)).clone());
        }

        Board {
            quadrants: quadrants
        }
    }

    // Place a stone of a given color at a square given by its indexes.
    pub fn place(&self, q_ix: usize, s_ix: usize, color: &Color) -> Board {
        Board {
            quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
                if (ix == q_ix) {
                    Rc::new(quadrant.place(s_ix, color))
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

    pub fn rotate(&self, q_ix: usize, direction: usize) -> Board {
        Board {
            quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
                if (ix == q_ix) {
                    Rc::new(quadrant.rotate(direction))
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

    pub fn rotate_whole(&self, direction: usize) -> Board {
        Board {
            quadrants: self.quadrants.iter().map(|quadrant| {
                Rc::new(quadrant.rotate(direction))
            }).collect()
        }
    }


}
