extern crate num;
extern crate itertools;

use std::rc::Rc;
use self::num::traits::{Zero, One};
use self::num::bigint::BigUint;
use self::itertools::{Product, Zip};

use pentago::state::State;
use pentago::coordinates::Coordinates;
use pentago::lattice::{LatticeBuilder, Point, Lattice};
use pentago::math_utils::{three_raised_to, mult2, mult3};
use pentago::color::Color;


#[derive(Debug, Clone, Copy)]
pub enum DimDir {
    Null,
    Forward,
    Backward
}

pub type LineDir = Vec<DimDir>;

fn init_line_direction(prior_dim: usize) -> LineDir {
    let mut ld = vec![DimDir::Null; prior_dim];
    ld.push(DimDir::Forward);
    ld
}

fn non_negative_line_directions(prior_dim: usize) -> Vec<LineDir> {
    let mut next_directions = vec![init_line_direction(prior_dim)];
    for prior_dir in get_all_line_directions(prior_dim).iter() {
        for dir in [DimDir::Null, DimDir::Forward, DimDir::Backward].iter() {
            let mut next_dir = prior_dir.clone();
            next_dir.push(dir.clone());
            next_directions.push(next_dir);
        }
    }
    next_directions
}

pub fn get_all_line_directions(dim: usize) -> Vec<LineDir> {
    match dim {
        0 => vec![],
        _ => non_negative_line_directions(dim - 1)
    }
}
