

use bit_board::{BitBoard, PieceType};
use engine::{apply_move, generate_attack_maps, generate_moves, get_piece_type, king_in_check, run_engine, unapply_move, MoveResult};
use utils::{fen_to_bitboard, print_bitset, print_board, print_move};

pub mod piece_set;
pub mod bit_board;
pub mod attack_maps;
pub mod utils;
pub mod move_generator;
pub mod engine;
mod tests;

extern crate pleco;


fn main() {

} 
