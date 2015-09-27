extern crate num;

use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::quadrant;
use pentago::quadrant::Quadrant;
use std::collections::HashMap;
use pentago::math_utils::{three_raised_to, mult2, mult3};

#[derive(Debug, Clone)]
pub struct Board {
    pub quadrants: HashMap<usize, Quadrant>
}

impl Board {
    pub fn new(num_quadrants: usize, quadrant_size: usize) -> Board {
        let mut quadrants = HashMap::with_capacity(num_quadrants as usize);

        for quadrant_ix in 0..num_quadrants {
            quadrants.insert((quadrant_ix as usize), Quadrant::new(quadrant_size));
        }

        Board {
            quadrants: quadrants
        }
    }

    // Get a (big) integer representing the value of this square.
    pub fn val(&self, quadrant_size: &usize) -> BigUint {
        let mut val = BigUint::zero();

        for (ix, quadrant) in &self.quadrants {
            let mut quadrant_val = quadrant.val();
            if (quadrant_val.is_zero()) {
                continue;
            }
            if (ix != &0) {
                quadrant_val = quadrant_val * (three_raised_to((*ix) * (*quadrant_size)));
            }
            val = val + quadrant_val;
        }

        val
    }


    // pub fn orient(&self, quadrant_ix: usize, top_corner: &Vec<bool>, spin: &Vec<usize>) -> Board {
    //     Board {
    //         dim: self.dim,
    //         length: self.length,
    //         quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
    //             if (ix == quadrant_ix) {
    //                 quadrant.orient(&top_corner, &spin, &self.length)
    //             } else {
    //                 quadrant.clone()
    //             }
    //         }).collect()
    //     }
    // }

    // pub fn quadrant_size(&self) -> u32 {
    //     (self.length as u32).pow(self.dim as u32)
    // }


    // // The unique number of a given quadrant given its coordinates.
    // pub fn numbering(&self) -> u32 {
    //     self.coordinates.iter().enumerate()
    //         .fold(0u32, |quadrant_numbering, (coordinate_ix, &coordinate_val)| {
    //             if (coordinate_val) {
    //                 quadrant_numbering + 2u32.pow(coordinate_ix as u32)
    //             } else {
    //                 quadrant_numbering
    //             }
    //         })
    // }

    // pub fn val(&self) -> BigUint {
    //     let quadrant_size = self.quadrant_size();
    //     self.quadrants.iter().fold(BigUint::zero(), |val, quadrant| {
    //         val + quadrant.val(&self.length, &quadrant_size)
    //     })
    // }

}
