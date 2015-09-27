extern crate num;

use std::rc::Rc;
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
    pub quadrants: Vec<Quadrant>
}

impl Board {
    // Generate a new board with some number of quadrants of a given size.
    pub fn new(cfg: Rc<GameConfiguration>) -> Board {
        Board {
            cfg: cfg.clone(),
            quadrants: (0..cfg.num_quadrants).map(|_| {
                Quadrant::new(&cfg)
            }).collect()
        }
    }

    pub fn place(&self, quadrant_ix: usize, square_ix: usize, color: &Color) -> Board {
        Board {
            cfg: self.cfg.clone(),
            quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
                if (ix == quadrant_ix) {
                    quadrant.place(square_ix, color)
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }




    // pub fn orient(&self, quadrant_ix: usize, top_corner: &Vec<bool>, spin: &Vec<usize>) -> Board {
    //     Board {
    //         dim: self.dim,
    //         length: self.length,
    //         quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
    //             if (ix == quadrant_ix) {
    //                 quadrant.orient(&top_corner, &spin, &self.length)
    //             } else {
    //                 quadrant.clone()
    //             }
    //         }).collect()
    //     }
    // }


}
