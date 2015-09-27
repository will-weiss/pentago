
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
