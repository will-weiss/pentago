use pentago::color;
use pentago::color::Color;

fn rotate_coordinates(coords: &mut Vec<i32>, x_plane: usize, y_plane: usize, rotations: i32, length: i32) {
    let mut x = coords[x_plane];
    let mut y = coords[y_plane];
    for _ in 0..rotations {
        let next_x = y;
        y = length - x - 1;
        x = next_x;
    }
    coords[x_plane] = x;
    coords[y_plane] = y;
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
}
