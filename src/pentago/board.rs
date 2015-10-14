use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White
}

pub type Quadrant = Vec<Option<Color>>;
pub type Board = Vec<Rc<Quadrant>>;


fn init_quadrant(sq_len: usize) -> Quadrant {
    (0..sq_len).map(|_| None).collect()
}

pub fn init_board(qs_len: usize, sq_len: usize) -> Board {
    (0..qs_len).map(|_| Rc::new(init_quadrant(sq_len))).collect()
}
