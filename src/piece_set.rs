

#[derive(Clone)]
pub struct PieceSet
{
    pub rooks : u64,
    pub knights : u64,
    pub bishops : u64,
    pub queens : u64,
    pub kings : u64,
    pub pawns : u64,
    pub occupied : u64,
    pub double_push_pawns : u64,
    pub castle_rooks : u64,   
}


impl PieceSet
{
    pub fn get_empty_piece_set() -> Self
    {
        let piece_set: PieceSet = PieceSet{
            rooks : 0,
            knights : 0,
            bishops : 0,
            queens : 0,
            kings : 0,
            pawns : 0,
            occupied : 0,
            double_push_pawns : 0,
            castle_rooks : 0,
        };
        return piece_set;
    }
    
    pub fn get_starting_white_set() -> Self {
        let mut set = PieceSet::get_empty_piece_set();
    
        set.rooks = 1 | (1 << 7);                   
        set.knights = (1 << 1) | (1 << 6);          
        set.bishops = (1 << 2) | (1 << 5);          
        set.queens = 1 << 3;                        
        set.kings = 1 << 4;                         
        set.pawns = 0xFF00;                         
        set.castle_rooks = set.rooks;               
        set.occupied = set.rooks | set.knights | set.bishops | set.queens | set.kings | set.pawns;
    
        set
    }
    
    pub fn get_starting_black_set() -> Self {
        let mut set = PieceSet::get_empty_piece_set();
    
        set.rooks = (1 << 56) | (1 << 63);         
        set.knights = (1 << 57) | (1 << 62);       
        set.bishops = (1 << 58) | (1 << 61);       
        set.queens = 1 << 59;                      
        set.kings = 1 << 60;                       
        set.pawns = 0xFF000000000000;              
        set.castle_rooks = set.rooks;             
        set.occupied = set.rooks | set.knights | set.bishops | set.queens | set.kings | set.pawns;
    
        set
    }
    

}

