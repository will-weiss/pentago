use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White
}

pub type Space = Option<Color>;

pub type Quadrant = Vec<Space>;
pub type QuadrantRef = Rc<Quadrant>;
pub type Board = Vec<QuadrantRef>;


fn init_quadrant(sq_len: usize) -> Quadrant {
    vec![None; sq_len]
}

pub fn init_board(qs_len: usize, sq_len: usize) -> Board {
    vec![Rc::new(init_quadrant(sq_len)); qs_len]
}
