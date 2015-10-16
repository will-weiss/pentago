extern crate time;
extern crate num;
extern crate itertools;

use std::rc::Rc;
use std::ops::BitXor;
use self::itertools::Product;
use self::num::bigint::BigUint;
use pentago::configuration::Configuration;
use pentago::board::{Board, Color, Line};
use pentago::board::Color::{White, Black};
pub use self::GameResult::*;
use self::time::get_time;

static mut states_calculated: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(Color),
    Draw
}

#[derive(Debug, Clone)]
pub struct State {
    cfg: Rc<Configuration>,
    black_to_move: bool,
    board: Board
}

impl State {

    pub fn new(cfg: Rc<Configuration>) -> State {
        State {
            cfg: cfg.clone(),
            black_to_move: true,
            board: cfg.init_board()
        }
    }

    fn transition(&self, take_turn: bool, board: Board) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: take_turn.bitxor(self.black_to_move),
            board: board
        }
    }

    // pub fn val(&self) -> BigUint {
    //     self.cfg.get_state_val(&self)
    // }

    fn to_move(&self) -> Color {
        if self.black_to_move { Black } else { White }
    }

    pub fn rotate_single_quadrant(&self, rotate_q_ix: usize, direction: usize) -> State {
        self.transition(false, self.board.rotate_single_quadrant(rotate_q_ix, direction, &self.cfg))
    }

    pub fn rotate_board(&self, direction: usize) -> State {
        self.transition(false, self.board.rotate_board(direction, &self.cfg))
    }

    pub fn place(&self, place_q: usize, place_s: usize, color: Color) -> State {
        self.transition(true, self.board.place(place_q, place_s, color))
    }

    pub fn possible_placement_states(&self) -> Vec<State> {
        let color = self.to_move();
        self.board.possible_placement_ixs().iter().map(|&(q_ix, s_ix)| {
            self.place(q_ix, s_ix, color)
        }).collect()
    }

    pub fn possible_rotations(&self) -> Vec<State> {
        self.cfg.get_quadrant_rotations(self)
    }

    fn test_color(&self, color: Color) -> bool {
        self.cfg.test_color(&self.board, color)
    }

    pub fn current_result(&self) -> Option<GameResult> {
        let black_has_line = self.test_color(Black);
        let white_has_line = self.test_color(White);

        match (black_has_line, white_has_line) {
            (false, false) => None,
            (true, false) => Some(Win(Black)),
            (false, true) => Some(Win(White)),
            (true, true) => Some(Draw),
        }
    }

    fn test_rotation_states(&self, to_move: Color) -> GameResult {
        let mut draw_seen = false;
        for rotation_state in self.possible_rotations() {
            match rotation_state.full_result() {
                Draw => { draw_seen = true; },
                Win(color) => {
                    if color == to_move { return Win(to_move) }
                }
            }
        }
        if draw_seen { Draw }
        else {
            match to_move {
                Black => Win(White),
                White => Win(Black),
            }
        }
    }

    fn test_for_result(&self) -> GameResult {
        let possible_placements = self.possible_placement_states();
        if possible_placements.len() == 0 { Draw }
        else {
            let to_move = self.to_move();
            let mut draw_seen = false;

            for placement_state in possible_placements.iter() {
                if placement_state.test_color(to_move) { return Win(to_move) }
                else {
                    match placement_state.test_rotation_states(to_move) {
                        Draw => { draw_seen = true; },
                        Win(color) => {
                            if color == to_move { return Win(to_move) }
                        }
                    }
                }
            }
            if draw_seen { Draw }
            else {
                match to_move {
                    Black => Win(White),
                    White => Win(Black),
                }
            }
        }
    }

    pub fn full_result(&self) -> GameResult {
        unsafe {
            states_calculated = states_calculated + 1;
            if states_calculated % 50000 == 0 {
                println!("CALCULATED: {:?} STATES", states_calculated);
                println!("TIME: {:?}", get_time());
            }
        }
        let current_result = self.current_result();
        match current_result {
            Some(result) => result,
            None => { self.test_for_result() }
        }
    }

    pub fn states_calculated(&self) -> u64 {
        unsafe {
            states_calculated
        }
    }

}
