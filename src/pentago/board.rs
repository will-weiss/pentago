extern crate num;

use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use pentago::quadrant;
use pentago::quadrant::Quadrant;


fn with_bool(p: &Vec<bool>, b:bool) -> Vec<bool> {
    let mut c = p.clone();
    c.push(b);
    c
}

fn gen_all_coordinates(dim: u8) -> Vec<Vec<bool>> {
    let mut coords = vec![vec![]];
    for _ in 0..dim {
        let mut next_coords = vec![];
        for p in coords.iter() {
            next_coords.push(with_bool(&p, true));
            next_coords.push(with_bool(&p, false));
        }
        coords = next_coords;
    }
    coords
}

#[derive(Debug, Clone)]
pub struct Board {
    pub dim: u8,
    pub length: u8,
    pub quadrants: Vec<Quadrant>
}

impl Board {
    pub fn new(dim: u8, length: u8) -> Board {
        Board {
            dim: dim,
            length: length,
            quadrants: gen_all_coordinates(dim).iter().map(|coordinates| {
                Quadrant::new(coordinates, dim, length)
            }).collect()
        }
    }

    pub fn orient(&self, quadrant_ix: usize, top_corner: &Vec<bool>, spin: &Vec<usize>) -> Board {
        Board {
            dim: self.dim,
            length: self.length,
            quadrants: self.quadrants.iter().enumerate().map(|(ix, quadrant)| {
                if (ix == quadrant_ix) {
                    quadrant.orient(&top_corner, &spin, &self.length)
                } else {
                    quadrant.clone()
                }
            }).collect()
        }
    }

    pub fn quadrant_size(&self) -> u32 {
        (self.length as u32).pow(self.dim as u32)
    }

    pub fn val(&self) -> BigUint {
        let quadrant_size = self.quadrant_size();
        self.quadrants.iter().fold(BigUint::zero(), |val, quadrant| {
            val + quadrant.val(&self.length, &quadrant_size)
        })
    }

}
