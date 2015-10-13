extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub points: Vec<Vec<usize>>,
    pub rotations: Vec<Vec<usize>>,
}

impl Lattice {

    // The possible points of a lattice with a given length and dimension.
    fn get_points(&self) -> Vec<Vec<usize>> {
        (0..self.dim).fold(vec![vec![]], |all_coords, _| {
            Product::new(all_coords.iter(), (0..self.length)).map(|(coords, c)| {
                // There has to be a functional way to do this...
                let mut cs = coords.clone();
                cs.push(c);
                cs
            }).collect()
        })
    }

    fn get_rotation_planes(&self) -> Vec<[usize; 2]> {
        Product::new((0..self.dim), (0..self.dim))
            .filter(|&(d_i, d_j)| { d_i != d_j })
            .map(|(d_i, d_j)| { [d_i, d_j] })
            .collect()
    }

    fn get_rotations(&self) -> Vec<Vec<usize>> {
        let rotation_planes = self.get_rotation_planes();
        (&rotation_planes).iter().map(|rotation_plane| {
            (&self.points).iter().map(|coordinates| {
                let rotated_cs = self.apply_rotation(&coordinates, &rotation_plane);
                self.to_usize(&rotated_cs)
            }).collect()
        }).collect()
    }

    fn apply_rotation(&self, coordinates: &Vec<usize>, rotation_plane: &[usize; 2]) -> Vec<usize> {
        let d_i = rotation_plane[0];
        let d_j = rotation_plane[1];
        let mut rotated_coordinates = coordinates.clone();
        rotated_coordinates[d_i] = self.length - 1 - coordinates[d_j];
        rotated_coordinates[d_j] = coordinates[d_i];
        rotated_coordinates
    }

    fn to_usize(&self, coordinates: &Vec<usize>) -> usize {
        coordinates.iter().enumerate().fold(0, |n, (ix, c)| {
            n + (c * self.length.pow((self.dim - ix - 1) as u32))
        })
    }

    pub fn new(dim: usize, length: usize) -> Lattice {
        let mut lattice = Lattice {
            dim: dim,
            length: length,
            points: vec![],
            rotations: vec![]
        };

        let points = lattice.get_points();
        for p in points {
            lattice.points.push(p);
        }

        let rotations = lattice.get_rotations();
        for r in rotations {
            lattice.rotations.push(r);
        }

        lattice
    }

    pub fn rotate(&self, ix: usize, direction: usize) -> usize {
        self.rotations[direction][ix]
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

