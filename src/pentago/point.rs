extern crate itertools;
use self::itertools::Product;
use std::rc::Rc;
use pentago::lattice::Lattice;

#[derive(Debug, Clone)]
pub struct Point {
    pub lattice: Rc<Lattice>,
    pub ix: usize,
}

impl Point {

    pub fn rotate(&self, direction: usize) -> usize {
        self.lattice.rotations[direction][self.ix]
    }

    pub fn new(lattice: Rc<Lattice>, ix: usize) -> Point {
        Point {
            lattice: lattice,
            ix: ix
        }
    }

}
