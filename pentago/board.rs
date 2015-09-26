use pentago::quadrant;
use pentago::quadrant::Quadrant;


#[derive(Debug)]
pub struct Board {
    pub dim: i32,
    pub length: i32,
    pub orientation: i32,
    pub quadrants: Vec<Quadrant>
}

impl Board {
    pub fn new(dim: i32, length: i32) -> Board {
        let mut quadrants = Vec::<Quadrant>::new();
        for _ in 0..2i32.pow(dim as u32) {
            quadrants.push(Quadrant::new(dim, length));
        }
        Board {
            dim: dim,
            length: length,
            orientation: 0,
            quadrants: quadrants
        }
    }

}
