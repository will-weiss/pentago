extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use self::itertools::{Product, Zip};

use pentago::state::State;
use pentago::lattice::{LatticeBuilder, Point, Lattice, Coordinates};
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color;

const quadrant_length: usize = 2;


#[derive(Debug, Clone)]
pub struct Square {
    pub b_pt: Point,
    pub q_pt: Point,
    pub s_pt: Point,

    pub if_white: BigUint,
    pub if_black: BigUint,
}


pub type Squares = Vec<Square>;

pub type RotationPlane = [usize; 2];
pub type RotationPlanes = Vec<RotationPlane>;

pub type Quadrant = Vec<Option<Color>>;
pub type Board = Vec<Rc<Quadrant>>;


#[derive(Debug, Clone)]
pub struct Configuration {
    pub dim: usize,
    pub radius: usize,
    pub quadrant_length: usize,
    pub diameter: usize,
    pub victory: usize,
    pub rotation_planes: RotationPlanes,
    pub whole_board: Lattice,
    pub quadrants: Lattice,
    pub single_quadrant: Lattice,
    pub squares: Squares,
    pub square_ix_by_quadrant: Vec<Vec<usize>>,
}

impl Configuration {

    fn add_rotation_planes(&mut self) {
        self.rotation_planes = Product::new((0..self.dim), (0..self.dim))
            .filter(|&(d_i, d_j)| { d_i != d_j })
            .map(|(d_i, d_j)| { [d_i, d_j] })
            .collect();
    }

    fn lattice(&self, length: usize) -> Lattice {
        LatticeBuilder::build(self, length)
    }

    fn add_whole_board(&mut self) {
        self.whole_board = self.lattice(self.diameter);
    }

    fn add_quadrants(&mut self) {
        self.quadrants = self.lattice(self.quadrant_length);
    }

    fn add_single_quadrant(&mut self) {
        self.single_quadrant = self.lattice(self.radius);
    }

    fn add_squares(&mut self) {
        self.squares = Zip::new((
            self.whole_board.iter(),
            Product::new(self.quadrants.iter(),self.single_quadrant.iter())
        )).map(|(b_pt, (q_pt, s_pt))| {

            let if_white = three_raised_to(b_pt.ix);
            let if_black = mult2(if_white.clone());

            Square {
                b_pt: b_pt.clone(),
                q_pt: q_pt.clone(),
                s_pt: s_pt.clone(),
                if_white: if_white,
                if_black: if_black
            }
        }).collect();
    }

    fn add_square_ix_by_quadrant(&mut self) {
        let mut square_ix_by_quadrant: Vec<Vec<usize>> =
            (0..self.quadrants.len()).map(|_| {
                vec![0; self.single_quadrant.len()]
            }).collect();

        for sq in (&self.squares) {
            square_ix_by_quadrant[sq.q_pt.ix][sq.s_pt.ix] = sq.b_pt.ix
        }

        self.square_ix_by_quadrant = square_ix_by_quadrant;
    }

    // Create a new Game of a given dimension, quadrant radius, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, radius: usize, victory: usize) -> Configuration {

        let mut configuration = Configuration {
            dim: dim,
            radius: radius,
            quadrant_length: quadrant_length,
            diameter: radius * quadrant_length,
            victory: victory,
            rotation_planes: vec![],
            whole_board: vec![],
            quadrants: vec![],
            single_quadrant: vec![],
            squares: vec![],
            square_ix_by_quadrant: vec![]
        };

        configuration.add_rotation_planes();
        configuration.add_whole_board();
        configuration.add_quadrants();
        configuration.add_single_quadrant();
        configuration.add_squares();
        configuration.add_square_ix_by_quadrant();

        configuration

    }

    pub fn init_quadrant(&self) -> Quadrant {
        (0..self.single_quadrant.len()).map(|_| {
            None
        }).collect()
    }

    pub fn init_board(&self) -> Board {
        (0..self.quadrants.len()).map(|_| {
            Rc::new(self.init_quadrant())
        }).collect()
    }

    pub fn init_state(self) -> State {
        State::new(Rc::new(self))
    }

    // Calculate the numeric representation of a given board.
    pub fn val(&self, board: &Board) -> BigUint {
        (self.squares).iter().fold(BigUint::zero(), |val, sq| {
            let square = board[sq.q_pt.ix][sq.s_pt.ix];
            match square {
                None => val,
                Some(Color::White) => val + &sq.if_white,
                Some(Color::Black) => val + &sq.if_black
            }
        })
    }


}
