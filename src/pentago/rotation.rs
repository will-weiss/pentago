extern crate itertools;
use self::itertools::Product;

pub type RotationDir = [usize; 2];
pub type RotationPlane = [usize; 2];

pub type RotationDirs = Vec<RotationDir>;
pub type RotationPlanes = Vec<RotationPlane>;


pub fn get_all_rotation_planes(dim: usize) -> RotationPlanes {
    let mut rotation_planes = vec![];
    for d_i in 0..dim {
        for d_j in (1 + d_i)..dim {
            rotation_planes.push([d_i, d_j]);
        }
    }
    rotation_planes
}

pub fn get_all_rotation_dirs(rotation_planes: &RotationPlanes) -> RotationDirs {
    let mut rotation_dirs = vec![];
    for rotation_plane in rotation_planes {
        rotation_dirs.push(rotation_plane.clone());
        rotation_dirs.push([rotation_plane[1], rotation_plane[0]]);
    }
    rotation_dirs
}
