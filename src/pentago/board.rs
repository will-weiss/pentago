extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::bigint::BigUint;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White
}

pub type Space = Option<Color>;
pub type Quadrant = Vec<Space>;
pub type QuadrantRef = Rc<Quadrant>;
pub type Board = Vec<QuadrantRef>;
pub type SquareIxs = (usize, usize);
pub type Line = Vec<SquareIxs>;

#[derive(Debug, Clone)]
pub struct Square {
    pub ixs: SquareIxs,
    pub if_white: BigUint,
    pub if_black: BigUint,
}

pub type Squares = Vec<Square>;

impl Square {
    pub fn of(&self, board: &Board) -> Space {
        let (q_ix, s_ix) = self.ixs;
        board[q_ix][s_ix]
    }
}

fn init_quadrant(sq_len: usize) -> Quadrant {
    vec![None; sq_len]
}

pub fn init(qs_len: usize, sq_len: usize) -> Board {
    vec![Rc::new(init_quadrant(sq_len)); qs_len]
}
