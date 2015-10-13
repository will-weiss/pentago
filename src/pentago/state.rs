extern crate num;

use std::rc::Rc;
use pentago::color::Color;
use pentago::configuration::{Configuration, Board, Quadrant};
use self::num::bigint::BigUint;

#[derive(Debug, Clone)]
pub enum GameResult {
    BlackWin,
    WhiteWin
}

#[derive(Debug, Clone)]
pub struct State {
    pub cfg: Rc<Configuration>,
    pub black_to_move: bool,
    pub result: Option<GameResult>,
    pub board: Board
}

impl State {

    pub fn new(cfg: Rc<Configuration>) -> State {
        State {
            cfg: cfg.clone(),
            black_to_move: true,
            result: None,
            board: cfg.init_board()
        }
    }

    pub fn val(&self) -> BigUint {
        self.cfg.val(&self.board)
    }

    pub fn to_move(&self) -> Color {
        match self.black_to_move {
            true => Color::Black,
            false => Color::White
        }
    }

    pub fn place(&self, place_q: usize, place_s: usize, color: Color) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: !self.black_to_move,
            result: None,
            board: self.board.iter().enumerate().map(|(q_ix, quadrant)| {
                if (place_q == q_ix) {
                    Rc::new(quadrant.iter().enumerate().map(|(s_ix, square)| {
                        if (place_s == s_ix) {
                            Some(color)
                        } else {
                            square.clone()
                        }
                    }).collect())
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

    pub fn rotate_quadrant(&self, rotate_q: usize, direction: usize) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: self.black_to_move,
            result: None,
            board: self.board.iter().enumerate().map(|(q_ix, quadrant)| {
                if (rotate_q == q_ix) {
                    Rc::new(quadrant.iter().enumerate().map(|(s_ix, square)| {
                        let rotate_to = self.cfg.single_quadrant[s_ix].rotations[direction];
                        quadrant[rotate_to].clone()
                    }).collect())
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

    pub fn rotate_board(&self, direction: usize) -> State {
        State {
            cfg: self.cfg.clone(),
            black_to_move: self.black_to_move,
            result: None,
            board: self.board.iter().enumerate().map(|(q_ix, quadrant)| {
                Rc::new(quadrant.iter().enumerate().map(|(s_ix, square)| {
                    let ix = self.cfg.square_indexes_by_quadrant[q_ix][s_ix];
                    quadrant[ix].clone()
                }).collect())
            }).collect()
        }
    }

    pub fn possible_placements(&self) -> Vec<State> {
        let color = self.to_move();
        let mut placement_states = vec![];

        for (q_ix, q) in self.board.iter().enumerate() {
            for (s_ix, s) in q.iter().enumerate() {
                if (*s).is_none() {
                    placement_states.push(self.place(q_ix, s_ix, color));
                }
            }
        }

        placement_states

    }

}
