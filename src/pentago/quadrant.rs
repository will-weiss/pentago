extern crate num;

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
            squares: (0..cfg.square_cfgs.square_configurations.len()).map(|_| {
                Rc::new(Square::new())
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

    // Place a stone of a given color at a square given by its index.
    pub fn place(&self, square_ix: usize, color: &Color) -> Quadrant {
        Quadrant {
            cfg: self.cfg.clone(),
            squares: self.squares.iter().enumerate().map(|(ix, square)| {
                if (ix == square_ix) {
                    Rc::new(Square::fill(color))
                } else {
                    square.clone()
                }
            }).collect()
        }
    }

    // pub fn rotate(&self, rotation: [usize; 2]) -> Quadrant {
    //     let d_i = rotation[0];
    //     let d_j = rotation[1];
    //     let square_coords = self.cfg.square_coords
    //     Quadrant {
    //         cfg: self.cfg.clone(),
    //         self.squares.iter().enumerate().map(|(ix, square)| {
    //             let color = square.color;
    //             let coords = square_coords[ix];


    //         }).collect()
    //     }
    // }

}

