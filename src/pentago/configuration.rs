extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::itertools::Product;

use pentago::rotation::{RotationPlanes, RotationDirs, get_all_rotation_planes, get_all_rotation_dirs};
use pentago::direction::{LineDir, get_all_line_directions};
use pentago::board::{Square, Squares, Line};
use pentago::lattice::{build_lattice, Lattice, Point};
use pentago::coordinates::{Coordinates, coordinates_to_ix};
use pentago::state::State;
use pentago::math_utils::{three_raised_to, mult2};


const QUADRANT_LENGTH: usize = 2;

pub type Adjacency = Option<usize>;


#[derive(Debug, Clone)]
pub struct Configuration {
    pub dim: usize,
    pub quadrant_length: usize,
    pub radius: usize,
    pub diameter: usize,
    pub num_quadrants: usize,
    pub squares_per: usize,
    pub victory: usize,
    pub line_directions: Vec<LineDir>,
    pub rotation_planes: RotationPlanes,
    pub rotation_dirs: RotationDirs,
    pub squares: Squares,
    pub square_ixs_by_quadrant: Vec<Vec<usize>>,
    pub quadrant_rotations: Vec<Vec<usize>>,
    pub board_rotations: Vec<Vec<usize>>,
    pub lines_by_ix: Vec<Vec<Line>>,
    pub all_lines: Vec<Line>,
}

impl Configuration {

    fn add_planes_dirs(&mut self) {
        self.line_directions = get_all_line_directions(self.dim);
        self.rotation_planes = get_all_rotation_planes(self.dim);
        self.rotation_dirs = get_all_rotation_dirs(&self.rotation_planes);
        self.quadrant_rotations = self.rotations_of(self.radius);
        self.board_rotations = self.rotations_of(self.diameter);
    }

    fn rotations_of(&self, length: usize) -> Vec<Vec<usize>> {
        self.lattice(length).iter().map(|pt| pt.rotations.clone()).collect()
    }

    fn lattice(&self, length: usize) -> Lattice {
        build_lattice(&self.rotation_dirs, self.dim, length)
    }

    fn add_square_fields(&mut self) {

        self.square_ixs_by_quadrant = vec![vec![0; self.squares_per]; self.num_quadrants];

        self.squares = Product::new(0..self.num_quadrants, 0..self.squares_per).enumerate()
            .map(|(b_ix, (q_ix, s_ix))| {
                self.square_ixs_by_quadrant[q_ix][s_ix] = b_ix;
                Square {
                    ixs: (q_ix, s_ix),
                    if_white: three_raised_to(b_ix),
                    if_black: mult2(three_raised_to(b_ix))
                }
            }).collect()
    }

    fn coordinates_out_of_bounds(&self, cs: &Vec<i32>) -> bool {
        !cs.iter().all(|&coordinate|
            coordinate >= 0 && coordinate < (self.diameter as i32)
        )
    }

    fn maybe_adj_coordinates(&self, pt: &Point, ld: &LineDir) -> Option<Coordinates> {
        let adj_cs: Vec<i32> = pt.coordinates.iter()
            .zip(ld).map(|(coordinate, dim_dir)| {
                (coordinate.clone() as i32) + dim_dir.as_i32()
            }).collect();
        if self.coordinates_out_of_bounds(&adj_cs) { None }
        else { Some(adj_cs.iter().map(|c| *c as usize).collect()) }
    }

    fn get_square_adjacencies(&self) -> Vec<Vec<Adjacency>> {
        let whole_board = self.lattice(self.diameter);

        let mut square_adjacencies =
            vec![vec![None; self.squares.len()]; self.line_directions.len()];

        for (sq_ix, (ld_ix, ld)) in Product::new(
            0..self.squares.len(),
            self.line_directions.iter().enumerate()
        ) {
            let maybe_adj_cs = self.maybe_adj_coordinates(&whole_board[sq_ix], ld);
            if let Some(adj_cs) = maybe_adj_cs {
                let adj_ix = coordinates_to_ix(adj_cs, self.diameter);
                square_adjacencies[ld_ix][sq_ix] = Some(adj_ix);
            }
        }

        square_adjacencies

    }

    fn add_lines_from_adjacencies(&mut self, adjs: &Vec<Adjacency>, ix: usize) -> &mut Self {
        let mut line_ixs: Vec<usize> = vec![];
        let mut ref_ix = ix;
        for _ in 1..self.victory {
            match adjs[ref_ix] {
                None => { return self },
                Some(next_ix) => {
                    line_ixs.push(next_ix);
                    ref_ix = next_ix;
                }
            }
        }

        line_ixs.push(ref_ix);
        let line = self.to_line(&line_ixs);

        self.all_lines.push(line.clone());

        for ix in line_ixs {
            self.lines_by_ix[ix].push(line.clone());
        }

        self
    }

    fn to_line(&self, line_ixs: &Vec<usize>) -> Line {
        line_ixs.iter().map(|&ix| self.squares[ix].ixs.clone()).collect()
    }

    fn add_lines(&mut self) {
        self.lines_by_ix = vec![vec![]; self.squares.len()];

        for adjs in self.get_square_adjacencies().iter() {
            for ix in 0..adjs.len() {
                self.add_lines_from_adjacencies(adjs, ix);
            }
        }
    }

    // Create a new Game of a given dimension, quadrant radius, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, radius: usize, victory: usize) -> Configuration {

        let mut configuration = Configuration {
            dim: dim,
            quadrant_length: QUADRANT_LENGTH,
            radius: radius,
            diameter: radius * QUADRANT_LENGTH,
            num_quadrants: (2usize).pow(dim as u32),
            squares_per: (radius).pow(dim as u32),
            victory: victory,
            line_directions: vec![],
            rotation_planes: vec![],
            rotation_dirs: vec![],
            quadrant_rotations: vec![],
            board_rotations: vec![],
            squares: vec![],
            square_ixs_by_quadrant: vec![],
            lines_by_ix: vec![],
            all_lines: vec![],
        };

        configuration.add_planes_dirs();
        configuration.add_square_fields();
        configuration.add_lines();

        configuration

    }

    pub fn init_state(self) -> State {
        State::new(Rc::new(self))
    }

    pub fn get_board_rotation_sq(&self, q_ix: usize, s_ix: usize, direction: usize) -> &Square {
        let ix = self.square_ixs_by_quadrant[q_ix][s_ix];
        let rotate_ix = self.board_rotations[ix][direction];
        &self.squares[rotate_ix]
    }

    pub fn get_quadrant_rotation_ix(&self, s_ix: usize, direction: usize) -> usize {
        self.quadrant_rotations[s_ix][direction]
    }


}
