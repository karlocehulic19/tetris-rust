const SIZE: usize = 10;

fn main() {
    let player1 = Player {
        name: String::from("Joe"),
        board: [[0; SIZE]; SIZE],
    };
    let player2 = Player {
        name: String::from("Computer"),
        board: [[0; SIZE]; SIZE],
    };

    println!("Player {}, with board {:?}", player1.name, player1.board);
    println!("Player {}, with board {:?}", player2.name, player2.board);
}

struct Player {
    name: String,
    board: [[u32; SIZE]; SIZE],
}

fn attack(row: usize, col: usize, board: &mut [[u32; SIZE]; SIZE]) {
    board[row][col] = 1;
}
