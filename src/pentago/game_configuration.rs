extern crate num;
extern crate itertools;

use std::rc::Rc;
use pentago::game_state::GameState;
use pentago::square_configuration::SquareCfgList;
use pentago::coordinate_utils::{get_coordinates, apply_rotation, get_rotations};


#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub square_cfgs: SquareCfgList,
    pub quadrant_coords: Vec<Vec<usize>>
}


impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            square_cfgs: SquareCfgList::new(dim, length),
            quadrant_coords: get_coordinates(dim, 2)
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

    // pub fn rotate(&self, rotation: [usize; 2]) -> usize {

    // }

}
