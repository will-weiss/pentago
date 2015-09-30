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
    pub quadrants: Vec<Point>,
}


impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            squares: Lattice::points(dim, length),
            quadrants: Lattice::points(dim, 2),
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

}
