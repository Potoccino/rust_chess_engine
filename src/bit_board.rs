use std::hash::{Hasher , Hash};

use crate:: piece_set::PieceSet;

#[derive(Eq, Hash, PartialEq , Debug )]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    
} 

#[derive(Clone , Eq, PartialEq)]
pub struct BitBoard
{
    pub black_set : PieceSet,
    pub white_set : PieceSet,  

}


impl BitBoard {

    pub fn get_empty_board() -> BitBoard
    {
        let board: BitBoard = BitBoard {
            white_set : PieceSet::get_empty_piece_set(),
            black_set : PieceSet::get_empty_piece_set(),
        };
        return board;
    }

    pub fn get_starting_board() -> BitBoard
    {
        return  BitBoard {
            white_set : PieceSet::get_starting_white_set(),
            black_set : PieceSet::get_starting_black_set(),
        };
    }

}

impl Hash for BitBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash white pieces
        self.white_set.rooks.hash(state);
        self.white_set.knights.hash(state);
        self.white_set.bishops.hash(state);
        self.white_set.queens.hash(state);
        self.white_set.kings.hash(state);
        self.white_set.pawns.hash(state);
        self.white_set.castle_rooks.hash(state);
        self.white_set.double_push_pawns.hash(state);
        
        // Hash black pieces
        self.black_set.rooks.hash(state);
        self.black_set.knights.hash(state);
        self.black_set.bishops.hash(state);
        self.black_set.queens.hash(state);
        self.black_set.kings.hash(state);
        self.black_set.pawns.hash(state);
        self.black_set.castle_rooks.hash(state);
        self.black_set.double_push_pawns.hash(state);
    }
}