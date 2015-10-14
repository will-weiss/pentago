extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use self::itertools::{Product, Zip};

use pentago::state::State;
use pentago::rotation_plane::{RotationPlanes, RotationPlane, get_all_rotation_planes};
use pentago::direction::{DimDir, LineDir, get_all_line_directions};
use pentago::square::{Square, Squares};
use pentago::lattice::{LatticeBuilder, Point, Lattice};
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color;
use pentago::coordinates::Coordinates;

const quadrant_length: usize = 2;

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
    pub square_adjacencies: Vec<Vec<Option<usize>>>,
}

impl Configuration {

    fn add_rotation_planes(&mut self) {
        self.rotation_planes = get_all_rotation_planes(self.dim);
    }

    fn add_line_directions(&mut self) {
        self.line_directions = get_all_line_directions(self.dim);
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
        let squares = Square::all(self);
        self.squares = squares;
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

    fn add_square_adjacencies(&mut self) {
        let mut square_adjacencies: Vec<Vec<Option<usize>>> =
            (0..self.whole_board.len()).map(|_| {
                vec![None; self.line_directions.len()]
            }).collect();

        for (sq, (ld_ix, ld)) in Product::new(
            self.squares.iter(),
            self.line_directions.iter().enumerate()
        ) {
            let maybe_coordinates: Vec<i32> = self.whole_board[sq.b_ix].coordinates
                .iter()
                .zip(ld)
                .map(|(coordinate, dim_dir)| {
                    (coordinate.clone() as i32) + match *dim_dir {
                        DimDir::Null => 0,
                        DimDir::Forward => 1,
                        DimDir::Backward => -1
                    }
                }).collect();

            if maybe_coordinates.iter().all(|&coordinate| {
                coordinate >= 0 && coordinate < (self.diameter as i32)
            }) {
                 println!("{:?}", &maybe_coordinates);
            }
        }

        self.square_adjacencies = square_adjacencies;
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
            square_ix_by_quadrant: vec![],
            square_adjacencies: vec![],
        };

        configuration.add_rotation_planes();
        configuration.add_line_directions();
        configuration.add_whole_board();
        configuration.add_quadrants();
        configuration.add_single_quadrant();
        configuration.add_squares();
        configuration.add_square_ix_by_quadrant();
        configuration.add_square_adjacencies();

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
