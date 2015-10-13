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
    pub b_ix: usize,
    pub q_ix: usize,
    pub s_ix: usize,

    pub if_white: BigUint,
    pub if_black: BigUint,
}

pub type Squares = Vec<Square>;

#[derive(Debug, Clone, Copy)]
pub enum DimDir {
    Null,
    Forward,
    Backward
}

pub type LineDir = Vec<DimDir>;

fn init_line_direction(prior_dim: usize) -> LineDir {
    let mut ld = vec![DimDir::Null; prior_dim];
    ld.push(DimDir::Forward);
    ld
}

fn non_negative_line_directions(prior_dim: usize) -> Vec<LineDir> {
    let mut next_directions = vec![init_line_direction(prior_dim)];
    for prior_dir in distinct_line_directions(prior_dim).iter() {
        for dir in [DimDir::Null, DimDir::Forward, DimDir::Backward].iter() {
            let mut next_dir = prior_dir.clone();
            next_dir.push(dir.clone());
            next_directions.push(next_dir);
        }
    }
    next_directions
}

fn distinct_line_directions(dim: usize) -> Vec<LineDir> {
    match dim {
        0 => vec![],
        _ => non_negative_line_directions(dim - 1)
    }
}

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
    pub line_directions: Vec<LineDir>,
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

    fn add_line_directions(&mut self) {
        self.line_directions = distinct_line_directions(self.dim);
    }

    fn lattice(&self, length: usize) -> Lattice {
        LatticeBuilder::build(&self.rotation_planes, self.dim, length)
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
            0..self.whole_board.len(),
            Product::new(0..self.quadrants.len(), 0..self.single_quadrant.len())
        )).map(|(b_ix, (q_ix, s_ix))| {

            let if_white = three_raised_to(b_ix);
            let if_black = mult2(if_white.clone());

            Square {
                b_ix: b_ix,
                q_ix: q_ix,
                s_ix: s_ix,
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
            square_ix_by_quadrant[sq.q_ix][sq.s_ix] = sq.b_ix
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
            line_directions: vec![],
            whole_board: vec![],
            quadrants: vec![],
            single_quadrant: vec![],
            squares: vec![],
            square_ix_by_quadrant: vec![]
        };

        configuration.add_rotation_planes();
        configuration.add_line_directions();
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
            let square = board[sq.q_ix][sq.s_ix];
            match square {
                None => val,
                Some(Color::White) => val + &sq.if_white,
                Some(Color::Black) => val + &sq.if_black
            }
        })
    }


}
