extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::point::Point;



#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: usize,
    pub length: usize,
    pub coordinates: Vec<Vec<usize>>,
    pub rotations: Vec<Vec<usize>>
}

impl Lattice {

    // The possible coordinates of a lattice with a given length and dimension.
    fn get_coordinates(&self) -> Vec<Vec<usize>> {
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
            (&self.coordinates).iter().map(|c| {
                let rotated_cs = self.apply_rotation(&c, &rotation_plane);
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

    fn new(dim: usize, length: usize) -> Lattice {
        let mut lattice = Lattice {
            dim: dim,
            length: length,
            coordinates: vec![],
            rotations: vec![]
        };

        let coordinates = lattice.get_coordinates();
        for c in coordinates {
            lattice.coordinates.push(c);
        }

        let rotations = lattice.get_rotations();
        for r in rotations {
            lattice.rotations.push(r);
        }

        lattice
    }

    pub fn points(dim: usize, length: usize) -> Vec<Point> {
        let lattice = Rc::new(Lattice::new(dim, length));
        (0..lattice.coordinates.len()).map(|ix| {
            Point::new(lattice.clone(), ix)
        }).collect()
    }

}

