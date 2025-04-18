

use std::ops::Not;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Player
{
    White,
    Black,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}