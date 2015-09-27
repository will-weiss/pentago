extern crate num;
extern crate itertools;

use std::rc::Rc;
use pentago::game_state::GameState;
use pentago::coordinates::Coordinates;
use pentago::coordinate_utils::{get_coordinates, apply_rotation, get_rotations};


#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    // pub possible_coordinates: Vec<Coordinates>,
    pub quadrant_coords: Vec<Vec<usize>>,
    pub square_coords: Vec<Vec<usize>>,
}


impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            quadrant_coords: get_coordinates(dim, 2),
            square_coords: get_coordinates(dim, length)
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

    // pub fn rotate(&self, rotation: [usize; 2]) -> usize {

    // }

}
