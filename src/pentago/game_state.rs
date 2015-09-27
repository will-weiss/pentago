extern crate num;

use std::rc::Rc;
use pentago::board::Board;
use pentago::game_configuration::GameConfiguration;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct GameState {
    pub cfg: Rc<GameConfiguration>,
    pub black_to_move: bool,
    pub board: Board
}

impl GameState {

    pub fn new(cfg: Rc<GameConfiguration>) -> GameState {
        GameState {
            cfg: cfg.clone(),
            black_to_move: true,
            board: Board::new(cfg)
        }
    }

    // Calculate the numeric representation of a given game.
    pub fn val(&self) -> BigUint {
        let enumerated_qs = self.board.quadrants.iter().enumerate();
        enumerated_qs.fold(BigUint::zero(), |game_val, (ix, quadrant)| {
            // Get the base value of the quadrant.
            let quadrant_val = quadrant.val();

            if (quadrant_val.is_zero()) {
                // Return the running game value if the quadrant has value zero.
                game_val
            } else if (ix == 0) {
                // If the quadrant index is zero, the quadrant value need not be
                // multiplied, it is simply added to the running game value.
                game_val + quadrant_val
            // If the quadrant index is not zero, the quadrant value must be
            // adjusted according to its index.
            } else {
                // The index of the starting square is the quadrant index
                // multiplied by the size of a quadrant.
                let starting_square_ix = (ix) * (self.cfg.quadrant_size);

                // The quadrant multiplier is 3 raised to the index of the
                // quadrant's starting square.
                let quadrant_multiplier = three_raised_to(starting_square_ix);

                // The quadrant value is multiplied by the multiplier to obtain
                // the quadrant value to be added to the game value.
                game_val + (quadrant_val * quadrant_multiplier)
            }
        })
    }

    // pub fn placements(&self) -> Vec<GameState> {
    //     for quadrant in self.board.quadrants() {

    //     }
    // }

    // pub fn rotations(&self) -> Vec<GameState> {

    // }

}
