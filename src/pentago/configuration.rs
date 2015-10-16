extern crate num;
extern crate itertools;

use std::rc::Rc;
use std::ops::Range;
use self::itertools::Product;
use self::num::bigint::BigUint;
use self::num::traits::Zero;

use pentago::square::{Square, Line};
use pentago::board::{Board, Color};
use pentago::board::Color::{White, Black};
use pentago::lattice::Lattice;
use pentago::state::State;


const QUADRANT_LENGTH: usize = 2;


#[derive(Debug, Clone)]
pub struct Configuration {
    dim: usize,
    quadrant_length: usize,
    radius: usize,
    diameter: usize,
    num_quadrants: usize,
    squares_per: usize,
    victory: usize,
    spinner: Spinner,
    rotation_planes: RotationPlanes,
    rotation_dirs: RotationDirs,
    spins: Vec<Vec<usize>>,
    quadrant_rotations: Vec<(usize, usize)>,
    squares: Line,
    square_ixs_by_quadrant: Vec<Vec<usize>>,
    quadrant_sq_rotations: Vec<Vec<usize>>,
    board_sq_rotations: Vec<Vec<usize>>,
    all_lines: Vec<Line>,
    lines_by_ix: Vec<Vec<Line>>,
}

impl Configuration {

    // Create a new Game of a given dimension, quadrant radius, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, radius: usize, victory: usize) -> Configuration {

        let board_cfg = BoardCfg::new(dim, radius);

        let spinner = Spinner::new(board_cfg);

        Configuration {
            squares: vec![],
            all_lines: vec![],
            dim: dim,
            quadrant_length: QUADRANT_LENGTH,
            radius: radius,
            diameter: radius * QUADRANT_LENGTH,
            num_quadrants: (2usize).pow(dim as u32),
            squares_per: (radius).pow(dim as u32),
            victory: victory,
            spinner: spinner,
        }
    }

    pub fn init_state(self) -> State {
        State::new(Rc::new(self))
    }

    pub fn init_board(&self) -> Board {
        Board::blank(self.squares_per, self.num_quadrants)
    }

    pub fn get_board_rotation_sq(&self, q_ix: usize, s_ix: usize, direction: usize) -> &Square {
        self.spinner.whole_board_rotations[q_ix][s_ix][direction]
    }

    pub fn get_quadrant_rotation_ix(&self, s_ix: usize, direction: usize) -> usize {
        self.spinner.single_quadrant_rotations[s_ix][direction]
    }

    pub fn get_quadrant_rotations(&self, state: &State) -> Vec<State> {
        self.quadrant_rotations.iter().map(|&(rotate_q_ix, direction)| {
            state.rotate_single_quadrant(rotate_q_ix, direction)
        }).collect()
    }

    pub fn test_line(&self, board: &Board, color: Color, line: &Line) -> bool {
        line.iter()
            .map(|sq| board.get_space(sq))
            .all(|space| space == Some(color))
    }

    fn test_lines(&self, board: &Board, color: Color, lines: &Vec<Line>) -> bool {
        lines.iter().any(|line| self.test_line(board, color, line))
    }

    pub fn test_color(&self, board: &Board, color: Color) -> bool {
        self.test_lines(board, color, &self.all_lines)
    }

    pub fn test_square(&self, board: &Board, color: Color, ix: usize) -> bool {
        self.test_lines(board, color, &self.lines_by_ix[ix])
    }


    // pub fn get_state_val(&self, state: &State) -> BigUint {
    //     self.squares.iter().fold(BigUint::zero(), |val, sq| {
    //         let space = state.get_space(sq);
    //         match space {
    //             None => val,
    //             Some(White) => val + &sq.if_white,
    //             Some(Black) => val + &sq.if_black
    //         }
    //     })
    // }

}
