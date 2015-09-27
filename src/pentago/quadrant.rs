extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::square;
use pentago::square::Square;
use pentago::math_utils::{three_raised_to, mult2, mult3};

fn gen_all_coordinates(dim: u8, length: u8) -> Vec<Vec<u8>> {
    let mut coords = vec![vec![]];
    for _ in 0..dim {
        let mut next_coords = vec![];
        for p in coords.iter() {
            for coord in 0..length {
                let mut c = p.clone();
                c.push(coord);
                next_coords.push(c);
            }
        }
        coords = next_coords;
    }
    coords
}

#[derive(Debug, Clone)]
pub struct Quadrant {
    pub coordinates: Vec<bool>,
    pub squares: Vec<Square>
}

impl Quadrant {
    // Generate a new quadrant with assigned coordinates for some dimesion and
    // length.
    pub fn new(coordinates: &Vec<bool>, dim: u8, length: u8) -> Quadrant {
        Quadrant {
            coordinates: coordinates.clone(),
            squares: gen_all_coordinates(dim, length).iter().map(|coords| {
                Square::new(coords)
            }).collect()
        }
    }

    // Return a new Quadrant from this quadrant by applying an orientation to
    // each square.
    pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: &u8)
        -> Quadrant {
            Quadrant {
                coordinates: self.coordinates.clone(),
                squares: self.squares.iter().map(|square| {
                    square.orient(&top_corner, &spin, length)
                }).collect()
            }
        }

    // The unique number of a given quadrant given its coordinates.
    pub fn numbering(&self) -> u32 {
        self.coordinates.iter().enumerate()
            .fold(0u32, |quadrant_numbering, (coordinate_ix, &coordinate_val)| {
                if (coordinate_val) {
                    quadrant_numbering + 2u32.pow(coordinate_ix as u32)
                } else {
                    quadrant_numbering
                }
            })
    }

    // The (big) integer value representing the value of the squares of the
    // quadrant given their coordinates.
    pub fn val(&self, length: &u8, quadrant_size: &u32) -> BigUint {
        // Keep a tally of the value of the squares, initially zero.
        let mut squares_val = BigUint::zero();

        // Iterate over the squares, adding the value of each square to the
        // running tally.
        for square in self.squares.iter() {
            squares_val = squares_val + square.val(length);
        }

        // If the total value of the squares is zero, immediately return.
        if (squares_val.is_zero()) {
            return squares_val;
        }

        //
        let quadrant_exponent = self.numbering() * quadrant_size;

        let quadrant_val = three_raised_to(quadrant_exponent);

        squares_val * quadrant_val
    }
}
