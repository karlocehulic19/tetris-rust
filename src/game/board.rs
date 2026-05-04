use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;

use crate::ColorBox;
use crate::game::block::{Block, CellPosition};
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
        // TODO: handle unwrap properly (This will be the error that causes game to end)
        let should_create_new_block = self.curr_block.is_none()
            || self
                .curr_block
                .as_mut()
                .unwrap()
                .move_down(self.blocks)
                .is_err();
        if should_create_new_block {
            self.curr_block = Some(Block::new(self.blocks).unwrap());
        }

        let (prev_cells, next_cells) = match self.curr_block.as_ref() {
            Some(block) => (
                block.get_prev_block_cells().clone(),
                block.get_block_cells(),
            ),
            None => {
                return;
            }
        };
        if let Some(ref prev_cells) = prev_cells {
            self.clean_box(prev_cells);
        }
        self.update_board(&next_cells, Color::Red);
    }

    fn move_box(&mut self, movement: Movement) {
        match self.curr_block {
            Some(ref mut block) => match block.move_block(movement, self.blocks) {
                Ok(new_pos) => {
                    if let Some(prev_cells) = block.get_prev_block_cells() {
                        self.clean_box(&prev_cells);
                    }
                    self.update_board(&new_pos, Color::Red);
                }
                Err(_) => {}
            },
            None => {}
        }
    }

    fn clean_box(&mut self, cells: &Vec<(usize, usize)>) -> &mut Board {
        self.update_board(cells, Color::Empty);
        return self;
    }

    fn update_board(&mut self, positions: &Vec<(usize, usize)>, color: Color) {
        for (row, col) in positions {
            self.blocks[row.clone()][col.clone()] = color;
        }
        self.event_sender.send(self.blocks);
    }
}
