use crate::general::{
    colors::Color,
    constants::{BOX_HEIGHT, BOX_WIDTH},
    movements::Movement,
    types::ColorBox,
};

#[derive(Debug)]
pub enum BlockError {
    Grounded,
    OutOfBounds,
}

#[derive(Debug)]
pub struct Block {
    row: usize,
    col: usize,
}

impl Block {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row: row, col: col }
    }

    pub fn move_block(
        &mut self,
        movement: Movement,
        board: ColorBox,
    ) -> Result<(usize, usize), BlockError> {
        match movement {
            Movement::Left => {
                if self.col == 0 {
                    return Err(BlockError::OutOfBounds);
                }
                self.col -= 1;
                return Ok((self.row, self.col));
            }
            Movement::Right => {
                if self.col == BOX_WIDTH - 1 {
                    return Err(BlockError::OutOfBounds);
                };
                self.col += 1;
                return Ok((self.row, self.col));
            }
            Movement::Down => self.move_down(board),
        }
    }

    pub fn move_down(&mut self, board: ColorBox) -> Result<(usize, usize), BlockError> {
        if self.is_grounded(board) {
            return Err(BlockError::Grounded);
        }

        self.row += 1;
        return Ok((self.row, self.col));
    }

    // can either do this, and then have to call move down, or try to move down in the first place
    fn is_grounded(&self, board: ColorBox) -> bool {
        return self.row == BOX_HEIGHT - 1
            || !matches!(board[self.row + 1][self.col], Color::Empty);
    }

    pub fn get_current_possition(&self) -> (usize, usize) {
        return (self.row, self.col);
    }
}
