
use std::vec;

use crate::{attack_maps::{DIAGONAL_RAYS, KING_ATTACKS, KNIGHT_JUMPS, PAWN_CAPTURES, PAWN_PUSH, STRAIGHT_RAYS}, piece_set::PieceSet, utils::{flip_bit, get_lsb, test_bit}};



const KNIGHT_PROMOTED : u16 = 1;
const BISHOP_PROMOTED : u16 = 2;
const ROOK_PROMOTED : u16 = 3;
const QUEEN_PROMOTED : u16 = 4;

const EN_PESSANT : u16 = 5;

const CASTLE_KING : u16 = 6;
const CASTLE_QUEEN : u16 = 7;

const WHITE_KING_START: usize = 4;
const BLACK_KING_START: usize = 60;
const WHITE_KINGSIDE_MASK: u64 = 0x60; 
const WHITE_QUEENSIDE_MASK: u64 = 0x0E;
const BLACK_KINGSIDE_MASK: u64 = 1u64 << 61 | 1u64 << 62;
const BLACK_QUEENSIDE_MASK: u64 = 1u64 << 59 | 1u64 << 58 | 1u64 << 57;

pub fn generate_diagonal_moves(index : usize , occupied : u64) -> u64{
    
    let mut moves : u64 = 0;

    for direction in 0..4 {
        let ray = DIAGONAL_RAYS[direction][index];
        let blockers = ray & occupied;
        
        let blocker_pos = if blockers == 0{
            64
        } else {
            match direction {
                0 | 3 => blockers.trailing_zeros() as usize,
                _ => 63 - blockers.leading_zeros() as usize,
            }
        };

        moves |= ray ^ DIAGONAL_RAYS[direction][blocker_pos];
    }

    moves
}

pub fn generate_straight_moves(index : usize , occupied : u64) -> u64 {
    let mut moves : u64 = 0;

    for direction in 0..4 {
        let ray = STRAIGHT_RAYS[direction][index];
        let blockers = ray & occupied;

        let blocker_pos : usize = if blockers == 0 {
            64
        } else {
            match direction {
                0 | 1 => blockers.trailing_zeros() as usize,
                _ => 63 - blockers.leading_zeros() as usize
            }
        };

        moves |= ray ^ STRAIGHT_RAYS[direction][blocker_pos];
    }

    moves

}


pub fn generate_knight_moves(index : usize) -> u64 {
    KNIGHT_JUMPS[index]
}


pub fn generate_pawn_moves(index : usize , occupied : u64 , turn : usize , enemy_double_pawn_push : u64) -> u64 {
    let mut moves = PAWN_PUSH[turn][index] & !occupied;

    let rank = index >> 3;

    if turn == 0 && rank == 1 {
        moves |= (moves << 8) & !occupied;
    }
    
    if turn == 1 && rank == 6 {
        moves |= (moves >> 8) & !occupied;
    }

    moves |= PAWN_CAPTURES[turn][index] & (occupied | enemy_double_pawn_push);    

    moves
}


pub fn generate_king_moves(index : usize , occupied : u64 , castle_rooks : u64 , turn :bool , in_check : bool) -> u64{
    let mut moves = KING_ATTACKS[index];
    if in_check == true || castle_rooks == 0 {
        return moves;
    }

    if turn == false && index == WHITE_KING_START {
        if test_bit(castle_rooks, 7) && (occupied & WHITE_KINGSIDE_MASK == 0) {
            moves |= 1 << 6;
        }
        if test_bit(castle_rooks, 0) && (occupied & WHITE_QUEENSIDE_MASK == 0) {
            moves |= 1 << 2;
        }
    } 

    if turn == true && index == BLACK_KING_START {
        if test_bit(castle_rooks, 63) && (occupied & BLACK_KINGSIDE_MASK == 0) {
            moves |= 1u64 << 62;
        }
        
        if test_bit(castle_rooks, 56) && (occupied & BLACK_QUEENSIDE_MASK == 0) {
            moves |= 1u64 << 58;
        }
        
    }

    moves
}


pub fn iterate_move_map( piece_set : &PieceSet , src_index : usize,
    moves_for_piece : &mut u64  , mode : u8 ) ->  Vec<u16>{
    
    let mut moves : Vec<u16> = vec![];

    loop {
        let dis_index = get_lsb(*moves_for_piece) ;
        if dis_index == 64 {
            break;
        }

        let mov : u16 = src_index as u16 | (dis_index << 6) as u16;
        let distance = src_index.abs_diff(dis_index);

        if mode == 1 {
            if dis_index >= 56 || dis_index <= 7 {
                moves.push(mov | (KNIGHT_PROMOTED << 12));
                moves.push(mov | (QUEEN_PROMOTED << 12 ));
                moves.push(mov | (BISHOP_PROMOTED << 12 ));
                moves.push(mov | (ROOK_PROMOTED << 12 ));
            }
            else if (distance == 7 || distance == 9) && !test_bit(piece_set.occupied, dis_index as usize) {
                moves.push(mov | (EN_PESSANT << 12));
            }
            else 
            {
                moves.push(mov);
            }
        } 
        else if mode == 2 && distance >= 2 && dis_index >> 3 == src_index >> 3 {
            if distance == 2 {
                moves.push(mov | (CASTLE_KING << 12));
            }
            else 
            {
                moves.push(mov | (CASTLE_QUEEN  << 12) );
            }
        }
        else
        {
            moves.push(mov);
        }

        flip_bit(moves_for_piece , dis_index as usize);
    }

    moves
}


pub fn iterate_possible_move< F , T>(
    mut piece_positions : u64,
    ally : &PieceSet,
    enemy : &PieceSet,    
    mode : u8 ,
    generation_function : F , 
    args : T ,
) -> Vec<u16> 
where 
    F : Fn(usize , &T) -> u64 {
    let mut moves : Vec<u16> = vec![];
    
    
    loop {
        let index = get_lsb(piece_positions);
        if index == 64{
            break;
        }

        let mut moves_for_piece = generation_function(index , &args) & (!ally.occupied);
        moves.append(
            &mut iterate_move_map(&enemy, index, &mut moves_for_piece, mode)
        );

        flip_bit(&mut piece_positions, index);
    }

    moves
}
