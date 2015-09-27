extern crate itertools;
use self::itertools::Product;

// The possible coordinates of a lattice with a given length and dimension.
pub fn get_coordinates(dim: usize, length: usize) -> Vec<Vec<usize>> {
    (0..dim).fold(vec![vec![]], |all_coords, _| {
        Product::new(all_coords.iter(), (0..length)).map(|(coords, c)| {
            // There has to be a functional way to do this...
            let mut cs = coords.clone();
            cs.push(c);
            cs
        }).collect()
    })
}

pub fn apply_rotation(coordinates: Vec<usize>, rotation: [usize; 2], length: usize) -> Vec<usize> {
    let d_i = rotation[0];
    let d_j = rotation[1];
    let mut rotated_coordinates = coordinates.clone();
    rotated_coordinates[d_i] = coordinates[d_j];
    rotated_coordinates[d_j] = length - 1 - coordinates[d_i];
    rotated_coordinates
}

// The possible choices of rotation.
pub fn get_rotations(dim: usize) -> Vec<[usize; 2]> {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| { d_i != d_j })
        .map(|(d_i, d_j)| { [d_i, d_j] })
        .collect()
}
