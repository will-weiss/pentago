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
    pub ix: usize,
    pub coordinates: Coordinates,
    pub rotations: Vec<usize>
}

pub type Lattice = Vec<Point>;

pub struct LatticeBuilder {
    dim: usize,
    length: usize,
}

impl LatticeBuilder {

    pub fn build(cfg: &Configuration, length: usize) -> Lattice {
        let builder = LatticeBuilder {
            dim: cfg.dim,
            length: length,
        };

        let all_coordinates = builder.get_all_coordinates();

        all_coordinates.iter().enumerate().map(|(ix, coordinates)| {
            Point {
                ix: ix,
                coordinates: coordinates.clone(),
                rotations: builder.get_rotations(&cfg.rotation_planes, coordinates)
            }
        }).collect()
    }

    fn get_all_coordinates(&self) -> Vec<Coordinates> {
        (0..self.dim).fold(vec![vec![]], |all_cs, _| {
            Product::new(all_cs.iter(), (0..self.length)).map(|(coords, c)| {
                // There has to be a functional way to do this...
                let mut cs = coords.clone();
                cs.push(c);
                cs
            }).collect()
        })
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
