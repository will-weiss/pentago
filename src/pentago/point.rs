extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::lattice::Lattice;

#[derive(Debug, Clone)]
pub struct Point {
    lattice: Rc<Lattice>,
    pub ix: usize,
}

impl Point {
    pub fn rotate(&self, rotation_ix: usize) -> Point {
        Point {
            lattice: self.lattice.clone(),
            ix: self.lattice.rotations[rotation_ix][self.ix],
        }
    }

    pub fn new(lattice: Rc<Lattice>, ix: usize) -> Point {
        Point {
            lattice: lattice,
            ix: ix
        }
    }

}
