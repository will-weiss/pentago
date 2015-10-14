use pentago::coordinates::{Coordinates, get_all_coordinates, coordinate_to_ix};
use pentago::rotation_plane::{RotationPlane, RotationPlanes};
use pentago::configuration::Configuration;
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color::{Black, White};


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
            all_coordinates: get_all_coordinates(dim, length)
        }).to_lattice(rp)
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

    fn get_rotations(&self, rotation_planes: &RotationPlanes, coordinates: &Coordinates) -> Vec<usize> {
        (rotation_planes).iter().map(|rotation_plane| {
            let rotated_coordinates = self.apply_rotation(coordinates, &rotation_plane);
            coordinate_to_ix(rotated_coordinates, self.length)
        }).collect()
    }

}
