
use crate::{bit_board::{BitBoard, PieceType} ,  piece_set::PieceSet, player::Player, utils::{ get_lsb, reset_bit, set_bit}};


const KNIGHT_PROMOTED : u16 = 1;
const BISHOP_PROMOTED : u16 = 2;
const ROOK_PROMOTED : u16 = 3;
const QUEEN_PROMOTED : u16 = 4;

const CASTLE_KING : u16 = 6;



pub enum MoveResult {
    Enpassant(u64 , u64),
    Castle(u64 , u64),
    Promotions(PieceType, Option<PieceType> , u64 , u64),
    NormalMove(PieceType, Option<PieceType> , u64 , u64 , u64),
    DoublePawnPush(u64),
}

pub fn get_piece_type(pieces : &PieceSet, index : usize) -> Option<PieceType> {
    if pieces.rooks & (1u64 << index) != 0 {
        Some(PieceType::Rook)
    }
    else if pieces.knights & (1u64 << index) != 0 {
        Some(PieceType::Knight)
    }
    else if pieces.bishops & (1u64 << index) != 0 {
        Some(PieceType::Bishop)
    }
    else if pieces.queens & (1u64 << index) != 0 {
        Some(PieceType::Queen)
    }
    else if pieces.kings & (1u64 << index) != 0 {
        Some(PieceType::King)
    }
    else if pieces.pawns & (1u64 << index) != 0 {
        Some(PieceType::Pawn)
    } else {
        None
    }
}

fn get_piece_bitset<'a>(pieces: &'a mut PieceSet, piece_type: &PieceType) -> &'a mut u64 {
    match piece_type {
        PieceType::Rook => &mut pieces.rooks,
        PieceType::Knight => &mut pieces.knights,
        PieceType::Bishop => &mut pieces.bishops,
        PieceType::Queen => &mut pieces.queens,
        PieceType::King => &mut pieces.kings,
        PieceType::Pawn => &mut pieces.pawns,
    }
}


pub fn apply_normal_move<'a>(board : &'a mut BitBoard , turn : Player , mov : u16) -> (PieceType, Option<PieceType> , u64 , u64, u64) {

    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let src_piece_type  = get_piece_type(ally_pieces, src).unwrap();

    let ally_rooks = ally_pieces.castle_rooks;
    let enemy_rooks = enemy_pieces.castle_rooks;

    if src_piece_type == PieceType::King {
        ally_pieces.castle_rooks = 0;
    }

    if src_piece_type == PieceType::Rook {
        reset_bit(&mut ally_pieces.castle_rooks, src);
    }

    let dest_piece_type = get_piece_type(enemy_pieces, dest);

    let src_bitset = get_piece_bitset(ally_pieces, &src_piece_type);

    reset_bit(src_bitset, src);
    set_bit(src_bitset, dest);
    
    reset_bit(&mut ally_pieces.occupied, src);
    set_bit(&mut ally_pieces.occupied, dest);

    if let Some(piece_type) = dest_piece_type.as_ref() {
        if piece_type == &PieceType::Rook {
            reset_bit(&mut enemy_pieces.castle_rooks, dest);
        }
        reset_bit(&mut enemy_pieces.occupied, dest);
        let dest_piece = get_piece_bitset(enemy_pieces, piece_type);
        reset_bit(dest_piece, dest);
    }

    let enemy_double_push_pawns = enemy_pieces.double_push_pawns;
    enemy_pieces.double_push_pawns = 0;

    (src_piece_type , dest_piece_type , ally_rooks , enemy_rooks , enemy_double_push_pawns)
}


pub fn unapply_normal_move<'a>(board : &'a mut BitBoard , turn : Player , mov : u16, src_piece_type : PieceType, dest_piece_type : Option<PieceType> , ally_rooks : u64,
        enemy_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    ally_pieces.castle_rooks = ally_rooks;
    enemy_pieces.castle_rooks = enemy_rooks;

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let src_bitset = get_piece_bitset(ally_pieces, &src_piece_type);
    

    reset_bit(src_bitset, dest);
    set_bit(src_bitset, src);
    
    reset_bit(&mut ally_pieces.occupied, dest);
    set_bit(&mut ally_pieces.occupied, src);

    if let Some(dest_piece) = dest_piece_type {
        let dest_bitest = get_piece_bitset(enemy_pieces, &dest_piece);
        set_bit(dest_bitest, dest);
        set_bit(&mut enemy_pieces.occupied, dest);
    }

    enemy_pieces.double_push_pawns = enemy_double_pawn_push;
}


