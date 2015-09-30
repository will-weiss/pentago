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

#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub points: Vec<Rc<Point>>,
    pub rotations: Vec<Vec<usize>>
}

impl Lattice {

    fn to_usize(&self, coordinates: Vec<usize>) -> usize {
        coordinates.iter().enumerate().fold(0, |n, (ix, c)| {
            n + (c * self.length.pow((self.dim - ix - 1) as u32))
        })
    }
    // The possible choices of rotation.
    fn get_rotations(&self) -> Vec<[usize; 2]> {
        Product::new((0..self.dim), (0..self.dim))
            .filter(|&(d_i, d_j)| { d_i != d_j })
            .map(|(d_i, d_j)| { [d_i, d_j] })
            .collect()
    }

    pub fn new(dim: usize, length: usize) -> Lattice {

        let mut lattice = Lattice {
            dim: dim,
            length: length,
            points: get_coordinates(dim, length).iter().enumerate()
                .map(|(ix, coordinates)| {
                    Rc::new(
                        Point {
                            ix: ix,
                            dim: dim,
                            length: length,
                            coordinates: coordinates.clone(),
                        }
                    )
                }).collect(),
            rotations: vec![]
        };

        for rotation in lattice.get_rotations() {
            let additional_rotations = (&lattice.points).iter().map(|point| {
                lattice.to_usize(point.apply_rotation(&rotation))
            }).collect();
            lattice.rotations.push(additional_rotations);
        }

        lattice

    }

    pub fn rotate(&self, ix: usize) -> Vec<Rc<Point>> {

    }

}

