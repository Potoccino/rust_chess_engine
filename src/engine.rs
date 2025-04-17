use crate::{attack_maps::{DIAGONAL_RAYS, KING_ATTACKS, KNIGHT_JUMPS, STRAIGHT_RAYS}, 
bit_board::{BitBoard, PieceType}, 
move_generator::{generate_diagonal_moves, generate_king_attacks, generate_king_moves, generate_knight_moves,
     generate_pawn_attacks, generate_pawn_moves, generate_straight_moves, iterate_attack_moves, iterate_possible_move}, 
     piece_set::PieceSet, utils::{flip_bit, get_lsb, print_bitset, print_board, read_move_components, reset_bit, set_bit, test_bit}};


const KNIGHT_PROMOTED : u16 = 1;
const BISHOP_PROMOTED : u16 = 2;
const ROOK_PROMOTED : u16 = 3;
const QUEEN_PROMOTED : u16 = 4;

const EN_PESSANT : u16 = 5;

const CASTLE_KING : u16 = 6;
const CASTLE_QUEEN : u16 = 7;

const DOUBLE_PAWN_PUSH : u16= 8;


pub enum MoveResult {
    Value(u64 , u64),
    Promotions(PieceType, Option<PieceType> , u64 , u64),
    Normal(PieceType, Option<PieceType> , u64 , u64 , u64),
    None(u64),
}


