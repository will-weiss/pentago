use std::rc::Rc;
use pentago::game_configuration::GameConfiguration;
use pentago::color::Color;


#[derive(Debug, Clone)]
pub struct Quadrant {
    pub cfg: Rc<GameConfiguration>,
    pub squares: Vec<Option<Color>>,
}

impl Quadrant {
    // Generate a new quadrant with some number of squares.
    pub fn new(cfg: Rc<GameConfiguration>) -> Quadrant {
        Quadrant {
            cfg: cfg.clone(),
            squares: (0..cfg.squares.len()).map(|_| {
                None
            }).collect(),
        }
    }

    // Place a stone of a given color at a square given by its index.
    pub fn place(&self, square_ix: usize, color: Color) -> Quadrant {
        Quadrant {
            cfg: self.cfg.clone(),
            squares: self.squares.iter().enumerate().map(|(ix, square)| {
                if (ix == square_ix) {
                    Some(color)
                } else {
                    square.clone()
                }
            }).collect(),
        }
    }

    pub fn rotate(&self, direction: usize) -> Quadrant {
        Quadrant {
            cfg: self.cfg.clone(),
            squares: (&self.squares).iter().enumerate().map(|(ix, square)| {
                self.squares[self.cfg.squares.rotate(ix, direction)].clone()
            }).collect(),
        }
    }


}

