use crate::{
    game::position::{BlockRelativePosition, CellPosition, CellPositions, PositionError},
    general::{colors::Color, constants::BOX_HEIGHT, movements::Movement, types::ColorBox},
};

#[derive(Debug)]
pub enum BlockError {
    Grounded,
    OutOfBounds,
}

impl From<PositionError> for BlockError {
    fn from(_: PositionError) -> Self {
        BlockError::OutOfBounds
    }
}

#[derive(Debug)]
pub struct Block {
    position: BlockRelativePosition,
    prev_position: Option<BlockRelativePosition>,
}

impl Block {
    pub fn new(board: ColorBox) -> Result<Self, BlockError> {
        // Hardcoded for L block for now..., will discover better approaches later...
        let main_cell_row: usize = 0;
        let main_cell_col: usize = 3;
        let offset_cells: Vec<(isize, isize)> = vec![(1, 0), (1, 1), (1, 2)];
        // TODO:  handle position checking correctly
        let position =
            BlockRelativePosition::new((main_cell_row, main_cell_col), offset_cells).unwrap();
        Block::check_board(&position, board);

        Ok(Self {
            position,
            prev_position: None,
        })
    }

    // TODO: handle game end criterie
    pub fn check_board(
        position: &BlockRelativePosition,
        board: ColorBox,
    ) -> Result<(), BlockError> {
        return Ok(());
    }

    pub fn move_block(
        &mut self,
        movement: Movement,
        board: ColorBox,
    ) -> Result<CellPositions, BlockError> {
        if self.is_grounded(board) {
            return Err(BlockError::Grounded);
        };

        let new_position = self.position.move_block(&movement)?;
        Self::check_board(&new_position, board)?;

        self.prev_position = Some(self.position.clone());
        self.position = new_position;

        return Ok(self.position.clone().into());
    }

    // can either do this, and then have to call move down, or try to move down in the first place
    fn is_grounded(&self, board: ColorBox) -> bool {
        let cells: CellPositions = self.position.clone().into();

        for (row, col) in &cells {
            let is_last_row = *row == BOX_HEIGHT - 1;
            if is_last_row {
                return true;
            }

            let is_bottom_cell = !cells.contains(&(row.clone() + 1, col.clone()));
            let is_bellow_colored = !matches!(board[row.clone() + 1][col.clone()], Color::Empty);
            if is_bottom_cell && is_bellow_colored {
                return true;
            }
        }

        return false;
    }

    pub fn get_prev_block_cells(&self) -> Option<CellPositions> {
        if let Some(ref prev_pos) = self.prev_position {
            return Some(prev_pos.clone().into());
        }

        return None;
    }

    pub fn get_block_cells(&self) -> CellPositions {
        return self.position.clone().into();
    }
}
