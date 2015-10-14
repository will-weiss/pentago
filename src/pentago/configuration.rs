extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::itertools::Product;

use pentago::rotation_plane::{RotationPlanes, get_all_rotation_planes};
use pentago::direction::{LineDir, get_all_line_directions};
use pentago::square::{Square, Squares};
use pentago::lattice::{build_lattice, Lattice};
use pentago::coordinates::{Coordinates, coordinates_to_ix};
use pentago::state::State;

const QUADRANT_LENGTH: usize = 2;


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

    fn lattice(&self, length: usize) -> Lattice {
        build_lattice(&self.rotation_planes, self.dim, length)
    }

    fn add_rotation_planes(&mut self) {
        self.rotation_planes = get_all_rotation_planes(self.dim);
    }

    fn add_line_directions(&mut self) {
        self.line_directions = get_all_line_directions(self.dim);
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
            vec![vec![0; self.single_quadrant.len()]; self.quadrants.len()];

        for sq in (&self.squares) {
            square_ix_by_quadrant[sq.q_ix][sq.s_ix] = sq.b_ix
        }

        self.square_ix_by_quadrant = square_ix_by_quadrant;
    }

    fn coordinates_out_of_bounds(&self, cs: &Vec<i32>) -> bool {
        !cs.iter().all(|&coordinate|
            coordinate >= 0 && coordinate < (self.diameter as i32)
        )
    }

    fn maybe_adj_coordinates(&self, sq: &Square, ld: &LineDir) -> Option<Coordinates> {
        let adj_cs: Vec<i32> = self.whole_board[sq.b_ix].coordinates.iter()
            .zip(ld).map(|(coordinate, dim_dir)| {
                (coordinate.clone() as i32) + dim_dir.as_i32()
            }).collect();
        if self.coordinates_out_of_bounds(&adj_cs) { None }
        else { Some(adj_cs.iter().map(|c| *c as usize).collect()) }
    }

    fn add_square_adjacencies(&mut self) {
        let mut square_adjacencies: Vec<Vec<Option<usize>>> =
            vec![vec![None; self.line_directions.len()]; self.whole_board.len()];

        let square_iter = Product::new(
            self.squares.iter(),
            self.line_directions.iter().enumerate()
        );

        for (sq, (ld_ix, ld)) in square_iter {
            let maybe_adj_cs = self.maybe_adj_coordinates(sq, ld);
            if let Some(adj_cs) = maybe_adj_cs {
                let adj_ix = coordinates_to_ix(adj_cs, self.diameter);
                square_adjacencies[sq.b_ix][ld_ix] = Some(adj_ix);
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
            quadrant_length: QUADRANT_LENGTH,
            diameter: radius * QUADRANT_LENGTH,
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

    pub fn init_state(self) -> State {
        State::new(Rc::new(self))
    }

}
