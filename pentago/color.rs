#[derive(Debug)]
pub enum Color {
    Empty,
    Black,
    White
}


impl Color {
    pub fn val(self) -> i32 {
        match self {
            Color::Empty => 0,
            Color::Black => 1,
            Color::White => 2
        }
    }

    pub fn is_empty(self) -> bool {
        match self {
            Color::Empty => true,
            _ => false
        }
    }
}
