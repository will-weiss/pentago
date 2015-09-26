use std::rc::Rc;
use pentago::board;
use pentago::board::Board;


#[derive(Debug)]
pub struct Game {
    pub victory: i32,
    pub board: Board
}

impl Game {
    pub fn new(dim: i32, length: i32, victory: i32) -> Game {
        Game {
            victory: victory,
            board: Board::new(dim, length)
        }
    }
}