use crate:: piece_set::PieceSet;

#[derive(PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    
} 

#[derive(Clone)]
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
