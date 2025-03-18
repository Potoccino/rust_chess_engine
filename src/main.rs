
use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use bit_board::BitBoard;
use engine::{apply_move, generate_attack_maps, generate_moves, king_in_check, run_engine, unapply_move};
use utils::fen_to_bitboard;

pub mod piece_set;
pub mod bit_board;
pub mod attack_maps;
pub mod utils;
pub mod move_generator;
pub mod engine;
mod tests;


fn main() {
    // let fen = String::from("
    //     r3k2r/8/8/8/4P3/8/8/R3K2R w KQkq - 0 1
    // ");

    // let (mut board , turn) = match  fen_to_bitboard(&fen) {
    //     Ok(board) => board,
    //     _ => panic!()
    
    // };

    // let mut hasher = DefaultHasher::new();
    // board.hash(&mut hasher);
    // println!("haser {} " , hasher.finish());


    // run_engine(&mut board, turn);

    perft_test();

} 



fn preft_helper(board: &mut BitBoard, turn: bool, depth: i32, transposition_table: &mut HashMap<(u64, bool, i32), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    let hash = hasher.finish();
    let key = (hash, turn, depth);
    
    if let Some(&count) = transposition_table.get(&key) {
        return count;
    }


    board.white_set.attack_map = generate_attack_maps(board, false);
    board.black_set.attack_map = generate_attack_maps(board, true);
    
    let moves = generate_moves(board, turn);
    let mut move_count = 0;

    for &mov in &moves {  // Borrow the moves vector
        let mov_result = apply_move(board, turn, mov);
        if !king_in_check(board, turn) {
            move_count += preft_helper(board, !turn, depth - 1, transposition_table);
        }
        unapply_move(board, turn, mov, mov_result);
    }


    transposition_table.insert(key, move_count);
    move_count
}


fn perft_test(){
    let mut board = BitBoard::get_starting_board();
    let mut hash_map :  HashMap<(u64, bool, i32), u64> = HashMap::new();

    let move_count = preft_helper(&mut board, false, 4 , &mut hash_map);
    println!("{}" , move_count);

}

