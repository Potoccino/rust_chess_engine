
use crate::{attack_maps::{DIAGONAL_RAYS, KNIGHT_JUMPS, STRAIGHT_RAYS}, bit_board::{BitBoard, PieceType}, move_generator::{generate_diagonal_moves, generate_king_moves, generate_knight_moves, generate_pawn_moves, generate_straight_moves, iterate_possible_move}, piece_set::PieceSet, utils::{flip_bit, get_lsb, reset_bit, set_bit, test_bit}};


const KNIGHT_PROMOTED : u16 = 1;
const BISHOP_PROMOTED : u16 = 2;
const ROOK_PROMOTED : u16 = 3;
const QUEEN_PROMOTED : u16 = 4;

const EN_PESSANT : u16 = 5;

const CASTLE_KING : u16 = 6;
const CASTLE_QUEEN : u16 = 7;


pub fn king_in_check( board : &BitBoard  , turn : bool) -> bool {
    
    let ally : &PieceSet = if turn == false {&board.white_set} else {&board.black_set};
    let enemy : &PieceSet = if turn == false {&board.black_set} else {&board.white_set};

    let king_index = get_lsb(ally.kings);


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

        if generate_straight_moves(index, occupied) & king_index as u64 != 0 {
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

        if generate_diagonal_moves(index, occupied) & king_index as u64 != 0 {
            return true;
        }
    }

    let attack_direction :i8 = if turn {-8} else {8};

    if attack_direction < 0 || attack_direction > 63 {
        return false;
    }

    test_bit(enemy.pawns, king_index + attack_direction as usize - 1)
    | test_bit(enemy.pawns, king_index + attack_direction as usize + 1) 

}


pub fn generate_moves(board : &BitBoard , turn : bool) -> Vec<u16> {
    let mut moves : Vec<u16> = Vec::new();

    let ally : &PieceSet = if turn == false {&board.white_set} else {&board.black_set};
    let enemy : &PieceSet = if turn == false {&board.black_set} else {&board.white_set};

    let occupied = ally.occupied | enemy.occupied;

    moves.append(
        &mut iterate_possible_move(ally.bishops, ally, enemy, 0,  |index, occupied| generate_diagonal_moves(index, *occupied),
        occupied)
    );   

    moves.append(
        &mut iterate_possible_move(ally.queens, ally, enemy, 0,  |index, occupied| generate_diagonal_moves(index, *occupied),
        occupied)
    );

    moves.append(
        &mut iterate_possible_move(ally.rooks, ally, enemy, 0,  |index, occupied| generate_straight_moves(index, *occupied),
        occupied)
    );

    moves.append(
        &mut iterate_possible_move(ally.queens , ally , enemy , 0 , |index , occupied| generate_straight_moves(index , *occupied) , occupied)
    );

    moves.append(
        &mut iterate_possible_move(ally.knights , ally , enemy , 0 , |index , _ | generate_knight_moves(index) , ())
    );

    moves.append(
        &mut iterate_possible_move(ally.pawns , ally , enemy ,  1 ,
            |index , args| {
                let (occupied, turn , double_pawn_push) = *args;
                generate_pawn_moves(index, occupied, turn as usize , double_pawn_push)
            },(occupied, turn , enemy.double_push_pawns)
        )
    );
    
    moves.append(
        &mut iterate_possible_move(ally.kings , ally , enemy ,  2 ,
            |index , args| {
                let (occupied, castle_rooks, turn, in_check) = *args;
                generate_king_moves(index, occupied, castle_rooks, turn, in_check)
            },(occupied, ally.castle_rooks, turn, king_in_check( board, turn))
        )
    );

    moves
}


fn get_affected_bitset<'a>(set : &'a mut  PieceSet , index : usize) -> Option<&'a mut u64> {
   if test_bit(set.bishops , index) {
         return Some(&mut set.bishops);
    }
    else if test_bit(set.rooks , index) {
         return Some(&mut set.rooks);
    }
    else if test_bit(set.queens , index) {
         return Some(&mut set.queens);
    }
    else if test_bit(set.knights , index) {
         return Some(&mut set.knights);
    }
    else if test_bit(set.kings , index) {
         return Some(&mut set.kings);
    }
    else if test_bit(set.pawns , index) {
         return Some(&mut set.pawns);
   }

   None
}

