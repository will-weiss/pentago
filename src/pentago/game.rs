use std::rc::Rc;
use pentago::board;
use pentago::board::Board;


#[derive(Debug, Clone)]
pub struct Game {
    pub victory: u8,
    pub board: Board
}

impl Game {
    pub fn new(dim: u8, length: u8, victory: u8) -> Game {
        Game {
            victory: victory,
            board: Board::new(dim, length)
        }
    }
}
