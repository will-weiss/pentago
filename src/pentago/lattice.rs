extern crate itertools;
use self::itertools::Product;
use pentago::direction::{DimDir, LineDir};
use std::slice::Iter;
use std::iter::{Enumerate, Map};

type Dimension = usize;
type Coordinate = usize;
type Length = usize;
type PointIx = usize;

pub type RotationPlane = (Dimension, Dimension);
pub type RotationDir = (Dimension, Dimension);

type Point = Vec<Coordinate>;
type Points = Vec<Point>;


#[derive(Debug, Clone)]
pub struct Lattice {
    pub dim: Dimension,
    pub length: Length,
    pub points: Points,
}


impl Lattice {

    fn create(dim: Dimension, length: Length, points: Points) -> Lattice {
        Lattice { dim: dim, length: length, points: points }
    }

    pub fn new(dim: Dimension, length: Length) -> Lattice {
        if dim == 0 { Lattice::create(0, length, vec![vec![]]) }
        else { Lattice::nonzero(dim, length) }
    }

    pub fn iter_pts(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn enum_pts(&self) -> Enumerate<Iter<Point>> {
        self.iter_pts().enumerate()
    }

    pub fn ix_of_point(&self, point: Point) -> PointIx {
        point.iter().enumerate().fold(0, |ix, (dimension_ix, coordinate)| {
            let inverted_dim_ix = (point.len() - dimension_ix - 1) as u32;
            let dim_multiplier = self.length.pow(inverted_dim_ix);
            ix + (coordinate * dim_multiplier)
        })
    }

    fn nonzero(dim: Dimension, length: Length) -> Lattice {
        let prior_lattice = Lattice::new(dim - 1, length);
        let points = Product::new(prior_lattice.points.iter(), (0..length))
            .map(|(coords, c)| {
                // There has to be a functional way to do this...
                let mut cs = coords.clone();
                cs.push(c);
                cs
            }).collect();
        Lattice::create(dim, length, points)
    }

}
