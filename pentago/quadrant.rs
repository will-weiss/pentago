use pentago::square;
use pentago::square::Square;

fn gen_all_coordinates(dim: i32, length: i32) -> Vec<Vec<i32>> {
    let mut coords = vec![vec![]];
    for _ in 0..dim {
        let mut next_coords = vec![];
        for p in coords.iter() {
            for coord in 0..length {
                let mut c = p.clone();
                c.push(coord);
                next_coords.push(c);
            }
        }
        coords = next_coords;
    }
    coords
}


#[derive(Debug)]
pub struct Quadrant {
    pub dim: i32,
    pub length: i32,
    pub orientation: i32,
    pub squares: Vec<Square>
}

impl Quadrant {
    pub fn new(dim: i32, length: i32) -> Quadrant {
        let mut squares = Vec::<Square>::new();
        let all_coordinates = gen_all_coordinates(dim, length);
        for coordinates in all_coordinates  {
            squares.push(Square::new(coordinates));
        }
        Quadrant {
            dim: dim,
            length: length,
            orientation: 0,
            squares: squares
        }
    }
}
