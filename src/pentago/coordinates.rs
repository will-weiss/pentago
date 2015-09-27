use std::rc::Rc;
use pentago::coordinate_utils::{get_coordinates, apply_rotation, get_rotations};

#[derive(Debug, Clone)]
pub struct Coordinates {
    dim: usize,
    length: usize,
    coordinates: Vec<usize>
}

impl Coordinates {

    pub fn all(dim: usize, length: usize) -> Vec<Coordinates> {
        get_coordinates(dim, length).iter().map(|c| {
            Coordinates {
                dim: dim,
                length: length,
                coordinates: c.clone()
            }
        }).collect()
    }

    pub fn to_usize(&self) {

    }

}
