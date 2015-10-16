extern crate itertools;

use self::itertools::Product;

use pentago::lattice::{Lattice, Dimension, Length, Point, PointIx};

use pentago::direction::*

type DimOrder = Vec<Dimension>;


pub type RotationDirs = Vec<RotationDir>;
pub type RotationPlanes = Vec<RotationPlane>;

pub type Spin = Vec<Dimension>;
pub type Spins = Vec<Spin>;

const QUADRANT_LENGTH: Length = 2;


pub type QuadrantIx = usize;
pub type PositionIx = usize;

pub type SquareIxs = (usize, usize);

pub type Line = Vec<SquareIxs>;

pub type Adjacency = Option<PointIx>;


pub struct Square {
    ixs: SquareIxs,
    point: Point,
}


pub struct BoardCfg {
    pub dim: Dimension,
    pub radius: Length,
    pub num_qs: usize,
    pub single_quadrant: Lattice,
    pub squares: Vec<Square>,
}


fn nonzero_spins(dim: Dimension) -> Vec<Spin> {
    let prior_dim = dim - 1;
    let mut all_spins = vec![];
    for prior_spin in get_all_spins(prior_dim).iter() {
        for loc in 0..dim {
            let mut next_spin = prior_spin.clone();
            next_spin.insert(loc, prior_dim);
            all_spins.push(next_spin)
        }
    }
    all_spins
}

fn get_rotation_planes(dim: Dimension) -> RotationPlanes {
    let mut rotation_planes = vec![];
    for d_i in 0..dim {
        for d_j in (1 + d_i)..dim {
            rotation_planes.push((d_i, d_j));
        }
    }
    rotation_planes
}

fn get_rotation_dirs(dim: Dimension) -> RotationDirs {
    let mut rotation_dirs = vec![];
    for &(d_i, d_j) in get_rotation_planes() {
        rotation_dirs.push((d_i, d_j));
        rotation_dirs.push((d_j, d_i));
    }
    rotation_dirs
}

fn get_all_spins(dim: Dimension) -> Vec<Spin> {
    if dim == 0 { vec![vec![]] }
    else { nonzero_spins(dim) }
}

pub fn get_rotations(lattice: &Lattice, rotation_dirs: &RotationDirs) -> Vec<Vec<PointIx>> {
    lattice.points.iter_pts().map(|pt| {
        rotation_dirs.iter().map(|&(d_i, d_j)| {
            let mut rotated_point = lattice.points[ix].clone();
            rotated_point[d_i] = lattice.length - 1 - point[d_j];
            rotated_point[d_j] = point[d_i];
            lattice.ix_of_point(rotated_point)
        }).collect()
    }).collect()
}

fn all_squares(dim: Dimension, radius: Length) -> Vec<Square> {
    let single_quadrant = Lattice::new(dim, radius);
    let quadrants = Lattice::new(dim, QUADRANT_LENGTH);

    Product::new(
        single_quadrant.enum_pts(),
        quadrants.enum_pts()
    ).map(|((q_ix, q_pt), (p_ix, p_pt))| {
        Square {
            ixs: (q_ix, p_ix),
            point: q_pt.iter().zip(p_pt).map(|(q_coordinate, p_coordinate)| {
                p_coordinate + (q_coordinate * radius)
            }).collect()
        }
    })
}


pub type Spinner {
    pub single_quadrant_rotations: Vec<PositionIx>,
    pub whole_board_rotations: Vec<Vec<Vec<SquareIxs>>>,
}


impl Spinner {
    pub fn new(dim: Dimension, radius: Length) -> Spinner {

        let rotation_planes = get_all_rotation_planes(dim);
        let rotation_dirs = get_all_rotation_dirs(&rotation_planes);
        let spins = get_all_spins(dim);

        let squares = all_squares(dim, radius);

        let single_quadrant_rotations = squares.iter().enumerate().map(|pt| {
            rotation_dirs.iter().map(|&(d_i, d_j)| {
                let mut rotated_point = lattice.points[ix].clone();
                rotated_point[d_i] = lattice.length - 1 - point[d_j];
                rotated_point[d_j] = point[d_i];
                lattice.ix_of_point(rotated_point)
            }).collect()
        }).collect()




        let mut whole_board_rotations = vec![vec![vec![]; cfg.single_quadrant.len()]; cfg.num_qs];

        get_rotations(cfg.squares).iter().enumerate().map(|ix, sq| {

        })


        .map(|(rd_ix, rotations)| {
            rotations.iter().enumerate().map(|(pt_ix, rotated_pt_ix)| {
                let (q_ix, psn_ix) = cfg.to_square(pt_ix);
                whole_board_rotations[q_ix][psn_ix][rd_ix] = cfg.to_square(rotated_pt_ix);
            }).collect()
        });

        Spinner {
            single_quadrant_rotations: single_quadrant_rotations,
            whole_board_rotations: whole_board_rotations
        }
    }
}


