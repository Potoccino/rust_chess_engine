use crate::engine::{
    apply_castle_move, apply_double_pawn_push, apply_enpessant, apply_normal_move, apply_promotion,
    unapply_castle_move, unapply_double_pawn_push, unapply_normal_move, unapply_promotion,
    unpply_enpessant,
};
use crate::move_generator::{
    generate_diagonal_moves, generate_king_attacks, generate_king_moves, generate_knight_moves,
    generate_pawn_attacks, generate_pawn_moves, generate_straight_moves, iterate_attack_moves,
    iterate_possible_move,
};

use crate::piece_set::PieceSet;
use crate::utils::{ read_move_components,get_lsb, flip_bit, test_bit};
use crate::attack_maps::{DIAGONAL_RAYS, KING_ATTACKS, KNIGHT_JUMPS, STRAIGHT_RAYS};
use crate::player::Player;

const KNIGHT_PROMOTED: u16 = 1;
const BISHOP_PROMOTED: u16 = 2;
const ROOK_PROMOTED: u16 = 3;
const QUEEN_PROMOTED: u16 = 4;

const EN_PESSANT: u16 = 5;

const CASTLE_KING: u16 = 6;
const CASTLE_QUEEN: u16 = 7;

const DOUBLE_PAWN_PUSH: u16 = 8;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum MoveResult {
    Enpassant(u64, u64),
    Castle(u64, u64),
    Promotions(PieceType, Option<PieceType>, u64, u64),
    NormalMove(PieceType, Option<PieceType>, u64, u64, u64),
    DoublePawnPush(u64),
}

#[derive(Clone, Eq, PartialEq)]
pub struct BitBoard {
    pub black_set: PieceSet,
    pub white_set: PieceSet,
    pub player: Player,
}

impl BitBoard {
    pub fn get_empty_board() -> BitBoard {
        let board: BitBoard = BitBoard {
            white_set: PieceSet::get_empty_piece_set(),
            black_set: PieceSet::get_empty_piece_set(),
            player: Player::White,
        };
        return board;
    }

    pub fn get_starting_board() -> BitBoard {
        return BitBoard {
            white_set: PieceSet::get_starting_white_set(),
            black_set: PieceSet::get_starting_black_set(),
            player: Player::White,
        };
    }

