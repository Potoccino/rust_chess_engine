
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STRAIGHT_RAYS: [[u64; 65]; 4] = initialize_straight_rays();
    pub static ref DIAGONAL_RAYS: [[u64; 65]; 4] = initialize_diagonal_rays();
    pub static ref KNIGHT_JUMPS: [u64; 65] = initialize_knight_jumps();
    pub static ref KING_ATTACKS: [u64; 65] = initialize_king_attacks();
    pub static ref PAWN_PUSH: [[u64; 65]; 2] = initialize_pawn_push();
    pub static ref PAWN_CAPTURES: [[u64; 65]; 2] = initialize_pawn_captures();
}


fn initialize_straight_rays() -> [[u64; 65]; 4]  {
    let mut rays = [[0u64 ; 65] ; 4];

    let straight_rays_direction_and_limit = [
        (0 , 7) , (1 , 7) , (2 , 0) , (3 , 0)
    ];

    for (direction , limit) in straight_rays_direction_and_limit.iter() {
        let offset = if *direction == 0 || *direction == 2 {8} else {1};
        let offset = offset * if *direction == 0 || *direction == 1 {1} else {-1};

        let calculate_squares = |pos:usize| -> usize {
            if *direction == 0 || *direction== 2 {
                pos / 8
            } else {
                pos % 8
            }
        };

        for i in 0..64 {
            let mut current_position = i;

            let mut squares_to_cover = (calculate_squares(current_position) as i32 - *limit as i32).abs() as usize;

            let mut ray = 0u64;
            
            while squares_to_cover > 0 {
                if offset > 0 {
                    current_position += offset as usize;
                } else {
                    current_position -= (-offset) as usize;
                }
                ray |= 1u64 << current_position;
                squares_to_cover -= 1;
            }

            rays[*direction as usize][i] = ray;
        }
    }

    rays
}

fn initialize_diagonal_rays() -> [[u64 ; 65] ; 4] {
    let mut rays = [[0u64 ; 65] ; 4];


    let diagonal_rays_and_limits = [
        (0 , 7 , 7) , (1 , 0 , 7) ,
        (2 , 0 , 0) , (3 , 7 , 0)
    ];

    for (direction , veritcal_limit , horizontal_limit) in diagonal_rays_and_limits.iter() {
        let horizonal_offset = if *direction == 0 || *direction == 1 {1} else {-1};
        let vertical_offset = if *direction == 0 || *direction == 3 {8} else {-8};

        for i in 0..64 {
            let mut squares_to_cover = std::cmp::min(
                ((i % 8) as i32 - *horizontal_limit).abs() as usize ,
                ((i / 8) as i32 - *veritcal_limit).abs() as usize
            );

            let mut current_position = i;
            let mut ray = 0u64;

            while squares_to_cover > 0 {
                current_position = (current_position as i32) + vertical_offset + horizonal_offset;

                ray |= 1u64 << current_position;
                squares_to_cover -= 1;
            }

            rays [*direction as usize][i as usize] = ray;
        }
    }

    rays
}


fn initialize_knight_jumps() -> [u64 ; 65] {
    let knight_jumps = [
        (1 , 2),
        (1 , -2),
        (-1 , 2),
        (-1 , -2),
        (2 , 1),
        (2 , -1),
        (-2 , 1),
        (-2 , -1)
    ];

    let mut rays = [0u64 ; 65];

    for i in 0..64 {
        let mut ray = 0u64;
        for (h_offset , v_offset) in knight_jumps.iter() {
            let x = i % 8;
            let y = i / 8;

            if x + h_offset >= 8 || x + h_offset < 0 || y + v_offset < 0 || y + v_offset >= 8 {
                continue;
            }

            let new_position : usize = (i + (*h_offset as i32) + (*v_offset as i32) * 8 )as usize;
            ray |= 1u64 << new_position;
        }
        rays[i as usize] = ray;
    }

    rays
}


fn initialize_king_attacks() -> [u64 ; 65] {
    let mut attacks = [0u64 ; 65];
    
    let offsets = [
        (1 , 1) , (1 , 0) , (1 , -1),
        (-1 , 1) , (-1 , 0) , (-1 , -1),
        (0 , 1) , (0 , -1)
    ];

    for i in 0..64 {
        let mut ray = 0u64;
        for (v_offset , h_offset) in offsets.iter() {
            let x = i % 8;
            let y = i / 8;
            
            if x + h_offset >= 8 || x + h_offset < 0 || y + v_offset < 0 || y + v_offset >= 8 {
                continue;
            }
            
            let new_position : usize = (i + (*h_offset as i32) + (*v_offset as i32) * 8 )as usize;
            ray |= 1u64 << new_position;
        }
        attacks[i as usize] = ray;
    }

    attacks
}



fn initialize_pawn_captures() -> [[u64 ; 65] ; 2] {
    let mut attacks = [[0u64 ; 65] ; 2];

    for i in 8..56 {
        if i % 8 != 0 {
            attacks[0][i] |= 1u64 << (i + 7);
            attacks[1][i] |= 1u64 << (i - 9);
        }

        if i % 8 != 7 {
            attacks[0][i] |= 1u64 << (i + 9);
            attacks[1][i] |= 1u64 << (i - 7);    
        }
    }
    
    attacks 
}

fn initialize_pawn_push () -> [[u64 ; 65] ; 2] {
    let mut moves = [[0u64 ; 65] ; 2];

    for i in 8..56 {
        moves[0][i] |= 1u64 << (i + 8);
        moves[1][i] |= 1u64 << (i - 8);
    }    

    moves
}

