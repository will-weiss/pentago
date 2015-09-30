extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::lattice::Lattice;

#[derive(Debug, Clone)]
pub struct Point {
    pub ix: usize,
    pub dim: usize,
    pub length: usize,
    pub coordinates: Vec<usize>
}

impl Point {

    pub fn apply_rotation(&self, rotation: &[usize; 2]) -> Vec<usize> {
        let d_i = rotation[0];
        let d_j = rotation[1];
        let mut rotated_coordinates = self.coordinates.clone();
        rotated_coordinates[d_i] = self.length - 1 - self.coordinates[d_j];
        rotated_coordinates[d_j] = self.coordinates[d_i];
        rotated_coordinates
    }

}
