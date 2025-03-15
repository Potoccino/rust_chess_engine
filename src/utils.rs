use crate::bit_board::BitBoard;

pub fn print_move(mov : &u16) {
    let src = mov & 0x3F;
    let dest = (mov >> 6) & 0x3F;  
    let special = (mov >> 12) & 0x7;

    println!("Move from {} to {} with special {}", src, dest, special);

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

