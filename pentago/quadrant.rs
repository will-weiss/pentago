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
            orientation: 0,
            squares: squares
        }
    }

    pub fn orient(&self, top_corner: &Vec<bool>, spin: &Vec<usize>, length: i32) -> Vec<Vec<i32>> {
        let mut all_coords = Vec::<Vec<i32>>::new();
        for square in self.squares.iter() {
            let coords = square.orient(&top_corner, &spin, length);
            all_coords.push(coords);
        }
        all_coords
    }
}
