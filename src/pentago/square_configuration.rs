use std::rc::Rc;
use pentago::coordinate_utils::{get_coordinates, get_rotations};

#[derive(Debug, Clone)]
pub struct SquareConfiguration {
    pub coordinates: Vec<usize>,
    pub rotations: Vec<usize>
}

#[derive(Debug, Clone)]
pub struct SquareCfgList {
    pub dim: usize,
    pub length: usize,
    pub square_configurations: Vec<Rc<SquareConfiguration>>
}

fn to_usize(dim: usize, length: usize, coords: Vec<usize>) -> usize {
    coords.iter().enumerate().fold(0, |n, (ix, c)| {
        n + (c * length.pow((dim - ix - 1) as u32))
    })
}

fn apply_rotation(length: usize, coordinates: &Vec<usize>, rotation: &[usize; 2]) -> Vec<usize> {
    let d_i = rotation[0];
    let d_j = rotation[1];
    let mut rotated_coordinates = coordinates.clone();
    rotated_coordinates[d_i] = length - 1 - coordinates[d_j];
    rotated_coordinates[d_j] = coordinates[d_i];
    rotated_coordinates
}

impl SquareCfgList {

    pub fn new(dim: usize, length: usize) -> SquareCfgList {
        let all_coordinates = get_coordinates(dim, length);
        let rotations = get_rotations(dim);

        SquareCfgList {
            dim: dim,
            length: length,
            square_configurations: all_coordinates.iter().map(|coordinates| {
                Rc::new(SquareConfiguration {
                    coordinates: coordinates.clone(),
                    rotations: rotations.iter().map(|rotation| {
                        to_usize(dim, length, apply_rotation(length, coordinates, rotation))
                    }).collect()
                })
            }).collect()
        }
    }
}
