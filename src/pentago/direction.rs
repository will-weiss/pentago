pub use self::DimDir::*;

pub type LineDir = Vec<DimDir>;

#[derive(Debug, Clone, Copy)]
pub enum DimDir {
    Null,
    Forward,
    Backward
}


fn init_line_direction(prior_dim: usize) -> LineDir {
    let mut ld = vec![Null; prior_dim];
    ld.push(Forward);
    ld
}

fn nonzero_line_directions(prior_dim: usize) -> Vec<LineDir> {
    let mut next_directions = vec![init_line_direction(prior_dim)];
    for prior_dir in get_all_line_directions(prior_dim).iter() {
        for dir in [Null, Forward, Backward].iter() {
            let mut next_dir = prior_dir.clone();
            next_dir.push(dir.clone());
            next_directions.push(next_dir);
        }
    }
    next_directions
}

pub fn get_all_line_directions(dim: usize) -> Vec<LineDir> {
    if dim == 0 { vec![] }
    else { nonzero_line_directions(dim - 1) }
}
