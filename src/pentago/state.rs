extern crate num;

use std::ops::BitXor;
use std::iter::Enumerate;
use std::slice::Iter;
use std::rc::Rc;
use self::num::bigint::BigUint;
use pentago::configuration::Configuration;
use pentago::board::{Board, Square, Space, QuadrantRef, Color, Line};
use pentago::board::Color::{White, Black};
pub use self::GameResult::*;

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

    fn test_color(&self, color: Color) -> bool {
        self.cfg.test_color(&self.board, color)
    }

    pub fn get_result(&self) -> Option<GameResult> {
        let black_has_line = self.test_color(Black);
        let white_has_line = self.test_color(White);

        match (black_has_line, white_has_line) {
            (false, false) => None,
            (true, false) => Some(Win(Black)),
            (false, true) => Some(Win(White)),
            (true, true) => Some(Draw),
        }
    }

    fn transition(&self, take_turn: bool, board: Board) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: take_turn.bitxor(self.black_to_move),
            board: board
        }
    }

    pub fn val(&self) -> BigUint {
        self.cfg.get_state_val(&self)
    }

    fn to_move(&self) -> Color {
        if self.black_to_move { Black } else { White }
    }

    fn enum_board(&self) -> Enumerate<Iter<QuadrantRef>> {
        self.board.iter().enumerate()
    }

    pub fn get_space(&self, sq: &Square) -> Space {
        sq.of(&self.board)
    }

    fn rotate_quadrant(&self, quadrant: &QuadrantRef, direction: usize) -> QuadrantRef {
        Rc::new((0..quadrant.len()).map(|s_ix| {
            let rotate_ix = self.cfg.get_quadrant_rotation_ix(s_ix, direction);
            quadrant[rotate_ix].clone()
        }).collect())
    }

    pub fn rotate_single_quadrant(&self, rotate_q_ix: usize, direction: usize) -> State {
        self.transition(false, self.enum_board().map(|(q_ix, quadrant)| {
            if rotate_q_ix != q_ix { quadrant.clone() }
            else { self.rotate_quadrant(quadrant, direction) }
        }).collect())
    }

    pub fn rotate_board(&self, direction: usize) -> State {
        self.transition(false, self.enum_board().map(|(q_ix, quadrant)| {
            Rc::new((0..quadrant.len()).map(|s_ix| {
                let rotate_sq = self.cfg.get_board_rotation_sq(q_ix, s_ix, direction);
                self.get_space(rotate_sq)
            }).collect())
        }).collect())
    }

    fn place_in_quadrant(&self, quadrant: &QuadrantRef, place_s: usize, color: Color) -> QuadrantRef {
        Rc::new(quadrant.iter().enumerate().map(|(s_ix, space)| {
            if place_s == s_ix { Some(color) }
            else { space.clone() }
        }).collect())
    }

    pub fn place(&self, place_q: usize, place_s: usize, color: Color) -> State {
        self.transition(true, self.enum_board().map(|(q_ix, quadrant)| {
            if place_q != q_ix { quadrant.clone() }
            else { self.place_in_quadrant(quadrant, place_s, color) }
        }).collect())
    }

    pub fn possible_placements(&self) -> Vec<State> {
        let color = self.to_move();
        let mut placement_states = vec![];

        for (q_ix, q) in self.enum_board() {
            for (s_ix, s) in q.iter().enumerate() {
                if (*s).is_none() {
                    placement_states.push(self.place(q_ix, s_ix, color));
                }
            }
        }

        placement_states

    }

}