    pub fn print_board(&self)
    {
        let mut view : Vec<char> = vec!['.' ; 64];

        for i in 0..64 {
            if (1u64 << i) & self.white_set.pawns != 0 {
                view[i] = 'P';
            }
            else if (1u64 << i) & self.white_set.rooks != 0 {
                view[i] = 'R';
            }
            else if (1u64 << i) & self.white_set.knights != 0 {
                view[i] = 'N';
            }
            else if (1u64 << i) & self.white_set.bishops != 0 {
                view[i] = 'B';
            }
            else if (1u64 << i) & self.white_set.queens != 0 {
                view[i] = 'Q';
            }
            else if (1u64 << i) & self.white_set.kings != 0 {
                view[i] = 'K';
            }
            else if (1u64 << i) & self.black_set.pawns != 0 {
                view[i] = 'p';
            }
            else if (1u64 << i) & self.black_set.rooks != 0 {
                view[i] = 'r';
            }
            else if (1u64 << i) & self.black_set.knights != 0 {
                view[i] = 'n';
            }
            else if (1u64 << i) & self.black_set.bishops != 0 {
                view[i] = 'b';
            }
            else if (1u64 << i) & self.black_set.queens != 0 {
                view[i] = 'q';
            }
            else if (1u64 << i) & self.black_set.kings != 0 {
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

    
    pub fn fen_to_bitboard(fen: &str) -> Result<BitBoard , String> {
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
        
        let mut turn = Player::White;
        if components.len() > 4 {
            turn = if components[4] == "b" {Player::Black} else {Player::White};
        }
    
        board.player = turn;

        board.generate_attack_maps(turn);
        board.generate_attack_maps(!turn);

        Ok(board)
    }
    

    pub fn bitboard_to_fen(&self, turn: bool) -> String {
        let mut fen = String::new();
        
        // Generate the board representation
        for rank in (0..8).rev() {  // Start from the 8th rank (index 7) going down
            let mut empty_squares = 0;
            
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let bit = 1u64 << square_index;
                
                if (self.white_set.occupied | self.black_set.occupied) & bit != 0 {
                    // If we had empty squares before this piece, add the count
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    
                    // Determine which piece is on this square
                    let piece_char = if self.white_set.occupied & bit != 0 {
                        // White piece
                        if self.white_set.pawns & bit != 0 { 'P' }
                        else if self.white_set.knights & bit != 0 { 'N' }
                        else if self.white_set.bishops & bit != 0 { 'B' }
                        else if self.white_set.rooks & bit != 0 { 'R' }
                        else if self.white_set.queens & bit != 0 { 'Q' }
                        else if self.white_set.kings & bit != 0 { 'K' }
                        else { panic!("Invalid piece configuration at square {}", square_index) }
                    } else {
                        // Black piece
                        if self.black_set.pawns & bit != 0 { 'p' }
                        else if self.black_set.knights & bit != 0 { 'n' }
                        else if self.black_set.bishops & bit != 0 { 'b' }
                        else if self.black_set.rooks & bit != 0 { 'r' }
                        else if self.black_set.queens & bit != 0 { 'q' }
                        else if self.black_set.kings & bit != 0 { 'k' }
                        else { panic!("Invalid piece configuration at square {}", square_index) }
                    };
                    
                    fen.push(piece_char);
                } else {
                    // Empty square
                    empty_squares += 1;
                }
            }
            
            // If there are empty squares at the end of the rank
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            
            // Add rank separator, except for the last rank
            if rank > 0 {
                fen.push('/');
            }
        }
        
        // Add active color
        fen.push(' ');
        fen.push(if turn { 'b' } else { 'w' });
        
        // Add castling rights
        fen.push(' ');
        let mut has_castling_rights = false;
        
        if self.white_set.castle_rooks & (1u64 << 7) != 0 {
            fen.push('K');
            has_castling_rights = true;
        }
        if self.white_set.castle_rooks & 1u64 != 0 {
            fen.push('Q');
            has_castling_rights = true;
        }
        if self.black_set.castle_rooks & (1u64 << 63) != 0 {
            fen.push('k');
            has_castling_rights = true;
        }
        if self.black_set.castle_rooks & (1u64 << 56) != 0 {
            fen.push('q');
            has_castling_rights = true;
        }
        if !has_castling_rights {
            fen.push('-');
        }
        
        // Add en passant target square
        fen.push(' ');
        let mut has_en_passant = false;
        
        // Check white double push pawns
        if self.white_set.double_push_pawns != 0 {
            // Find the file of the double pushed pawn
            let pawn_index = self.white_set.double_push_pawns.trailing_zeros() as usize;
            let file = pawn_index % 8;
            // En passant square is behind the pawn (from white's perspective)
            let ep_rank = 2; // 3rd rank (index 2)
            
            fen.push((file as u8 + b'a') as char);
            fen.push((ep_rank as u8 + b'1') as char);
            has_en_passant = true;
        }
        // Check black double push pawns
        else if self.black_set.double_push_pawns != 0 {
            // Find the file of the double pushed pawn
            let pawn_index = self.black_set.double_push_pawns.trailing_zeros() as usize;
            let file = pawn_index % 8;
            // En passant square is behind the pawn (from black's perspective)
            let ep_rank = 5; // 6th rank (index 5)
            
            fen.push((file as u8 + b'a') as char);
            fen.push((ep_rank as u8 + b'1') as char);
            has_en_passant = true;
        }
        
        if !has_en_passant {
            fen.push('-');
        }
        
        // Add halfmove clock and fullmove number
        // Since these aren't tracked in the BitBoard structure, we'll add default values
        fen.push_str(" 0 1");
        
        fen
    }

}

impl BitBoard {
    pub fn generate_moves(&self, turn: Player) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();

        let (ally , enemy) = if turn == Player::White {
            (&self.white_set, &self.black_set)
        } else {
            (&self.black_set, &self.white_set)
        };

        let occupied = ally.occupied | enemy.occupied;

        iterate_possible_move(
            ally.bishops,
            ally,
            enemy,
            0,
            |index, occupied| generate_diagonal_moves(index, *occupied),
            occupied,
            &mut moves,
        );

        iterate_possible_move(
            ally.queens,
            ally,
            enemy,
            0,
            |index, occupied| generate_diagonal_moves(index, *occupied),
            occupied,
            &mut moves,
        );

        iterate_possible_move(
            ally.queens,
            ally,
            enemy,
            0,
            |index, occupied| generate_straight_moves(index, *occupied),
            occupied,
            &mut moves,
        );

        iterate_possible_move(
            ally.rooks,
            ally,
            enemy,
            0,
            |index, occupied| generate_straight_moves(index, *occupied),
            occupied,
            &mut moves,
        );

        iterate_possible_move(
            ally.knights,
            ally,
            enemy,
            0,
            |index, _| generate_knight_moves(index),
            (),
            &mut moves,
        );

        iterate_possible_move(
            ally.pawns,
            ally,
            enemy,
            1,
            |index, args| {
                let (occupied, turn, double_pawn_push) = *args;
                generate_pawn_moves(index, occupied, turn as usize, double_pawn_push)
            },
            (occupied, turn, enemy.double_push_pawns),
            &mut moves,
        );

        iterate_possible_move(
            ally.kings,
            ally,
            enemy,
            2,
            |index, args| {
                let (occupied, castle_rooks, turn) = *args;
                generate_king_moves(index, occupied, castle_rooks, turn, enemy.attack_map)
            },
            (occupied, ally.castle_rooks, turn),
            &mut moves,
        );

        moves
    }

    pub fn king_in_check<>( &self , turn : Player) -> bool {
    
        let ally : &PieceSet = if turn == Player::White  {&self.white_set} else {&self.black_set};
        let enemy : &PieceSet = if turn == Player::White  {&self.black_set} else {&self.white_set};
    
        let king_index = get_lsb(ally.kings);
        let enemey_king_index = get_lsb(enemy.kings);
    
    
        if (1u64 << king_index) & KING_ATTACKS[enemey_king_index] != 0 {
            return true;
        }
    
        
        if KNIGHT_JUMPS[king_index] & enemy.knights != 0 {
            return  true;
        }
        
        let occupied = ally.occupied | enemy.occupied;
        let mut on_same_file : u64 = 0;
        
        for i in 0..4 {
            on_same_file |= STRAIGHT_RAYS[i][king_index] & (enemy.rooks | enemy.queens);
        }
        
        loop {
            let index = get_lsb(on_same_file) ;
            if index == 64 {
                break;
            }
            
            if generate_straight_moves(index, occupied) & ((1 as u64) << king_index) as u64 != 0 {
                return true
            }
            
            flip_bit(&mut on_same_file, index);
        }
        
        let mut on_same_diagonal : u64 = 0;
        for i in 0..4 {
            on_same_diagonal |= DIAGONAL_RAYS[i][king_index] & (enemy.bishops | enemy.queens);
        }
        
        loop {
            let index = get_lsb(on_same_diagonal);
            if index == 64 {
                break;
            }
            
            if generate_diagonal_moves(index, occupied) & ((1 as u64 ) << king_index) as u64 != 0 {
                return true;
            }
            flip_bit(&mut on_same_diagonal, index);
        }
        
        let attack_direction :i8 = if turn == Player::Black {-8 + king_index as i8 } else {8 + king_index as i8 };
        
        if attack_direction < 0 || attack_direction > 63 {
            return false;
        }
        
        if king_index & 0b111 != 0b111  && test_bit(enemy.pawns , attack_direction as usize + 1){
            return true;
        }
    
        if king_index & 0b111 != 0  && test_bit(enemy.pawns , attack_direction as usize - 1){
            return true;
        }
    
        return false    
    }
    
    

    pub fn run_engine<'a>(&mut self, mut turn: Player) {
        loop {
            
            self.generate_attack_maps(turn);
            self.generate_attack_maps(!turn);

            let moves = self.generate_moves(turn);

            // get the best move from the moves
            // or in this case take input from the user

            let (src, dest, speical) = match read_move_components() {
                Ok((src, dest, special)) => (src, dest, special),
                Err(_) => {
                    println!("re-enter move");
                    continue;
                }
            };
            println!("Move from {} to {} with special {}", src, dest, speical);

            let mov = (src | (dest << 6) | (speical << 12)) as u16;

            if !moves.contains(&mov) {
                println!("Invalid move");
                continue;
            }

            let mov_result: MoveResult = self.apply_move(turn, mov);

            let in_check = self.king_in_check(turn);

            if in_check {
                self.unapply_move(turn, mov, mov_result);
                println!("Invalid move, king in check");
                continue;
            }

            turn = !turn;
        }
    }


