extern crate num;

use std::rc::Rc;
use pentago::board::Board;
use pentago::color::Color;
use pentago::color::Color::{Black, White};
use pentago::game_configuration::GameConfiguration;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub enum GameResult {
    BlackWin,
    WhiteWin
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub cfg: Rc<GameConfiguration>,
    pub black_to_move: bool,
    pub result: Option<GameResult>,
    pub board: Board
}

impl GameState {

    pub fn new(cfg: Rc<GameConfiguration>) -> GameState {
        GameState {
            cfg: cfg.clone(),
            black_to_move: true,
            result: None,
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
                let starting_square_ix = (ix) * (self.cfg.squares.len());

                // The quadrant multiplier is 3 raised to the index of the
                // quadrant's starting square.
                let quadrant_multiplier = three_raised_to(starting_square_ix);

                // The quadrant value is multiplied by the multiplier to obtain
                // the quadrant value to be added to the game value.
                game_val + (quadrant_val * quadrant_multiplier)
            }
        })
    }

    pub fn to_move(&self) -> Color {
        match self.black_to_move {
            true => Black,
            false => White
        }
    }

    pub fn place(&self, q_ix: usize, s_ix: usize, color: &Color) -> GameState {
        GameState {
            cfg: self.cfg.clone(),
            black_to_move: !self.black_to_move,
            result: None,
            board: self.board.place(q_ix, s_ix, color)
        }
    }

    pub fn possible_placements(&self) -> Vec<GameState> {
        let color = self.to_move();
        let mut placement_states = vec![];

        for (q_ix, q) in self.board.quadrants.iter().enumerate() {
            for (s_ix, s) in q.squares.iter().enumerate() {
                if s.is_empty() {
                    placement_states.push(self.place(q_ix, s_ix, &color));
                }
            }
        }

        placement_states

    }

    pub fn rotate(&self, q_ix: usize, direction: usize) -> GameState {
        GameState {
            cfg: self.cfg.clone(),
            black_to_move: self.black_to_move,
            result: None,
            board: self.board.rotate(q_ix, direction)
        }
    }

    pub fn rotate_whole(&self, direction: usize) -> GameState {
        GameState {
            cfg: self.cfg.clone(),
            black_to_move: self.black_to_move,
            result: None,
            board: self.board.rotate_whole(direction)
        }
    }

    // pub fn min_representation(&self) -> GameState {
    //     let states = self.cfg.q_lattice.rotations.iter.enumerate()
    //         .map(|(direction, _)| {
    //             self.rotate_whole(direction)
    //         });




    // }

}
