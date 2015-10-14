use pentago::coordinates::{Coordinates, get_all_coordinates, coordinates_to_ix};
use pentago::rotation_plane::{RotationPlane, RotationPlanes};


#[derive(Debug, Clone)]
pub struct Point {
    pub coordinates: Coordinates,
    pub rotations: Vec<usize>
}

pub type Lattice = Vec<Point>;

pub fn build_lattice(rp: &RotationPlanes, dim: usize, length: usize) -> Lattice {
    get_all_coordinates(dim, length).iter().map(|coordinates| {
        Point {
            coordinates: coordinates.clone(),
            rotations: get_rotations(length, rp, coordinates)
        }
    }).collect()
}

fn apply_rotation(length: usize, coordinates: &Coordinates, rotation_plane: &RotationPlane) -> Coordinates {
    let d_i = rotation_plane[0];
    let d_j = rotation_plane[1];
    let mut rotated_coordinates = coordinates.clone();
    rotated_coordinates[d_i] = length - 1 - coordinates[d_j];
    rotated_coordinates[d_j] = coordinates[d_i];
    rotated_coordinates
}

fn get_rotations(length: usize, rotation_planes: &RotationPlanes, coordinates: &Coordinates) -> Vec<usize> {
    (rotation_planes).iter().map(|rotation_plane| {
        let rotated_coordinates = apply_rotation(length, coordinates, &rotation_plane);
        coordinates_to_ix(rotated_coordinates, length)
    }).collect()
}
