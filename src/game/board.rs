use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::sleep;
use std::time::Duration;

use crate::ColorBox;
use crate::game::block::Block;
use crate::general::{
    colors::Color,
    dimensions::{BOX_HEIGHT, BOX_WIDTH},
    movements::Movement,
    speed::STARTING_SPEED_MS,
};

#[derive(Debug)]
pub struct Board {
    pub blocks: ColorBox,
    pub curr_block: Option<Block>,
    pub done: bool,
    command_reciever: Receiver<Movement>,
}

impl Board {
    pub fn new(c_rx: Receiver<Movement>) -> Self {
        return Self {
            blocks: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
            curr_block: None,
            done: false,
            command_reciever: c_rx,
        };
    }

    pub fn start_game(&mut self, mut f: impl FnMut(ColorBox) -> ()) {
        while !self.done {
            self.next_move();
            let second = Duration::from_millis(STARTING_SPEED_MS);
            f(self.blocks);
            sleep(second);
            let receive = self.command_reciever.try_recv();
            match receive {
                Ok(next_command) => {
                    self.move_box(next_command);
                }
                Err(_) => {}
            }
        }
    }

    pub fn next_move(&mut self) {
        match self.curr_block {
            None => {
                self.curr_block = Some(Block::new(0, 0));
                self.blocks[0][0] = Color::Red;
            }
            Some(ref mut block) => match block.move_down(self.blocks) {
                Ok((new_row, new_col)) => {
                    self.clean_box(new_row - 1, new_col);
                    self.blocks[new_row][new_col] = Color::Red;
                }
                Err(_) => {
                    self.curr_block = Some(Block::new(0, 0));
                }
            },
        };
    }

    pub fn move_box(&mut self, movement: Movement) {
        let (row, prev_col) = self.curr_block;
        let mut col = prev_col.clone();
        match movement {
            Movement::Left => {
                if col == 0 {
                    return;
                };
                col -= 1;
            }
            Movement::Right => {
                if col == dimensions::BOX_WIDTH - 1 {
                    return;
                }
                col += 1;
            }
        }

        self.curr_block = (row, col);
    }

    pub fn clean_box(&mut self, row: usize, col: usize) {
        self.blocks[row][col] = Color::Empty;
    }
}
