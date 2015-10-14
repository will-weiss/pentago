extern crate num;

use std::ops::BitXor;
use std::iter::Enumerate;
use std::slice::Iter;
use std::rc::Rc;
use pentago::configuration::Configuration;
use self::num::bigint::BigUint;
use self::num::traits::Zero;
use pentago::board::*;
use pentago::board::Color::{White, Black};

pub type GameResult = Option<Color>;

#[derive(Debug, Clone)]
pub struct State {
    pub cfg: Rc<Configuration>,
    pub black_to_move: bool,
    pub result: GameResult,
    pub board: Board
}

impl State {

    pub fn new(cfg: Rc<Configuration>) -> State {
        State {
            cfg: cfg.clone(),
            black_to_move: true,
            result: None,
            board: init_board(cfg.quadrants.len(), cfg.single_quadrant.len())
        }
    }

    fn transition(&self, take_turn: bool, board: Board) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: take_turn.bitxor(self.black_to_move),
            result: self.result,
            board: board
        }
    }

    pub fn val(&self) -> BigUint {
        self.cfg.squares.iter().fold(BigUint::zero(), |val, sq| {
            let space = self.board[sq.q_ix][sq.s_ix];
            match space {
                None => val,
                Some(White) => val + &sq.if_white,
                Some(Black) => val + &sq.if_black
            }
        })
    }

    fn to_move(&self) -> Color {
        if (self.black_to_move) { Black } else { White }
    }

    fn enum_board(&self) -> Enumerate<Iter<QuadrantRef>> {
        self.board.iter().enumerate()
    }

    fn rotate_quadrant(&self, quadrant: &QuadrantRef, direction: usize) -> QuadrantRef {
        Rc::new((0..quadrant.len()).map(|s_ix| {
            let this_pt = &self.cfg.single_quadrant[s_ix];
            let rotate_ix = this_pt.rotations[direction];
            quadrant[rotate_ix].clone()
        }).collect())
    }

    pub fn rotate_single_quadrant(&self, rotate_q: usize, direction: usize) -> State {
        self.transition(false, self.enum_board().map(|(q_ix, quadrant)| {
            if rotate_q != q_ix { quadrant.clone() }
            else { self.rotate_quadrant(quadrant, direction) }
        }).collect())
    }

    pub fn rotate_board(&self, direction: usize) -> State {
        self.transition(false, self.enum_board().map(|(q_ix, quadrant)| {
            Rc::new((0..quadrant.len()).map(|s_ix| {
                let ix = self.cfg.square_ix_by_quadrant[q_ix][s_ix];
                let this_pt = &self.cfg.whole_board[ix];
                let rotate_ix = this_pt.rotations[direction];
                let rotate_sq = &self.cfg.squares[rotate_ix];
                self.board[rotate_sq.q_ix][rotate_sq.s_ix].clone()
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
