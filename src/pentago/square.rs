extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;
use pentago::color;
use pentago::color::Color;
use pentago::math_utils::{three_raised_to, mult2, mult3};

fn rotate_coordinates(coords: &mut Vec<u8>, top_corner: &Vec<bool>, length: &u8) {
    for (i, &swap) in top_corner.iter().enumerate() {
        if swap {
            coords[i] = length - 1 - coords[i];
        }
    }
}

fn spin_coordinates(coords: &mut Vec<u8>, spin: &Vec<usize>) {
    let prior_coords = coords.clone();
    for (i, &use_ix) in spin.iter().enumerate() {
        coords[i] = prior_coords[use_ix];
    }
}

fn invert_coordinates(coords: &mut Vec<u8>, length: &u8) {
    let prior_coords = coords.clone();
    for (i, &prior_coord) in prior_coords.iter().enumerate() {
        coords[i] = length - 1 - prior_coord;
    }
}

#[derive(Debug, Clone)]
pub struct Square {
    pub coordinates: Vec<u8>,
    pub color: Color
}

impl Square {
    // Generate a new square with assigned coordinates.
    pub fn new(coordinates: &Vec<u8>) -> Square {
        Square {
            coordinates: coordinates.clone(),
            color: Color::Black
        }
    }
    // Orient a square given a new top corner for the quadrant, some spin, and
    // a quadrant length
    pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: &u8)
        -> Square {
            let mut coordinates = self.coordinates.clone();
            rotate_coordinates(&mut coordinates, &top_corner, &length);
            spin_coordinates(&mut coordinates, &spin);
            Square {
                coordinates: coordinates,
                color: self.color.clone()
            }
        }

    // Get the coordinate exponent given a quadrant length.
    pub fn coordinate_exp(&self, length: &u8) -> u32 {
        self.coordinates.iter().enumerate().fold(0u32, |exp, (ix, &val)| {
            exp + ((val as u32) * (length.pow(ix as u32)) as u32)
        })
    }

    // Get a (big) integer representing the value of this square.
    pub fn val(&self, length: &u8) -> BigUint {

        // Return zero if the square is empty.
        if (self.color.is_empty()) {
            return BigUint::zero();
        }

        // Obtain the coordinate exponent given the length of the quadrant.
        let coordinate_exp = self.coordinate_exp(&length);

        // Obtain the base value for a non-empty square.
        let base_val = three_raised_to(coordinate_exp);

        // If the color is black, multiply the base value by two, otherwise
        // return the base value.
        if (self.color.is_black()) {
            mult2(base_val)
        } else {
            base_val
        }
    }
}
