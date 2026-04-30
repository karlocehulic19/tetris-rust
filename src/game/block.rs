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
    CellOccupied,
}

pub type CellPosition = (usize, usize);
type BlockPosition = (CellPosition, Vec<CellPosition>);

#[derive(Debug)]
pub struct Block {
    position: BlockPosition,
    prev_position: Option<BlockPosition>,
}

impl Block {
    pub fn new(board: ColorBox) -> Result<Self, BlockError> {
        // Hardcoded for L block for now..., will discover better approaches later...
        let main_cell_row: usize = 0;
        let main_cell_col: usize = 3;
        let offset_cells: Vec<(usize, usize)> = vec![(1, 0), (1, 1), (1, 2)];
        Block::check_board(
            ((main_cell_row, main_cell_col), offset_cells.clone()),
            board,
        );

        Ok(Self {
            position: ((main_cell_row, main_cell_col), offset_cells),
            prev_position: None,
        })
    }

    pub fn check_board(position: BlockPosition, board: ColorBox) -> Result<(), BlockError> {
        return Ok(());
    }

    pub fn move_block(
        &mut self,
        movement: Movement,
        board: ColorBox,
    ) -> Result<(usize, usize), BlockError> {
        match movement {
            Movement::Left => {
                // if self.col == 0 {
                //     return Err(BlockError::OutOfBounds);
                // }
                // self.col -= 1;
                // return Ok((self.row, self.col));
                return Ok((0, 0));
            }
            Movement::Right => {
                // if self.col == BOX_WIDTH - 1 {
                //     return Err(BlockError::OutOfBounds);
                // };
                // self.col += 1;
                // return Ok((self.row, self.col));
                return Ok((0, 0));
            }
            Movement::Down => Ok((0, 0)),
        }
    }

    pub fn move_down(&mut self, board: ColorBox) -> Result<Vec<CellPosition>, BlockError> {
        if self.is_grounded(board) {
            return Err(BlockError::Grounded);
        }

        self.prev_position = Some(self.position.clone());
        self.position.0.0 += 1;

        return Ok(self.get_block_cells());
    }

    // can either do this, and then have to call move down, or try to move down in the first place
    fn is_grounded(&self, board: ColorBox) -> bool {
        return false;
        // return self.row == BOX_HEIGHT - 1
        //     || !matches!(board[self.row + 1][self.col], Color::Empty);
    }

    pub fn get_prev_block_cells(&self) -> Option<Vec<CellPosition>> {
        let prev_clone = self.prev_position.clone();

        if prev_clone.is_none() {
            return None;
        }
        return Some(Block::block_pos_to_cell_pos(prev_clone.unwrap()));
    }

    pub fn get_block_cells(&self) -> Vec<CellPosition> {
        return Block::block_pos_to_cell_pos(self.position.clone());
    }

    pub fn get_current_possition(&self) -> (usize, usize) {
        return (self.position.0.0, self.position.0.1);
    }

    fn block_pos_to_cell_pos(position: BlockPosition) -> Vec<CellPosition> {
        let ((row, col), offset) = position;
        let mut block_cells = vec![(row, col)];
        for (o_row, o_col) in offset {
            block_cells.push((row + o_row, col + o_col));
        }

        return block_cells;
    }
}
