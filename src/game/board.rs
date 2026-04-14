use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;

use crate::ColorBox;
use crate::game::block::Block;
use crate::general::commands::Command;
use crate::general::{
    colors::Color,
    constants::{BOX_HEIGHT, BOX_WIDTH, STARTING_SPEED_MS},
    movements::Movement,
};

#[derive(Debug)]
pub struct Board {
    pub blocks: ColorBox,
    pub curr_block: Option<Block>,
    pub done: bool,
    event_sender: Sender<ColorBox>,
    command_reciever: Receiver<Command>,
}

impl Board {
    pub fn new(e_tx: Sender<ColorBox>, c_rx: Receiver<Command>) -> Self {
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
                        self.handle_command(next_command);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::EndGame => {
                self.done = true;
            }
            Command::Move(movement) => {
                self.move_box(movement);
            }
        }
    }

    fn next_move(&mut self) {
        match self.curr_block {
            None => match Block::new(self.blocks) {
                Ok(block) => {
                    self.update_board(block.get_block_cells(), Color::Red);
                    self.curr_block = Some(block);
                }
                Err(_) => {
                    self.done = true;
                    return;
                }
            },
            Some(ref mut block) => match block.move_down(self.blocks) {
                Ok((new_row, new_col)) => {
                    // self.clean_box(new_row - 1, new_col);
                    // self.update_board(vec![(new_row, new_col)], Color::Red);
                }
                Err(_) => {
                    match Block::new(self.blocks) {
                        Ok(block) => {
                            self.curr_block = Some(block);
                        }
                        Err(_) => {
                            self.done = true;
                        }
                    }
                    self.update_board(vec![(0, 0)], Color::Red);
                }
            },
        };
    }

    fn move_box(&mut self, movement: Movement) {
        match self.curr_block {
            Some(ref mut block) => {
                let (prev_row, prev_col) = block.get_current_possition();

                match block.move_block(movement, self.blocks) {
                    Ok((new_r, new_c)) => {
                        self.clean_box(prev_row, prev_col);
                        self.update_board(vec![(new_r, new_c)], Color::Red);
                    }
                    Err(_) => {}
                }
            }
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
