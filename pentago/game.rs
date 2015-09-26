use pentago::board;
use pentago::board::Board;


#[derive(Debug)]
pub struct Game {
    pub dim: i32,
    pub length: i32,
    pub victory: i32,
    pub board: Board
}

impl Game {
    pub fn new(dim: i32, length: i32, victory: i32) -> Game {
        Game {
            dim: dim,
            length: length,
            victory: victory,
            board: Board::new(dim, length)
        }
    }
}
