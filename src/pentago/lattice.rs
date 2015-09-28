extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::point::Point;

// The possible coordinates of a lattice with a given length and dimension.
fn get_coordinates(dim: usize, length: usize) -> Vec<Vec<usize>> {
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
fn get_rotations(dim: usize) -> Vec<[usize; 2]> {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| { d_i != d_j })
        .map(|(d_i, d_j)| { [d_i, d_j] })
        .collect()
}


#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub points: Vec<Point>,
    pub rotations: Vec<Vec<Point>>
}

impl Lattice {

    pub fn new(dim: usize, length: usize) -> Lattice {
        let points: Vec<Point> = get_coordinates(dim, length).iter().enumerate()
            .map(|(ix, coordinates)| {
                Point {
                    dim: dim,
                    length: length,
                    coordinates: coordinates.clone()
                }
            }).collect();

        let mut rotations: Vec<Vec<Point>> = vec![];
        {
            for rotation in get_rotations(dim) {
                rotations.push(points.iter().map(|point| {
                    point.apply_rotation(&rotation)
                }).collect());
            }
        }

        Lattice {
            dim: dim,
            length: length,
            points: points,
            rotations: rotations
        }

    }
}

