extern crate num;

use std::rc::Rc;
use pentago::quadrant::Quadrant;
use pentago::color::Color;
use pentago::game_configuration::GameConfiguration;

#[derive(Debug, Clone)]
pub struct Board {
    pub quadrants: Vec<Rc<Quadrant>>
}

impl Board {
    // Generate a new board with some number of quadrants of a given size.
    pub fn new(cfg: Rc<GameConfiguration>) -> Board {
        Board {
            quadrants: (0..cfg.quadrants.len()).map(|_| {
                Rc::new(Quadrant::new(cfg.clone()))
            }).collect()
        }
    }

    // Place a stone of a given color at a square given by its indexes.
    pub fn place(&self, q_ix: usize, s_ix: usize, color: Color) -> Board {
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

}