pub fn apply_castle_move<'a>(board : &'a mut BitBoard , turn : Player , mov : u16) -> (u64 , u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let king_index = get_lsb(ally_pieces.kings);

    if mov >> 12 == CASTLE_KING {
        reset_bit(&mut ally_pieces.kings, king_index);
        reset_bit(&mut ally_pieces.occupied, king_index);
        
        reset_bit(&mut ally_pieces.rooks, king_index + 3);
        reset_bit(&mut ally_pieces.occupied, king_index + 3);
        
        set_bit(&mut ally_pieces.kings, king_index + 2);
        set_bit(&mut ally_pieces.occupied, king_index + 2);

        set_bit(&mut ally_pieces.rooks, king_index + 1);
        set_bit(&mut ally_pieces.occupied, king_index + 1);
        
    } else {
        reset_bit(&mut ally_pieces.kings, king_index);
        reset_bit(&mut ally_pieces.occupied, king_index);
        
        reset_bit(&mut ally_pieces.rooks, king_index - 4);
        reset_bit(&mut ally_pieces.occupied, king_index - 4);
        
        set_bit(&mut ally_pieces.kings, king_index - 2);
        set_bit(&mut ally_pieces.occupied, king_index - 2);
        
        set_bit(&mut ally_pieces.rooks, king_index - 1);
        set_bit(&mut ally_pieces.occupied, king_index - 1);
    }
    let castle_rooks = ally_pieces.castle_rooks;
    ally_pieces.castle_rooks = 0;
    
    let enemy_double_push_pawns = enemy_pieces.double_push_pawns;
    enemy_pieces.double_push_pawns = 0;

    
    (castle_rooks , enemy_double_push_pawns)
}


pub fn unapply_castle_move<'a>(board : &'a mut BitBoard , turn : Player , mov : u16 , castle_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };
    let king_index = get_lsb(ally_pieces.kings);

    if mov >> 12 == CASTLE_KING {
        reset_bit(&mut ally_pieces.kings, king_index );
        reset_bit(&mut ally_pieces.occupied, king_index );
        
        reset_bit(&mut ally_pieces.rooks, king_index - 1);
        reset_bit(&mut ally_pieces.occupied, king_index - 1);

        set_bit(&mut ally_pieces.kings, king_index - 2);
        set_bit(&mut ally_pieces.occupied, king_index - 2);
        
        set_bit(&mut ally_pieces.rooks, king_index + 1);
        set_bit(&mut ally_pieces.occupied, king_index + 1);
    } else {
        reset_bit(&mut ally_pieces.kings, king_index);
        reset_bit(&mut ally_pieces.occupied, king_index);
        
        reset_bit(&mut ally_pieces.rooks, king_index + 1);
        reset_bit(&mut ally_pieces.occupied, king_index + 1);
        
        set_bit(&mut ally_pieces.rooks, king_index - 2);
        set_bit(&mut ally_pieces.occupied, king_index - 2);
        
        set_bit(&mut ally_pieces.kings, king_index + 2);
        set_bit(&mut ally_pieces.occupied, king_index + 2);
    }
    ally_pieces.castle_rooks = castle_rooks;
    enemy_pieces.double_push_pawns = enemy_double_pawn_push;

}


pub fn apply_promotion(board : & mut BitBoard , turn : Player , mov : u16) ->  (PieceType , Option<PieceType> , u64 ,  u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let dest_piece_type = get_piece_type(&enemy_pieces, dest);

    let promotion_type = match mov >> 12 {
        KNIGHT_PROMOTED => PieceType::Knight,
        BISHOP_PROMOTED => PieceType::Bishop,
        ROOK_PROMOTED => PieceType::Rook,
        QUEEN_PROMOTED => PieceType::Queen,
        _ => unreachable!()
    };

    
    let enemy_rooks = enemy_pieces.castle_rooks;

    reset_bit(&mut ally_pieces.pawns, src);
    reset_bit(&mut ally_pieces.occupied, src);
    
    set_bit(&mut ally_pieces.occupied, dest);
    let promoted_bitset = get_piece_bitset(ally_pieces, &promotion_type);
    set_bit(promoted_bitset, dest);

    if let Some(piece_type) = dest_piece_type.as_ref() {
        if piece_type == &PieceType::Rook {
            reset_bit(&mut enemy_pieces.castle_rooks, dest);
        }
        let dest_piece = get_piece_bitset(enemy_pieces, piece_type);
        reset_bit(dest_piece, dest);
        reset_bit(&mut enemy_pieces.occupied, dest);
    }
    
    let enemy_double_push_pawns = enemy_pieces.double_push_pawns;
    enemy_pieces.double_push_pawns = 0;

    (promotion_type , dest_piece_type , enemy_rooks , enemy_double_push_pawns)
}


