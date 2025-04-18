

use bit_board::BitBoard;

pub mod piece_set;
pub mod bit_board;
pub mod attack_maps;
pub mod utils;
pub mod move_generator;
pub mod engine;
pub mod player;
mod tests;



// extern crate pleco;


fn main() {
    let board = BitBoard::get_starting_board();
    board.print_board();
    

} 
