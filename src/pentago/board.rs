// extern crate num;

// use std::rc::Rc;
// use self::num::bigint::BigUint;


// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Color {
//     Black,
//     White
// }

// pub type Space = Option<Color>;
// pub type Quadrant = Vec<Space>;
// pub type QuadrantRef = Rc<Quadrant>;
// pub type Board = Vec<QuadrantRef>;


extern crate num;

use std::rc::Rc;

use std::slice::Iter;
use std::iter::{Enumerate, Map};

use self::num::traits::Zero;
use self::num::bigint::BigUint;
pub use self::Color::*;

use pentago::configuration::Configuration;

pub type Square = (usize, usize);
pub type Squares = Vec<Square>;
pub type Line = Vec<Square>;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White
}

pub type Space = Option<Color>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quadrant {
    spaces: Vec<Space>,
    as_val: u32,
}

impl Quadrant {

    fn create(spaces: Vec<Space>, as_val: u32) -> QuadrantRef {
        Rc::new(Quadrant {
            spaces: spaces,
            as_val: as_val,
        })
    }

    pub fn blank(squares_per: usize) -> QuadrantRef {
        Quadrant::create(vec![None; squares_per], 0u32)
    }

    fn new(spaces: Vec<Space>) -> QuadrantRef {

        let mut as_val = 0u32;

        for (ix, &space) in spaces.iter().enumerate() {
            as_val = as_val + match space {
                None => 2,
                Some(White) => 1,
                Some(Black) => 0
            };
            if ix != 0 { as_val << 2; }
        }

        Quadrant::create(spaces, as_val)
    }

    pub fn enum_sps(&self) -> Enumerate<Iter<Space>> {
        self.spaces.iter().enumerate()
    }

    pub fn place(&self, place_s: usize, color: Color) -> QuadrantRef {
        Quadrant::new(self.enum_sps().map(|(s_ix, space)| {
            if place_s == s_ix { Some(color) }
            else { space.clone() }
        }).collect())
    }

    pub fn rotate(&self, direction: usize, cfg: &Rc<Configuration>) -> QuadrantRef {
        Quadrant::new((0..self.spaces.len()).map(|s_ix| {
            let rotate_ix = cfg.get_quadrant_rotation_ix(s_ix, direction);
            self.spaces[rotate_ix].clone()
        }).collect())
    }

}

pub type QuadrantRef = Rc<Quadrant>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub quadrants: Vec<QuadrantRef>,
    pub as_val: BigUint,
}

impl Board {

    fn create(quadrants: Vec<QuadrantRef>, as_val: BigUint) -> Board {
        Board {
            quadrants: quadrants,
            as_val: as_val,
        }
    }

    fn new(quadrants: Vec<QuadrantRef>) -> Board {
        let as_val = BigUint::new(quadrants.iter().map(|quadrant| {
            quadrant.as_val
        }).collect());

        Board::create(quadrants, as_val)
    }

    pub fn blank(squares_per: usize, num_quadrants: usize) -> Board {
        let quadrants = vec![Quadrant::blank(squares_per); num_quadrants];
        Board::create(quadrants, BigUint::zero())
    }

    fn iter_qs(&self) -> Iter<QuadrantRef> {
        self.quadrants.iter()
    }

    fn enum_qs(&self) -> Enumerate<Iter<QuadrantRef>> {
        self.iter_qs().enumerate()
    }

    fn transition(&self, quadrant_lambda: &Fn((usize, &QuadrantRef)) -> QuadrantRef) -> Board {
        Board::new(self.enum_qs().map(quadrant_lambda).collect())
    }

    pub fn get_space(&self, sq: &Square) -> Space {
        let &(q_ix, s_ix) = sq;
        self.quadrants[q_ix].spaces[s_ix]
    }

    pub fn rotate_single_quadrant(&self, rotate_q_ix: usize, direction: usize, cfg: &Rc<Configuration>) -> Board {
        self.transition(&|(q_ix, quadrant)| {
            if rotate_q_ix != q_ix { quadrant.clone() }
            else { quadrant.rotate(direction, cfg) }
        })
    }

    pub fn rotate_board(&self, direction: usize, cfg: &Rc<Configuration>) -> Board {
        self.transition(&|(q_ix, quadrant)| {
            Quadrant::new((0..quadrant.spaces.len()).map(|s_ix| {
                let rotate_sq = cfg.get_board_rotation_sq(q_ix, s_ix, direction);
                self.get_space(&rotate_sq)
            }).collect())
        })
    }

    pub fn place(&self, place_q: usize, place_s: usize, color: Color) -> Board {
        self.transition(&|(q_ix, quadrant)| {
            if place_q != q_ix { quadrant.clone() }
            else { quadrant.place(place_s, color) }
        })
    }

    pub fn possible_placement_ixs(&self) -> Vec<(usize, usize)> {
        let mut placement_ixs = vec![];

        for (q_ix, quadrant) in self.enum_qs() {
            for (s_ix, s) in quadrant.spaces.iter().enumerate() {
                if (*s).is_none() {
                    placement_ixs.push((q_ix, s_ix));
                }
            }
        }

        placement_ixs

    }

}