pub struct LineCfg {
    pub all_lines: Vec<Line>,
    pub lines_by_square: Vec<Vec<Vec<Line>>>,
}


impl LineCfg {
    pub fn new(cfg: &BoardCfg) -> LineCfg {
        let line_dirs = get_all_line_directions(cfg.dim);
        let mut all_lines = vec![];
        let mut lines_by_ix = vec![vec![vec![]; cfg.quadrants.len()]; cfg.single_quadrant.len()];

        cfg.whole_board.enum_pts().map(|(point_ix, point)| {
            for ld in line_dirs {
                for (i, dim_dir)

                adj_cs: Vec<i32> = pt.coordinates.iter()
                    .zip(ld).map(|(coordinate, dim_dir)| {
                        (coordinate.clone() as i32) + dim_dir.as_i32()
                    }).collect();
            }


        match *self {
            Null => 0,
            Forward => 1,
            Backward => -1
        }
    }


        }).collect();
    }
}


fn coordinates_out_of_bounds(lattice: &Lattice, cs: &Vec<i32>) -> bool {
    !cs.iter().all(|&coordinate|
        coordinate >= 0 && coordinate < (lattice.length as i32)
    )
}

fn maybe_adj_coordinates(lattice: &Lattice, pt: &Point, ld: &LineDir) -> Option<Coordinates> {
    let adj_cs: Vec<i32> = pt.coordinates.iter()
        .zip(ld).map(|(coordinate, dim_dir)| {
            (coordinate.clone() as i32) + dim_dir.as_i32()
        }).collect();
    if coordinates_out_of_bounds(lattice, &adj_cs) { None }
    else { Some(adj_cs.iter().map(|c| *c as usize).collect()) }
}

fn get_square_adjacencies(&self) -> Vec<Vec<Adjacency>> {
    let whole_board = self.lattice(self.diameter);

    let mut square_adjacencies =
        vec![vec![None; self.squares.len()]; self.line_directions.len()];

    for (sq_ix, (ld_ix, ld)) in Product::new(
        0..self.squares.len(),
        self.line_directions.iter().enumerate()
    ) {
        let maybe_adj_cs = self.maybe_adj_coordinates(&whole_board[sq_ix], ld);
        if let Some(adj_cs) = maybe_adj_cs {
            let adj_ix = coordinates_to_ix(adj_cs, self.diameter);
            square_adjacencies[ld_ix][sq_ix] = Some(adj_ix);
        }
    }

    square_adjacencies

}

fn to_line(&self, line_ixs: &Vec<usize>) -> Line {
    line_ixs.iter().map(|&ix| self.squares[ix].clone()).collect()
}

fn add_lines_from_adjacencies(&mut self, adjs: &Vec<Adjacency>, ix: usize) -> &mut Self {
    let mut line_ixs: Vec<usize> = vec![];
    let mut ref_ix = ix;
    for _ in 1..self.victory {
        match adjs[ref_ix] {
            None => { return self },
            Some(next_ix) => {
                line_ixs.push(next_ix);
                ref_ix = next_ix;
            }
        }
    }

    line_ixs.push(ref_ix);
    let line = self.to_line(&line_ixs);

    self.all_lines.push(line.clone());

    for ix in line_ixs {
        self.lines_by_ix[ix].push(line.clone());
    }

    self
}

fn add_lines(&mut self) {
    self.lines_by_ix = vec![vec![]; self.squares.len()];

    for adjs in self.get_square_adjacencies().iter() {
        for ix in 0..adjs.len() {
            self.add_lines_from_adjacencies(adjs, ix);
        }
    }
}

impl LineCfg {

    pub fn new() -> LineCfg {
        let line_directions = get_all_line_directions(cfg.dim);

    }


}