pub fn unapply_promotion(board : & mut BitBoard , turn : Player , mov : u16, promoted_type:PieceType, dest_piece_type : Option<PieceType> 
    , enemy_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let promoted_bitset = get_piece_bitset(ally_pieces, &promoted_type);

    reset_bit(promoted_bitset, dest);
    set_bit(&mut ally_pieces.pawns, src);
    reset_bit(&mut ally_pieces.occupied, dest);
    set_bit(&mut ally_pieces.occupied, src);

    if let Some(dest_piece) = dest_piece_type {
        let dest_piece = get_piece_bitset(enemy_pieces, &dest_piece);
        set_bit(dest_piece, dest);
        set_bit(&mut enemy_pieces.occupied, dest);
    }

    enemy_pieces.castle_rooks = enemy_rooks;   
    enemy_pieces.double_push_pawns = enemy_double_pawn_push;
}

pub fn apply_enpessant(board : & mut BitBoard , turn : Player , mov : u16) ->  (u64 , u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let capture_direction = if turn == Player::Black {8} else {-8};
    
    reset_bit(&mut ally_pieces.pawns, src);
    set_bit(&mut ally_pieces.pawns, dest);
    reset_bit(&mut ally_pieces.occupied, src);
    set_bit(&mut ally_pieces.occupied, dest);
    
    reset_bit(&mut enemy_pieces.pawns , (dest as i32 + capture_direction )as usize);
    reset_bit(&mut enemy_pieces.occupied, (dest as i32 + capture_direction) as usize);
    
    let enemy_double_push_pawns = enemy_pieces.double_push_pawns;
    enemy_pieces.double_push_pawns = 0;
    (enemy_double_push_pawns , 0)
}


pub fn unpply_enpessant(board : & mut BitBoard , turn : Player , mov : u16, enemy_double_push_pawns : u64){
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let capture_direction = if turn == Player::Black {8} else {-8};

    set_bit(&mut ally_pieces.pawns, src);
    set_bit(&mut ally_pieces.occupied, src);
    
    reset_bit(&mut ally_pieces.pawns, dest);
    reset_bit(&mut ally_pieces.occupied, dest);

    set_bit(&mut enemy_pieces.pawns , (dest as i32 + capture_direction )as usize);
    set_bit(&mut enemy_pieces.occupied, (dest as i32 + capture_direction) as usize);

    enemy_pieces.double_push_pawns = enemy_double_push_pawns;
}


pub fn apply_double_pawn_push<'a>(board : & 'a mut BitBoard , turn : Player , mov : u16) -> u64{
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let double_push_pawns = if Player::Black == turn {
        1u64 << (dest + 8)
    } else {
        1u64 << (dest - 8)
    };

    reset_bit(&mut ally_pieces.pawns, src);
    reset_bit(&mut ally_pieces.occupied, src);

    set_bit(&mut ally_pieces.pawns, dest);
    set_bit(&mut ally_pieces.occupied, dest);
    ally_pieces.double_push_pawns = double_push_pawns;

    let enemy_double_push_pawns = enemy_pieces.double_push_pawns;
    enemy_pieces.double_push_pawns = 0;
    enemy_double_push_pawns

}

pub fn unapply_double_pawn_push<'a>(board : & 'a mut BitBoard , turn : Player , mov : u16 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == Player::White  {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    
    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;
    
    reset_bit(&mut ally_pieces.pawns, dest);
    reset_bit(&mut ally_pieces.occupied, dest);
    
    set_bit(&mut ally_pieces.pawns, src);
    enemy_pieces.double_push_pawns = enemy_double_pawn_push;
    set_bit(&mut ally_pieces.occupied, src);
    ally_pieces.double_push_pawns = 0;
}
