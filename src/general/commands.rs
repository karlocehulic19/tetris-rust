use crate::general::movements::Movement;

pub enum Command {
    EndGame,
    Move(Movement),
}
