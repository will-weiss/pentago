extern crate num;
extern crate itertools;

use std::rc::Rc;
use pentago::game_state::GameState;
use pentago::lattice::Lattice;
use pentago::point::Point;


#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub squares: Vec<Point>,
    pub sq_lattice: Rc<Lattice>,
    pub quadrants: Vec<Point>,
    pub q_lattice: Rc<Lattice>
}


impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        let squares = Lattice::points(dim, length);
        let sq_lattice = (&squares)[0].lattice.clone();
        let quadrants = Lattice::points(dim, 2);
        let q_lattice = (&quadrants)[0].lattice.clone();


        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            squares: squares,
            quadrants: quadrants,
            sq_lattice: sq_lattice,
            q_lattice: q_lattice
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

}
