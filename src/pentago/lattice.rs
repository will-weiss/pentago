extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Point {
    pub coordinates: Vec<usize>,
    pub rotations: Vec<usize>
}

#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub points: Vec<Rc<Point>>
}


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

// The possible choices of rotation.
pub fn get_rotations(dim: usize) -> Vec<[usize; 2]> {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| { d_i != d_j })
        .map(|(d_i, d_j)| { [d_i, d_j] })
        .collect()
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

impl Lattice {

    pub fn new(dim: usize, length: usize) -> Lattice {
        let all_coordinates = get_coordinates(dim, length);
        let rotations = get_rotations(dim);

        Lattice {
            dim: dim,
            length: length,
            points: all_coordinates.iter().map(|coordinates| {
                Rc::new(Point {
                    coordinates: coordinates.clone(),
                    rotations: rotations.iter().map(|rotation| {
                        to_usize(dim, length, apply_rotation(length, coordinates, rotation))
                    }).collect()
                })
            }).collect()
        }
    }
}
