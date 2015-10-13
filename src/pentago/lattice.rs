extern crate itertools;

use self::itertools::Product;

use std::rc::Rc;

use pentago::state::State;
use pentago::configuration::{Configuration, RotationPlane, RotationPlanes};
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color::{Black, White};

pub type Coordinates = Vec<usize>;

#[derive(Debug, Clone)]
pub struct Point {
    pub coordinates: Coordinates,
    pub rotations: Vec<usize>
}

pub type Lattice = Vec<Point>;

pub struct LatticeBuilder {
    dim: usize,
    length: usize,
    all_coordinates: Vec<Coordinates>
}

impl LatticeBuilder {

    pub fn build(rp: &RotationPlanes, dim: usize, length: usize) -> Lattice {
        (LatticeBuilder {
            dim: dim,
            length: length,
            all_coordinates: LatticeBuilder::get_all_coordinates(dim, length)
        }).to_lattice(rp)
    }

    pub fn get_all_coordinates(dim: usize, length: usize) -> Vec<Coordinates> {
        (0..dim).fold(vec![vec![]], |all_cs, _| {
            Product::new(all_cs.iter(), (0..length)).map(|(coords, c)| {
                // There has to be a functional way to do this...
                let mut cs = coords.clone();
                cs.push(c);
                cs
            }).collect()
        })
    }

    fn to_lattice(&self, rp: &RotationPlanes) -> Lattice {
        self.all_coordinates.iter().enumerate().map(|(ix, coordinates)| {
            Point {
                coordinates: coordinates.clone(),
                rotations: self.get_rotations(rp, coordinates)
            }
        }).collect()
    }

    fn apply_rotation(&self, coordinates: &Coordinates, rotation_plane: &RotationPlane) -> Coordinates {
        let d_i = rotation_plane[0];
        let d_j = rotation_plane[1];
        let mut rotated_coordinates = coordinates.clone();
        rotated_coordinates[d_i] = self.length - 1 - coordinates[d_j];
        rotated_coordinates[d_j] = coordinates[d_i];
        rotated_coordinates
    }

    fn ix(&self, coordinates: &Coordinates) -> usize {
        coordinates.iter().enumerate().fold(0, |ix, (dimension_ix, coordinate)| {
            let inverted_dim_ix= (self.dim - dimension_ix - 1) as u32;
            let multiplier = self.length.pow(inverted_dim_ix);
            ix + (coordinate * multiplier)
        })
    }

    fn get_rotations(&self, rotation_planes: &RotationPlanes, coordinates: &Coordinates) -> Vec<usize> {
        (rotation_planes).iter().map(|rotation_plane| {
            let rotated_coordinates = self.apply_rotation(coordinates, &rotation_plane);
            self.ix(&rotated_coordinates)
        }).collect()
    }

}
