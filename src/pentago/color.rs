#[derive(Debug, Clone)]
pub enum Color {
    Black,
    White
}

impl Color {
    pub fn is_black(&self) -> bool {
        match self {
            &Color::Black => true,
            _ => false
        }
    }
}
