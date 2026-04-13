use std::sync::mpsc::{Receiver, Sender};
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
    event_sender: Sender<ColorBox>,
    command_reciever: Receiver<Movement>,
}

impl Board {
    pub fn new(e_tx: Sender<ColorBox>, c_rx: Receiver<Movement>) -> Self {
        return Self {
            blocks: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
            curr_block: None,
            done: false,
            event_sender: e_tx,
            command_reciever: c_rx,
        };
    }

    pub fn start_game(&mut self) {
        while !self.done {
            self.next_move();
            let big_interval = Duration::from_millis(STARTING_SPEED_MS);
            let interval_count = 50;
            let small_interval = big_interval / interval_count;
            for _ in 0..interval_count {
                sleep(small_interval);
                let receive = self.command_reciever.try_recv();
                match receive {
                    Ok(next_command) => {
                        self.move_box(next_command);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    fn next_move(&mut self) {
        match self.curr_block {
            None => {
                self.curr_block = Some(Block::new(0, 0));
                self.update_board(vec![(0, 0)], Color::Red);
            }
            Some(ref mut block) => match block.move_down(self.blocks) {
                Ok((new_row, new_col)) => {
                    self.clean_box(new_row - 1, new_col);
                    self.update_board(vec![(new_row, new_col)], Color::Red);
                }
                Err(_) => {
                    self.curr_block = Some(Block::new(0, 0));
                    self.update_board(vec![(0, 0)], Color::Red);
                }
            },
        };
    }

    fn move_box(&mut self, movement: Movement) {
        match self.curr_block {
            Some(ref mut block) => match block.move_horizontal(movement) {
                Ok((new_r, new_c, old_c)) => {
                    self.clean_box(new_r, old_c);
                    self.update_board(vec![(new_r, new_c)], Color::Red);
                }
                Err(_) => {}
            },
            None => {}
        }
    }

    fn clean_box(&mut self, row: usize, col: usize) {
        self.update_board(vec![(row, col)], Color::Empty);
    }

    fn update_board(&mut self, positions: Vec<(usize, usize)>, color: Color) {
        for (row, col) in positions {
            self.blocks[row][col] = color;
        }
        self.event_sender.send(self.blocks);
    }
}
