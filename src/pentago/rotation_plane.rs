extern crate itertools;
use self::itertools::Product;

pub type RotationPlane = [usize; 2];
pub type RotationPlanes = Vec<RotationPlane>;


pub fn get_all_rotation_planes(dim: usize) -> RotationPlanes {
    Product::new((0..dim), (0..dim))
        .filter(|&(d_i, d_j)| d_i != d_j )
        .map(|(d_i, d_j)| [d_i, d_j] )
        .collect()
}
