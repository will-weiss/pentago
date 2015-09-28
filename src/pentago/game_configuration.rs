extern crate num;
extern crate itertools;

use std::rc::Rc;
use pentago::game_state::GameState;
use pentago::lattice::Lattice;


#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub squares: Lattice,
    pub quadrants: Lattice
}


impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            squares: Lattice::new(dim, length),
            quadrants: Lattice::new(dim, 2),
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

    // pub fn rotate(&self, rotation: [usize; 2]) -> usize {

    // }

}
