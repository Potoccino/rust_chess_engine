use std::io::{self, BufRead};

use crate::{bit_board::BitBoard, engine::generate_attack_maps};

pub fn print_move(mov : &u16) {
    let src = mov & 0x3F;
    let dest = (mov >> 6) & 0x3F;  
    let special = (mov >> 12) & 0x7;

    println!("Move from {} to {} with special {}\n", src, dest, special);

}


pub fn read_move_components() -> Result<(usize, usize, usize), String> {
    let stdin = io::stdin();
    let mut line = String::new();
    
    // println!("Enter source, destination, and special move code (e.g., 8 16 3): ");
    stdin.lock().read_line(&mut line).map_err(|e| e.to_string())?;
    
    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|_| "Failed to parse input as numbers".to_string())?;
    
    if numbers.len() != 3 {
        return Err("Please enter exactly 3 numbers".to_string());
    }
    
    Ok((numbers[0], numbers[1], numbers[2]))
}


pub fn print_bitset(set : &u64){

    for i  in( 0..8).rev() {
        for j  in 0.. 8 {
            let sum = i * 8 + j;
            let bit  = if set & (1u64 << sum ) != 0 {1} else {0};
            print!("{} " , bit );
        } 
        println!("");
    }
    println!("");
}

pub fn print_board(board : &BitBoard)
{
    let mut view : Vec<char> = vec!['.' ; 64];

    for i in 0..64 {
        if (1u64 << i) & board.white_set.pawns != 0 {
            view[i] = 'P';
        }
        else if (1u64 << i) & board.white_set.rooks != 0 {
            view[i] = 'R';
        }
        else if (1u64 << i) & board.white_set.knights != 0 {
            view[i] = 'N';
        }
        else if (1u64 << i) & board.white_set.bishops != 0 {
            view[i] = 'B';
        }
        else if (1u64 << i) & board.white_set.queens != 0 {
            view[i] = 'Q';
        }
        else if (1u64 << i) & board.white_set.kings != 0 {
            view[i] = 'K';
        }
        else if (1u64 << i) & board.black_set.pawns != 0 {
            view[i] = 'p';
        }
        else if (1u64 << i) & board.black_set.rooks != 0 {
            view[i] = 'r';
        }
        else if (1u64 << i) & board.black_set.knights != 0 {
            view[i] = 'n';
        }
        else if (1u64 << i) & board.black_set.bishops != 0 {
            view[i] = 'b';
        }
        else if (1u64 << i) & board.black_set.queens != 0 {
            view[i] = 'q';
        }
        else if (1u64 << i) & board.black_set.kings != 0 {
            view[i] = 'k';
        }
    }

    for i in (0..8).rev() {
        for j in 0..8 {
            print!("{} " , view[i * 8 + j]);
        }
        println!("");
    }

    println!("");


}

pub fn get_bit(bitset :  u64 , index : usize) -> bool{
    (1u64 << index) & bitset != 0
}

pub fn set_bit( bitset :  &mut u64 , index : usize) {
    *bitset |= 1u64 << index;
}

pub fn reset_bit( bitset :  &mut u64 , index : usize) {
    *bitset &= !(1u64 << index);
}

pub fn test_bit(bitset :  u64 , index : usize) -> bool {
    bitset & (1u64 << index) != 0
}

pub fn flip_bit(bitset :  &mut u64 , index : usize) {
    *bitset ^= 1u64 << index;
}

pub fn get_lsb(bitset :  u64) -> usize {
    bitset.trailing_zeros() as usize
}

pub fn get_msb(bitset :  u64) -> usize {
    bitset.leading_zeros() as usize
}



pub fn fen_to_bitboard(fen: &str) -> Result<(BitBoard , bool), String> {
    let mut board = BitBoard::get_empty_board();
    
    // Split FEN string into its components
    let components: Vec<&str> = fen.split_whitespace().collect();
    if components.len() < 5 {
        return Err("Invalid FEN string format".to_string());
    }
    
    // Get the board representation part
    let board_str = components[0];
    
    // Parse the board representation
    let mut rank = 7; // Start from the 8th rank (index 7)
    let mut file = 0; // Start from the a-file (index 0)
    
    for c in board_str.chars() {
        match c {
            '/' => {
                // Move to the next rank
                rank -= 1;
                file = 0;
                if rank < 0 {
                    return Err("Invalid FEN: Too many ranks".to_string());
                }
            },
            '1'..='8' => {
                // Skip empty squares
                file += c.to_digit(10).unwrap() as i32;
                if file > 8 {
                    return Err("Invalid FEN: Rank too long".to_string());
                }
            },
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                // Place a piece
                if file >= 8 {
                    return Err("Invalid FEN: Rank too long".to_string());
                }
                
                let square_index = rank * 8 + file;
                let bit = 1u64 << square_index;
                
                // Check if the piece is white or black
                let is_white = c.is_uppercase();
                let piece_set = if is_white { &mut board.white_set } else { &mut board.black_set };
                
                // Update the appropriate bitboard
                match c.to_ascii_lowercase() {
                    'p' => piece_set.pawns |= bit,
                    'n' => piece_set.knights |= bit,
                    'b' => piece_set.bishops |= bit,
                    'r' => piece_set.rooks |= bit,
                    'q' => piece_set.queens |= bit,
                    'k' => piece_set.kings |= bit,
                    _ => unreachable!(),
                }
                
                // Update the occupied bitboard
                piece_set.occupied |= bit;
                
                file += 1;
            },
            _ => return Err(format!("Invalid character in FEN: {}", c)),
        }
    }
    
    // Parse castling rights
    if components.len() > 2 {
        let castling = components[2];
        for c in castling.chars() {
            match c {
                'K' => board.white_set.castle_rooks |= 1u64 << 7,  // H1
                'Q' => board.white_set.castle_rooks |= 1u64,       // A1
                'k' => board.black_set.castle_rooks |= 1u64 << 63, // H8
                'q' => board.black_set.castle_rooks |= 1u64 << 56, // A8
                '-' => {}, // No castling rights
                _ => {}    // Ignore other characters
            }
        }
    }
    
    // Parse en passant target square
    if components.len() > 3 {
        let en_passant = components[3];
        if en_passant != "-" {
            if en_passant.len() != 2 {
                return Err("Invalid en passant square".to_string());
            }
            
            let ep_file = en_passant.chars().nth(0).unwrap() as i32 - 'a' as i32;
            let ep_rank = en_passant.chars().nth(1).unwrap() as i32 - '1' as i32;
            
            if ep_file < 0 || ep_file > 7 || ep_rank < 0 || ep_rank > 7 {
                return Err("Invalid en passant square".to_string());
            }
            
            // Calculate the square of the pawn that made the double push
            let pawn_rank = if ep_rank == 2 { 3 } else { 4 };
            let pawn_square = pawn_rank * 8 + ep_file;
            let pawn_bit = 1u64 << pawn_square;
            
            // Mark the pawn as having made a double push
            if ep_rank == 2 {
                board.black_set.double_push_pawns |= pawn_bit;
            } else {
                board.white_set.double_push_pawns |= pawn_bit;
            }
        }
    }
    
    let mut turn = false;
    if components.len() > 4 {
        turn = if components[4] == "b" {true} else {false};
    }

    generate_attack_maps(&mut board, false);
    generate_attack_maps(&mut board, true);
    
    Ok((board , turn))
}