    pub fn apply_move(&mut self, turn: Player, mov: u16) -> MoveResult {
        match mov >> 12 {
            KNIGHT_PROMOTED | BISHOP_PROMOTED | ROOK_PROMOTED | QUEEN_PROMOTED => {
                let (promoted_piece_type, dest_piece_type, enemy_rooks, enemy_double_pawn_push) =
                    apply_promotion(self, turn, mov);
                MoveResult::Promotions(
                    promoted_piece_type,
                    dest_piece_type,
                    enemy_rooks,
                    enemy_double_pawn_push,
                )
            }
            EN_PESSANT => {
                let (enemy_double_push_pawn, _) = apply_enpessant(self, turn, mov);
                MoveResult::Enpassant(enemy_double_push_pawn, 0)
            }
            CASTLE_KING | CASTLE_QUEEN => {
                let (castle_rooks, enemy_double_pawn_push) = apply_castle_move(self, turn, mov);
                MoveResult::Castle(castle_rooks, enemy_double_pawn_push)
            }
            DOUBLE_PAWN_PUSH => {
                let enemy_double_pawn_push = apply_double_pawn_push(self, turn, mov);
                MoveResult::DoublePawnPush(enemy_double_pawn_push)
            }
            _ => {
                let (
                    src_piece_type,
                    dest_piece_type,
                    castle_rooks,
                    enemy_rooks,
                    enemy_double_pawn_push,
                ) = apply_normal_move(self, turn, mov);
                MoveResult::NormalMove(
                    src_piece_type,
                    dest_piece_type,
                    castle_rooks,
                    enemy_rooks,
                    enemy_double_pawn_push,
                )
            }
        }
    }

