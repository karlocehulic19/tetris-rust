use crate::ColorBox;
use crate::general::{
    colors::Color,
    dimensions::{BOX_HEIGHT, BOX_WIDTH},
};

pub struct Board {
    pub blocks: ColorBox,
}

impl Board {
    pub fn new() -> Self {
        return Self {
            blocks: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
        };
    }

    pub fn place_block(&mut self) {
        self.blocks[3][3] = Color::Green;
        self.blocks[4][3] = Color::Red;
        self.blocks[5][5] = Color::Blue;
        self.blocks[6][5] = Color::Yellow;
    }
}
