extern crate num;

use std::rc::Rc;
use pentago::board::Board;
use pentago::color::Color;
use pentago::game_configuration::GameConfiguration;
use self::num::bigint::BigUint;

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

    pub fn val(&self) -> BigUint {
        self.cfg.val(&self.board)
    }

    pub fn to_move(&self) -> Color {
        match self.black_to_move {
            true => Color::Black,
            false => Color::White
        }
    }

    pub fn place(&self, q_ix: usize, s_ix: usize, color: Color) -> GameState {
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
                if (*s).is_none() {
                    placement_states.push(self.place(q_ix, s_ix, color));
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

}
