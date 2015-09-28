extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::lattice::Lattice;

#[derive(Debug, Clone)]
pub struct Point {
    pub dim: usize,
    pub length: usize,
    pub coordinates: Vec<usize>
}

impl Point {

    pub fn apply_rotation(&self, rotation: &[usize; 2]) -> Point {
        let d_i = rotation[0];
        let d_j = rotation[1];
        let mut rotated_coordinates = self.coordinates.clone();
        rotated_coordinates[d_i] = self.length - 1 - self.coordinates[d_j];
        rotated_coordinates[d_j] = self.coordinates[d_i];
        Point {
            dim: self.dim,
            length: self.length,
            coordinates: rotated_coordinates
        }
    }

    pub fn to_usize(&self) -> usize {
        self.coordinates.iter().enumerate().fold(0, |n, (ix, c)| {
            n + (c * self.length.pow((self.dim - ix - 1) as u32))
        })
    }

}
