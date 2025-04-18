use std::io::{self, BufRead};


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

