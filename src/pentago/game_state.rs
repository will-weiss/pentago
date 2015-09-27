extern crate num;

use std::rc::Rc;
use pentago::board::Board;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct GameState {
    pub black_to_move: bool,
    pub board: Board
}

impl GameState {

    pub fn new(num_quadrants: usize, quadrant_size: usize) -> GameState {
        GameState {
            black_to_move: true,
            board: Board::new(num_quadrants, quadrant_size)
        }
    }

    // Calculate the numeric representation of a given game.
    pub fn val(&self, quadrant_size: usize) -> BigUint {

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
                let starting_square_ix = (*ix) * (quadrant_size);

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
