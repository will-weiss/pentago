extern crate num;

use std::rc::Rc;
use pentago::game_state::GameState;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub num_quadrants: usize,
    pub quadrant_size: usize
}

impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        // The number of quadrants is 2 raised the number of dimensions.
        let num_quadrants = 2u32.pow(dim as u32) as usize;
        // The size of each each quadrant is the length of each quadrant raised
        // the number of dimensions.
        let quadrant_size = (length as u32).pow(dim as u32) as usize;

        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            num_quadrants: num_quadrants,
            quadrant_size: quadrant_size
        }
    }

    pub fn init_state(&self) -> GameState {
        GameState::new(self.num_quadrants, self.quadrant_size)
    }

    pub fn val(&self, state: GameState) -> BigUint {
        state.val(self.quadrant_size)
    }
}
