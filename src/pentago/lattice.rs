extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::point::Point;


#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub rotation_planes: Vec<[usize; 2]>,
    pub coordinates: Vec<Vec<usize>>
}

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

fn get_rotation_planes(dim: usize) -> Vec<[usize; 2]> {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| { d_i != d_j })
        .map(|(d_i, d_j)| { [d_i, d_j] })
        .collect()
}

impl Lattice {

    fn new(dim: usize, length: usize) -> Lattice {
        Lattice {
            dim: dim,
            length: length,
            rotation_planes: get_rotation_planes(dim),
            coordinates: get_coordinates(dim, length)
        }
    }

    pub fn points(dim: usize, length: usize) -> Vec<Point> {
        let lattice = Lattice::new(dim, length);
        Point::from(lattice)
    }

    pub fn coordinates_val(&self, coordinates: &Vec<usize>) -> usize {
        (coordinates).iter().enumerate().fold(0, |n, (ix, c)| {
            n + (c * self.length.pow((self.dim - ix - 1) as u32))
        })
    }

}

