extern crate num;
extern crate itertools;

use std::rc::Rc;
use pentago::game_state::GameState;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::math_utils::{three_raised_to, mult2, mult3, factorial};
use self::itertools::Product;


#[derive(Debug, Clone)]
pub struct GameConfiguration {
    pub dim: usize,
    pub length: usize,
    pub victory: usize,
    pub rotations: Vec<[usize; 2]>,
    pub quadrant_coordinates: Vec<Vec<usize>>,
    pub square_coordinates: Vec<Vec<usize>>
}

// The possible coordinates of a lattice with a given length and dimension.
fn coordinates(dim: usize, length: usize) -> Vec<Vec<usize>> {
    (0..dim).fold(vec![vec![]], |all_coords, _| {
        Product::new(all_coords.iter(), (0..length)).map(|(coords, c)| {
            let mut cs = coords.clone();
            cs.push(c);
            cs
        }).collect()
    })
}

// The possible choices of rotation.
fn rotations(dim: usize) -> Vec<[usize; 2]> {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| { d_i != d_j })
        .map(|(d_i, d_j)| { [d_i, d_j] })
        .collect()
}

impl GameConfiguration {
    // Create a new Game of a given dimension, quadrant length, and number of
    // squares in a row that indicate victory.
    pub fn new(dim: usize, length: usize, victory: usize) -> GameConfiguration {
        GameConfiguration {
            dim: dim,
            length: length,
            victory: victory,
            rotations: rotations(dim),
            quadrant_coordinates: coordinates(dim, 2),
            square_coordinates: coordinates(dim, length)
        }
    }

    pub fn init_state(self) -> GameState {
        GameState::new(Rc::new(self))
    }

}
