extern crate num;

use std::rc::Rc;
use pentago::board;
use pentago::board::Board;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Game {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub num_quadrants: usize,
    pub quadrant_size: usize,
    pub board: Board
}

impl Game {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> Game {
        // The number of quadrants is 2 raised the number of dimensions.
        let num_quadrants = 2u32.pow(dim as u32) as usize;
        // The size of each each quadrant is the length of each quadrant raised
        // the number of dimensions.
        let quadrant_size = (length as u32).pow(dim as u32) as usize;

        Game {
            dim: dim,
            length: length,
            victory: victory,
            num_quadrants: num_quadrants,
            quadrant_size: quadrant_size,
            board: Board::new(num_quadrants, quadrant_size)
        }
    }

    // Calculate the numeric representation of a given game.
    pub fn val(&self) -> BigUint {

        // Keep a tally of the game's value.
        let mut game_val = BigUint::zero();

        // Loop over the quadrants and their indexes.
        for (ix, quadrant) in &self.board.quadrants {

            // Get the base value of the quadrant.
            let mut quadrant_val = quadrant.val();

            // No further calculations are necessary if the base value of the
            // quadrant is zero.
            if (quadrant_val.is_zero()) {
                continue;
            }

            // If the quadrant index is not zero, the quadrant value must be
            // adjusted according to its index.
            if (ix != &0) {

                // The index of the starting square is the quadrant index
                // multiplied by the size of a quadrant.
                let starting_square_ix = (*ix) * (self.quadrant_size);

                // The quadrant multiplier is 3 raised to the index of the
                // quadrant's starting square.
                let quadrant_multiplier = three_raised_to(starting_square_ix);

                // The quadrant value is multiplied by the multiplier to obtain
                // the quadrant value to be added to the game value.
                quadrant_val = quadrant_val * quadrant_multiplier;
            }

            // Update the game value by adding the quadrant value.
            game_val = game_val + quadrant_val;
        }

        // Return the game value.
        game_val
    }

}
