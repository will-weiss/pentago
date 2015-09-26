use pentago::color;
use pentago::color::Color;

fn rotate_coordinates(coords: &mut Vec<i32>, top_corner: &Vec<bool>, length: i32) {
    for (i, &swap) in top_corner.iter().enumerate() {
        if swap {
            coords[i] = length - 1 - coords[i];
        }
    }
}

fn spin_coordinates(coords: &mut Vec<i32>, spin: &Vec<usize>) {
    let prior_coords = coords.clone();
    for (i, &use_ix) in spin.iter().enumerate() {
        coords[i] = prior_coords[use_ix];
    }
}

#[derive(Debug)]
pub struct Square {
    pub coordinates: Vec<i32>,
    pub color: Color
}

impl Square {
    pub fn new(coordinates: Vec<i32>) -> Square {
        Square {
            coordinates: coordinates,
            color: Color::Empty
        }
    }

    pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: i32) -> Vec<i32> {
        let mut coords = self.coordinates.clone();
        rotate_coordinates(&mut coords, &top_corner, length);
        spin_coordinates(&mut coords, &spin);
        coords
    }
}
