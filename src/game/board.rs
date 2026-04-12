use std::thread::sleep;
use std::time::Duration;

use crate::ColorBox;
use crate::general::dimensions;
use crate::general::{
    colors::Color,
    dimensions::{BOX_HEIGHT, BOX_WIDTH},
    movements::Movement,
    speed::STARTING_SPEED_MS,
};

#[derive(Default, Debug)]
pub struct Board {
    pub blocks: ColorBox,
    pub curr_block: (usize, usize),
    pub done: bool,
}

impl Board {
    pub fn new() -> Self {
        return Self {
            blocks: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
            curr_block: (0, 0),
            done: false,
        };
    }

    pub fn start_game(&mut self, mut f: impl FnMut(ColorBox) -> ()) {
        while !self.done {
            self.next_move();
            let second = Duration::from_millis(STARTING_SPEED_MS);
            f(self.blocks);
            sleep(second);
        }
    }

    pub fn next_move(&mut self) {
        let (r, c) = self.curr_block;
        if r == BOX_HEIGHT || !matches!(self.blocks[r][c], Color::Empty) {
            self.curr_block = (0, 0);
            // self.done = true;
            return;
        }
        self.blocks[r][c] = Color::Red;
        if r > 0 {
            self.blocks[r - 1][c] = Color::Empty;
        }

        self.curr_block = (r + 1, c);
    }

    pub fn move_box(&mut self, movement: Movements) {
        let (row, mut prev_col) = self.curr_block;
        let mut col = prev_col.clone();
        match movement {
            Movements::Left => {
                col -= 1;
            }
            Movements::Right => {
                col += 1;
            }
        }

        // let is_near_border = col < 0 || col == dimensions::BOX_WIDTH;
        // if (is_near_border) {
        //     return;
        // }

        self.curr_block = (row, col);
    }
}
