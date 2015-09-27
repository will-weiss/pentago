#[derive(Debug, Clone)]
pub enum Color {
    Empty,
    Black,
    White
}

impl Color {
    pub fn val(&self) -> u8 {
        match self {
            &Color::Empty => 0,
            &Color::Black => 1,
            &Color::White => 2
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &Color::Empty => true,
            _ => false
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            &Color::Black => true,
            _ => false
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            &Color::White => true,
            _ => false
        }
    }
}