pub fn king_in_check<>( board : &BitBoard  , turn : bool) -> bool {
    
    let ally : &PieceSet = if turn == false {&board.white_set} else {&board.black_set};
    let enemy : &PieceSet = if turn == false {&board.black_set} else {&board.white_set};

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
    
    let attack_direction :i8 = if turn {-8 + king_index as i8 } else {8 + king_index as i8 };
    
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


pub fn generate_moves(board : &BitBoard , turn : bool) -> Vec<u16> {
    let mut moves : Vec<u16> = Vec::new();
    
    let ally : &PieceSet = if turn == false {&board.white_set} else {&board.black_set};
    let enemy : &PieceSet = if turn == false {&board.black_set} else {&board.white_set};

    let occupied = ally.occupied | enemy.occupied;

    iterate_possible_move(ally.bishops, ally, enemy, 0, 
        |index, occupied| generate_diagonal_moves(index, *occupied),
        occupied, &mut moves
    );   

    iterate_possible_move(ally.queens, ally, enemy, 0,  
        |index, occupied| generate_diagonal_moves(index, *occupied),
        occupied, &mut moves
    );

    iterate_possible_move(ally.queens, ally, enemy, 0, 
        |index, occupied| generate_straight_moves(index, *occupied),
        occupied, &mut moves
    );

    iterate_possible_move(ally.rooks, ally, enemy, 0,  
        |index, occupied| generate_straight_moves(index, *occupied),
        occupied, &mut moves
    );

    iterate_possible_move(ally.knights, ally, enemy, 0, 
        |index, _| generate_knight_moves(index),
        (), &mut moves
    );

    iterate_possible_move(ally.pawns, ally, enemy, 1,
        |index, args| {
            let (occupied, turn, double_pawn_push) = *args;
            generate_pawn_moves(index, occupied, turn as usize, double_pawn_push)
        },
        (occupied, turn, enemy.double_push_pawns),
        &mut moves
    );
    
    iterate_possible_move(ally.kings, ally, enemy, 2,
        |index, args| {
            let (occupied, castle_rooks, turn) = *args;
            generate_king_moves(index, occupied, castle_rooks, turn, enemy.attack_map)
        },
        (occupied, ally.castle_rooks, turn),
        &mut moves
    );


    moves
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


pub fn apply_normal_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16) -> (PieceType, Option<PieceType> , u64 , u64, u64) {

    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn unapply_normal_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16, src_piece_type : PieceType, dest_piece_type : Option<PieceType> , ally_rooks : u64,
        enemy_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn apply_castle_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16) -> (u64 , u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn unapply_castle_move<'a>(board : &'a mut BitBoard , turn : bool , mov : u16 , castle_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn apply_promotion(board : & mut BitBoard , turn : bool , mov : u16) ->  (PieceType , Option<PieceType> , u64 ,  u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn unapply_promotion(board : & mut BitBoard , turn : bool , mov : u16, promoted_type:PieceType, dest_piece_type : Option<PieceType> 
    , enemy_rooks : u64 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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

    enemy_pieces.double_push_pawns = enemy_double_pawn_push;
}

pub fn apply_enpessant(board : & mut BitBoard , turn : bool , mov : u16) ->  (u64 , u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let capture_direction = if turn {8} else {-8};
    
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


pub fn unpply_enpessant(board : & mut BitBoard , turn : bool , mov : u16, enemy_double_push_pawns : u64){
    let (ally_pieces,  enemy_pieces) = if turn == false {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let capture_direction = if turn {8} else {-8};

    set_bit(&mut ally_pieces.pawns, src);
    set_bit(&mut ally_pieces.occupied, src);
    
    reset_bit(&mut ally_pieces.pawns, dest);
    reset_bit(&mut ally_pieces.occupied, dest);

    set_bit(&mut enemy_pieces.pawns , (dest as i32 + capture_direction )as usize);
    set_bit(&mut enemy_pieces.occupied, (dest as i32 + capture_direction) as usize);

    enemy_pieces.double_push_pawns = enemy_double_push_pawns;
}


pub fn apply_double_pawn_push<'a>(board : & 'a mut BitBoard , turn : bool , mov : u16) -> u64{
    let (ally_pieces,  enemy_pieces) = if turn == false {
        (&mut board.white_set, &mut board.black_set)
    } else {
        (&mut board.black_set, &mut board.white_set)
    };

    let src = mov as usize & 0x3F;
    let dest = (mov as usize >> 6) & 0x3F;

    let double_push_pawns = if turn {
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

pub fn unapply_double_pawn_push<'a>(board : & 'a mut BitBoard , turn : bool , mov : u16 , enemy_double_pawn_push : u64) {
    let (ally_pieces,  enemy_pieces) = if turn == false {
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


pub fn apply_move<>(board : &  mut BitBoard , turn : bool , mov : u16) -> MoveResult {
        match mov >> 12 {
        KNIGHT_PROMOTED | BISHOP_PROMOTED | ROOK_PROMOTED | QUEEN_PROMOTED => {
            let (promoted_piece_type, dest_piece_type, enemy_rooks,enemy_double_pawn_push) = apply_promotion(board, turn, mov);
            MoveResult::Promotions(promoted_piece_type ,  dest_piece_type , enemy_rooks , enemy_double_pawn_push)
        },
        EN_PESSANT => {
            let (enemy_double_push_pawn , _) = apply_enpessant(board, turn, mov);
            MoveResult::Value(enemy_double_push_pawn , 0)  
        },
        CASTLE_KING | CASTLE_QUEEN => {
            let (castle_rooks , enemy_double_pawn_push) = apply_castle_move(board, turn, mov);
            MoveResult::Value(castle_rooks , enemy_double_pawn_push)
        },
        DOUBLE_PAWN_PUSH => {
            let enemy_double_pawn_push =  apply_double_pawn_push(board, turn, mov);
            MoveResult::None(enemy_double_pawn_push)
        },        
        _ => {
            let (src_piece_type , dest_piece_type , castle_rooks , enemy_rooks , enemy_double_pawn_push ) = apply_normal_move(board, turn, mov);
            MoveResult::Normal(src_piece_type, dest_piece_type , castle_rooks, enemy_rooks , enemy_double_pawn_push)
        }
    }

}

pub fn unapply_move<'a>(board : & 'a mut BitBoard , turn : bool , mov : u16 , mov_result : MoveResult) {
    match mov >> 12 {
        KNIGHT_PROMOTED | BISHOP_PROMOTED | ROOK_PROMOTED | QUEEN_PROMOTED => {
            if let MoveResult::Promotions(promoted_piece_type, dest_piece_type, enemy_rooks, enemy_double_pawn_push) = mov_result {
                unapply_promotion(board, turn, mov, promoted_piece_type, dest_piece_type , enemy_rooks ,enemy_double_pawn_push);
            }
        },
        EN_PESSANT => {
            if let MoveResult::Value(enemy_double_push_pawns, _) = mov_result {
                unpply_enpessant(board, turn, mov, enemy_double_push_pawns);
            }
        },
        CASTLE_KING | CASTLE_QUEEN => {
            if let MoveResult::Value(castle_rooks , enemy_double_pawn_push) = mov_result {
                unapply_castle_move(board, turn, mov, castle_rooks , enemy_double_pawn_push);
            }
        },
        DOUBLE_PAWN_PUSH => {
            if let MoveResult::None(enemy_double_pawn_push) = mov_result {
                unapply_double_pawn_push(board, turn, mov, enemy_double_pawn_push);
            }
        },
        _ => {
            if let MoveResult::Normal(src_piece_type, dest_piece_type , rooks , enemy_rooks ,enemy_double_pawn_push) = mov_result {
                unapply_normal_move(board, turn, mov, src_piece_type, dest_piece_type , rooks , enemy_rooks ,  enemy_double_pawn_push);
            }
        }
    }
}




pub fn run_engine<'a>(board : & 'a mut BitBoard , mut turn : bool){
    loop {

        print_board(&board);

        print_bitset(&board.white_set.castle_rooks);
        board.white_set.attack_map = generate_attack_maps(board, false);
        board.black_set.attack_map = generate_attack_maps(board, true);



        let moves = generate_moves(&board, turn );


        // get the best move from the moves 
        // or in this case take input from the user
        
        let (src , dest , speical) = match  read_move_components(){
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

        let mov_result: MoveResult = apply_move(board, turn, mov);

        let in_check = king_in_check(board, turn);

        if in_check{
            unapply_move(board, turn, mov, mov_result);
            println!("Invalid move, king in check");
            continue;
        }
      
        turn = !turn;
    }

}


pub fn generate_attack_maps(board : &  mut BitBoard , turn : bool) -> u64
{
    let mut attacks = 0;
    let occupied = board.black_set.occupied | board.white_set.occupied;
    let ally_pieces = if turn == false {&board.white_set} else {&board.black_set};

    attacks |= iterate_attack_moves(
        ally_pieces.pawns, 
        |index , turn|generate_pawn_attacks(index , *turn),
        turn
    );

    attacks |= iterate_attack_moves(
        ally_pieces.knights, 
            |index , _| generate_knight_moves(index),
        ()
    );

    attacks |= iterate_attack_moves(ally_pieces.kings,
        |index , _ | generate_king_attacks(index) ,
        ()
    );


    attacks |= iterate_attack_moves(ally_pieces.rooks, 
        |index , occupied | generate_straight_moves(index, *occupied), 
        occupied
    );

    attacks |= iterate_attack_moves(ally_pieces.queens, 
        |index , occupied | generate_straight_moves(index, *occupied), 
        occupied
    );

    attacks |= iterate_attack_moves(ally_pieces.queens, 
        |index , occupied | generate_diagonal_moves(index, *occupied), 
        occupied
    );

    attacks |= iterate_attack_moves(ally_pieces.bishops, 
        |index , occupied | generate_diagonal_moves(index, *occupied), 
        occupied
    );

    attacks
}