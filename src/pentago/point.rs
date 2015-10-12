extern crate itertools;
use self::itertools::Product;
use std::rc::{self, Rc};
use pentago::lattice::Lattice;

#[derive(Debug, Clone)]
pub struct Point {
    pub rotations: Vec<Rc<Point>>,
    pub coordinates: Vec<usize>,
    pub ix: usize,
}

impl Point {

    // pub fn rotate(&self, direction: usize) -> usize {
    //     self.lattice.rotations[direction][self.ix]
    // }

    pub fn rotate(&self, direction: usize) -> usize {
        self.ix
    }

    pub fn val(&self) -> usize {
        self.lattice.coordinates_val(&self.coordinates)
    }

    fn get_rotations(&self) -> Vec<usize> {
        let rotation_planes = self.lattice.rotation_planes.clone();
        (&rotation_planes).iter().map(|rotation_plane| {
            let rotated_cs = self.apply_rotation(&rotation_plane);
            self.lattice.coordinates_val(&rotated_cs)
        }).collect()
    }

    pub fn apply_rotation(&self, rotation_plane: &[usize; 2]) -> Vec<usize> {
        let d_i = rotation_plane[0];
        let d_j = rotation_plane[1];
        let mut rotated_coordinates = self.coordinates.clone();
        rotated_coordinates[d_i] = self.lattice.length - 1 - self.coordinates[d_j];
        rotated_coordinates[d_j] = self.coordinates[d_i];
        rotated_coordinates
    }

    fn init_points(lattice: Lattice) -> Vec<Point> {
        let l = Rc::new(lattice);

        (&l.coordinates).iter().enumerate().map(|(ix, c)| {
            Point {
                lattice: l.clone(),
                coordinates: c.clone(),
                ix: ix,
                rotations: vec![]
            }
        }).collect()
    }

    fn rs(points: &Vec<Point>) -> Vec<Vec<usize>> {
        (points).iter().map(|point| {
            point.get_rotations()
        }).collect()
    }

        // let mut point_refs = points.clone();

        // for (ix, rs) in rotations.iter().enumerate() {
        //     let point = rc::get_mut(&mut point_refs[ix]);
        //     for r in rs {
        //         println!("{:?}", r);
        //     }
        // }

        // let xx: Vec<Vec<usize>> = rotations.iter().enumerate().map(|(ix, rs)| {
        //     println!("{:?}", rs);
        //     rs.clone()
        // }).collect();

        // Product::new(points.iter_mut(), ).map(|(point, rs)| {
        //     println!("{:?}", rs);
        // }).collect();

    pub fn from(lattice: Lattice) -> Vec<Point> {
        let points = Point::init_points(lattice);
        let rotations = Point::rs(&points);

        let pp = vec![];

        for (ix, rs) in rotations.iter().enumerate() {
            for r in rs.clone() {
                let rp = points[r];
                pp.push(rp);
            }
        }

        println!("{:?}", pp);

        points

        // points.iter().map(|point| {
        //     Rc::new(point.clone())
        // }).collect()


    }

}
