
use engine::run_engine;
use utils::fen_to_bitboard;

pub mod piece_set;
pub mod bit_board;
pub mod attack_maps;
pub mod utils;
pub mod move_generator;
pub mod engine;
mod tests;


fn main() {
    let fen = String::from("
        r3k2r/8/8/8/4P3/8/8/R3K2R w KQkq - 0 1
    ");

    let (mut board , turn) = match  fen_to_bitboard(&fen) {
        Ok(board) => board,
        _ => panic!()
    };
    run_engine(&mut board, turn);


} 


