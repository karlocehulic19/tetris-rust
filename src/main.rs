const SIZE: usize = 10;

enum ShipOrientation {
    Horizontal,
    Vertical,
}

fn main() {
    let mut player1 = Player {
        name: String::from("Joe"),
        board: [[0; SIZE]; SIZE],
    };
    let player2 = Player {
        name: String::from("Computer"),
        board: [[0; SIZE]; SIZE],
    };

    player1.place_ship(1, 1, 3, ShipOrientation::Vertical);
    println!("Player {}, with board {:?}", player1.name, player1.board);
    println!("Player {}, with board {:?}", player2.name, player2.board);
}

struct Player {
    name: String,
    board: [[u32; SIZE]; SIZE],
}

impl Player {
    fn place_ship(
        &mut self,
        row: usize,
        col: usize,
        len: usize,
        orientation: ShipOrientation,
    ) -> Result<(), String> {
        match orientation {
            ShipOrientation::Horizontal => {
                let end_col = col + len;
                for c in col..end_col {
                    if self.board[row][c] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                    self.board[row][c] = 1;
                }
                Ok(())
            }
            ShipOrientation::Vertical => {
                let end_row = col + len;
                for r in row..end_row {
                    if self.board[r][col] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                    self.board[r][col] = 1;
                }
                Ok(())
            }
        }
    }
}

fn attack(row: usize, col: usize, board: &mut [[u32; SIZE]; SIZE]) {
    board[row][col] = 1;
}
