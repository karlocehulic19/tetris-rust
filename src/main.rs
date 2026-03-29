const SIZE: usize = 10;

enum ShipOrientation {
    Horizontal,
    Vertical,
}

type ShipSizesArray = [usize; 5];
const SHIP_SIZES: ShipSizesArray = [2, 3, 3, 4, 5];

fn main() {
    let mut player1 = Player {
        name: String::from("Joe"),
        board: Board::new(),
    };
    let player2 = Player {
        name: String::from("Computer"),
        board: Board::new(),
    };

    let place1 = player1.place_ship(1, 1, 6, ShipOrientation::Vertical);
    let place2 = player1.place_ship(2, 2, 3, ShipOrientation::Horizontal);
    let place3 = player1.place_ship(1, 1, 3, ShipOrientation::Vertical);
    match place1 {
        Ok(()) => {
            println!("NO!, should fail!!");
        }
        Err(msg) => {
            println!("ERROR: {:?}", msg);
        }
    }
    // player1.place_ship(1, 1, 3, ShipOrientation::Vertical);
    println!("Player {}, with board {:?}", player1.name, player1.board);
    println!("Player {}, with board {:?}", player2.name, player2.board);
}

struct Player {
    name: String,
    board: Board,
}

#[derive(Debug)]
struct Board {
    cells: [[u32; SIZE]; SIZE],
    ships_needed: ShipSizesArray,
}

impl Board {
    fn new() -> Board {
        return Board {
            cells: [[0; SIZE]; SIZE],
            ships_needed: SHIP_SIZES.clone(),
        };
    }
}

impl Player {
    // how to check for ships, if any is consumed drop otherwise place
    fn place_ship(
        &mut self,
        row: usize,
        col: usize,
        len: usize,
        orientation: ShipOrientation,
    ) -> Result<(), String> {
        if !self.board.ships_needed.contains(&len) {
            return Err("Wrong ship size to place".to_string());
        }

        match orientation {
            ShipOrientation::Horizontal => {
                let end_col = col + len;
                for c in col..end_col {
                    if self.board.cells[row][c] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                }
                for c in col..end_col {
                    self.board.cells[row][c] = 1;
                }
                Ok(())
            }
            ShipOrientation::Vertical => {
                let end_row = col + len;
                for r in row..end_row {
                    if self.board.cells[r][col] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                }
                for r in row..end_row {
                    self.board.cells[r][col] = 1;
                }

                Ok(())
            }
        }
    }
}

fn attack(row: usize, col: usize, board: &mut [[u32; SIZE]; SIZE]) {
    board[row][col] = 1;
}