pub fn apply_normal_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16) -> (Option<&'a mut  u64>, Option<& 'a mut   u64>){

    let (ally_pieces,  enemy_pieces) = if turn == false {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let src_piece_type  = if ally_pieces.rooks & (1u64 << src) != 0 {
        PieceType::Rook
    }
    else if ally_pieces.knights & (1u64 << src) != 0 {
        PieceType::Knight
    }
    else if ally_pieces.bishops & (1u64 << src) != 0 {
        PieceType::Bishop
    }
    else if ally_pieces.queens & (1u64 << src) != 0 {
        PieceType::Queen
    }
    else if ally_pieces.kings & (1u64 << src) != 0 {
        PieceType::King
    }
    else if ally_pieces.pawns & (1u64 << src) != 0 {
        PieceType::Pawn
    } else {
        return (None, None);
    };


    let dest_piece_type = if enemy_pieces.rooks & (1u64 << dest) != 0 {
        Some(PieceType::Rook)
    }
    else if enemy_pieces.knights & (1u64 << dest) != 0 {
        Some(PieceType::Knight)
    }
    else if enemy_pieces.bishops & (1u64 << dest) != 0 {
        Some(PieceType::Bishop)
    }
    else if enemy_pieces.queens & (1u64 << dest) != 0 {
        Some(PieceType::Queen)
    }
    else if enemy_pieces.kings & (1u64 << dest) != 0 {
        Some(PieceType::King)
    }
    else if enemy_pieces.pawns & (1u64 << dest) != 0 {
        Some(PieceType::Pawn)
    } else {
        None
    };

    let src_bitset = match src_piece_type {
        PieceType::Rook => &mut ally_pieces.rooks,
        PieceType::Knight => &mut ally_pieces.knights,
        PieceType::Bishop => &mut ally_pieces.bishops,
        PieceType::Queen => &mut ally_pieces.queens,
        PieceType::King => &mut ally_pieces.kings,
        PieceType::Pawn => &mut ally_pieces.pawns,  
    };

    reset_bit(src_bitset, src);
    set_bit(src_bitset, dest);
    
    reset_bit(&mut ally_pieces.occupied, src);
    set_bit(&mut ally_pieces.occupied, dest);


    let dest_bitset = if let Some(piece_type) = dest_piece_type {
        let dest_piece = match piece_type {
            PieceType::Rook => &mut enemy_pieces.rooks,
            PieceType::Knight => &mut enemy_pieces.knights,
            PieceType::Bishop => &mut enemy_pieces.bishops,
            PieceType::Queen => &mut enemy_pieces.queens,
            PieceType::King => &mut enemy_pieces.kings,
            PieceType::Pawn => &mut enemy_pieces.pawns,  
        };
        reset_bit(dest_piece, dest);
        reset_bit(&mut enemy_pieces.occupied, dest);
        Some(dest_piece)  
    }else {
        None
    };


    (Some(src_bitset), dest_bitset)
}


pub fn unapply_normal_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16, src_bitset : &mut u64, dest_bitset : Option<& 'a mut   u64>){
    let (ally_pieces,  enemy_pieces) = if turn == false {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    reset_bit(src_bitset, dest);
    set_bit(src_bitset, src);
    
    reset_bit(&mut ally_pieces.occupied, dest);
    set_bit(&mut ally_pieces.occupied, src);

    if let Some(dest_piece) = dest_bitset {
        set_bit(dest_piece, dest);
        set_bit(&mut enemy_pieces.occupied, dest);
    }
}


pub fn apply_castle_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16) -> u64 {
    let ally_pieces = if turn == false {
        &mut board.white_set
    } else {
        &mut board.black_set
    };
    let king_index = get_lsb(ally_pieces.kings);

    if mov >> 12 == CASTLE_KING {
        set_bit(&mut ally_pieces.kings, king_index + 2);
        reset_bit(&mut ally_pieces.kings, king_index);
        set_bit(&mut ally_pieces.rooks, king_index + 1);
        reset_bit(&mut ally_pieces.rooks, king_index + 3);
        set_bit(&mut ally_pieces.occupied, king_index + 2);
        reset_bit(&mut ally_pieces.occupied, king_index);
        set_bit(&mut ally_pieces.occupied, king_index + 1);
        reset_bit(&mut ally_pieces.occupied, king_index + 3);
    } else {
        set_bit(&mut ally_pieces.kings, king_index - 2);
        reset_bit(&mut ally_pieces.kings, king_index);
        set_bit(&mut ally_pieces.rooks, king_index - 1);
        reset_bit(&mut ally_pieces.rooks, king_index - 4);
        set_bit(&mut ally_pieces.occupied, king_index - 2);
        reset_bit(&mut ally_pieces.occupied, king_index);
        set_bit(&mut ally_pieces.occupied, king_index - 1);
        reset_bit(&mut ally_pieces.occupied, king_index - 4);
    }
    let castle_rooks = ally_pieces.castle_rooks;
    ally_pieces.castle_rooks = 0;
    castle_rooks
}


pub fn unapply_castle_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16 , castle_rooks : u64){
    let ally_pieces = if turn == false {
        &mut board.white_set
    } else {
        &mut board.black_set
    };
    let king_index = get_lsb(ally_pieces.kings);

    if mov >> 12 == CASTLE_KING {
        reset_bit(&mut ally_pieces.kings, king_index );
        set_bit(&mut ally_pieces.kings, king_index - 2);
        reset_bit(&mut ally_pieces.rooks, king_index - 1);
        set_bit(&mut ally_pieces.rooks, king_index + 1);
        reset_bit(&mut ally_pieces.occupied, king_index );
        set_bit(&mut ally_pieces.occupied, king_index - 2);
        reset_bit(&mut ally_pieces.occupied, king_index - 1);
        set_bit(&mut ally_pieces.occupied, king_index + 1);
    } else {
        reset_bit(&mut ally_pieces.kings, king_index);
        set_bit(&mut ally_pieces.kings, king_index + 2);
        reset_bit(&mut ally_pieces.rooks, king_index + 1);
        set_bit(&mut ally_pieces.rooks, king_index - 2);
        reset_bit(&mut ally_pieces.occupied, king_index);
        set_bit(&mut ally_pieces.occupied, king_index + 2);
        reset_bit(&mut ally_pieces.occupied, king_index + 1);
        set_bit(&mut ally_pieces.occupied, king_index - 2);
    }
    ally_pieces.castle_rooks = castle_rooks;
}