    pub fn unapply_move<'a>(&mut self, turn: Player, mov: u16, mov_result: MoveResult) {
        match mov >> 12 {
            KNIGHT_PROMOTED | BISHOP_PROMOTED | ROOK_PROMOTED | QUEEN_PROMOTED => {
                if let MoveResult::Promotions(
                    promoted_piece_type,
                    dest_piece_type,
                    enemy_rooks,
                    enemy_double_pawn_push,
                ) = mov_result
                {
                    unapply_promotion(
                        self,
                        turn,
                        mov,
                        promoted_piece_type,
                        dest_piece_type,
                        enemy_rooks,
                        enemy_double_pawn_push,
                    );
                }
            }
            EN_PESSANT => {
                if let MoveResult::Enpassant(enemy_double_push_pawns, _) = mov_result {
                    unpply_enpessant(self, turn, mov, enemy_double_push_pawns);
                }
            }
            CASTLE_KING | CASTLE_QUEEN => {
                if let MoveResult::Castle(castle_rooks, enemy_double_pawn_push) = mov_result {
                    unapply_castle_move(self, turn, mov, castle_rooks, enemy_double_pawn_push);
                }
            }
            DOUBLE_PAWN_PUSH => {
                if let MoveResult::DoublePawnPush(enemy_double_pawn_push) = mov_result {
                    unapply_double_pawn_push(self, turn, mov, enemy_double_pawn_push);
                }
            }
            _ => {
                if let MoveResult::NormalMove(
                    src_piece_type,
                    dest_piece_type,
                    rooks,
                    enemy_rooks,
                    enemy_double_pawn_push,
                ) = mov_result
                {
                    unapply_normal_move(
                        self,
                        turn,
                        mov,
                        src_piece_type,
                        dest_piece_type,
                        rooks,
                        enemy_rooks,
                        enemy_double_pawn_push,
                    );
                }
            }
        }
    }

    pub fn generate_attack_maps(&mut self, turn : Player) {
        let mut attacks = 0;
        let occupied = self.black_set.occupied | self.white_set.occupied;
        let ally_pieces = if turn == Player::White {
            &mut self.white_set
        } else {
            &mut self.black_set
        };

        attacks |= iterate_attack_moves(
            ally_pieces.pawns,
            |index, turn| generate_pawn_attacks(index, &turn),
            turn,
        );

        attacks |= iterate_attack_moves(
            ally_pieces.knights,
            |index, _| generate_knight_moves(index),
            (),
        );

        attacks |= iterate_attack_moves(
            ally_pieces.kings,
            |index, _| generate_king_attacks(index),
            (),
        );

        attacks |= iterate_attack_moves(
            ally_pieces.rooks,
            |index, occupied| generate_straight_moves(index, *occupied),
            occupied,
        );

        attacks |= iterate_attack_moves(
            ally_pieces.queens,
            |index, occupied| generate_straight_moves(index, *occupied),
            occupied,
        );

        attacks |= iterate_attack_moves(
            ally_pieces.queens,
            |index, occupied| generate_diagonal_moves(index, *occupied),
            occupied,
        );

        attacks |= iterate_attack_moves(
            ally_pieces.bishops,
            |index, occupied| generate_diagonal_moves(index, *occupied),
            occupied,
        );

        ally_pieces.attack_map = attacks;
    }
}
