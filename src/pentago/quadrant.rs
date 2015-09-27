extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::square;
use pentago::square::Square;
use pentago::math_utils::{three_raised_to, mult2, mult3};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Quadrant {
    pub squares: HashMap<usize, Square>
}

impl Quadrant {
    // Generate a new quadrant with assigned coordinates for some dimesion and
    // length.
    pub fn new(quadrant_size: usize) -> Quadrant {

        let mut squares = HashMap::with_capacity(quadrant_size);

        for square_ix in 0..quadrant_size {
            squares.insert((square_ix), Square::new());
        }

        Quadrant {
            squares: squares
        }
    }

    // Get a (big) integer representing the value of this square.
    pub fn val(&self) -> BigUint {
        let mut val = BigUint::zero();

        for (ix, square) in &self.squares {
            if square.color.is_empty() {
                continue;
            }
            let mut add_val = three_raised_to(*ix);
            if (square.color.is_black()) {
                add_val = mult2(add_val)
            }
            val = val + add_val;
        }

        val
    }

    // // Return a new Quadrant from this quadrant by applying an orientation to
    // // each square.
    // pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: &u8)
    //     -> Quadrant {
    //         Quadrant {
    //             squares: self.squares.iter().map(|square| {
    //                 square.orient(&top_corner, &spin, length)
    //             }).collect()
    //         }
    //     }


    // // The (big) integer value representing the value of the squares of the
    // // quadrant given their coordinates.
    // pub fn val(&self, length: &u8, quadrant_size: &u32) -> BigUint {
    //     // Keep a tally of the value of the squares, initially zero.
    //     let mut squares_val = BigUint::zero();

    //     // Iterate over the squares, adding the value of each square to the
    //     // running tally.
    //     for square in self.squares.iter() {
    //         squares_val = squares_val + square.val(length);
    //     }

    //     // If the total value of the squares is zero, immediately return.
    //     if (squares_val.is_zero()) {
    //         return squares_val;
    //     }

    //     //
    //     let quadrant_exponent = self.numbering() * quadrant_size;

    //     let quadrant_val = three_raised_to(quadrant_exponent);

    //     squares_val * quadrant_val
    // }

    // SQUARE

    // // Orient a square given a new top corner for the quadrant, some spin, and
    // // a quadrant length
    // pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: &u8)
    //     -> Square {
    //         let mut coordinates = self.coordinates.clone();
    //         rotate_coordinates(&mut coordinates, &top_corner, &length);
    //         spin_coordinates(&mut coordinates, &spin);
    //         Square {
    //             coordinates: coordinates,
    //             color: self.color.clone()
    //         }
    //     }

    // // Get the coordinate exponent given a quadrant length.
    // pub fn coordinate_exp(&self, length: &u8) -> u32 {
    //     self.coordinates.iter().enumerate().fold(0u32, |exp, (ix, &val)| {
    //         exp + ((val as u32) * (length.pow(ix as u32)) as u32)
    //     })
    // }

}


// fn rotate_coordinates(coords: &mut Vec<u8>, top_corner: &Vec<bool>, length: &u8) {
//     for (i, &swap) in top_corner.iter().enumerate() {
//         if swap {
//             coords[i] = length - 1 - coords[i];
//         }
//     }
// }

// fn spin_coordinates(coords: &mut Vec<u8>, spin: &Vec<usize>) {
//     let prior_coords = coords.clone();
//     for (i, &use_ix) in spin.iter().enumerate() {
//         coords[i] = prior_coords[use_ix];
//     }
// }

// fn invert_coordinates(coords: &mut Vec<u8>, length: &u8) {
//     let prior_coords = coords.clone();
//     for (i, &prior_coord) in prior_coords.iter().enumerate() {
//         coords[i] = length - 1 - prior_coord;
//     }
// }
