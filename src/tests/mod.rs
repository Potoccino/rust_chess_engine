

#[allow(unused)]#[cfg(test)]
mod tests {
    use std::collections::hash_map;
    use std::collections::HashMap;
    use std::hash::DefaultHasher;
    use std::hash::Hash;

    use super::*;

    use crate::bit_board::BitBoard;
    use crate::piece_set::PieceSet;
    use crate::utils::*;
    use crate::engine::*;

    const CASTLE_KING: u16 = 6; // Adjust to match your actual constant value
    const CASTLE_QUEEN: u16 = 7; // Adjust to match your actual constant value

    // Helper function to create a clean board with kings and rooks in castling position
    fn setup_castling_board(side: bool) -> BitBoard {
        let mut board = BitBoard::get_empty_board(); // Assuming there's a default implementation
        
            board.white_set.kings = 1u64 << 4; // King at e1
            board.white_set.rooks = (1u64 << 0) | (1u64 << 7); // Rooks at a1 and h1
            board.white_set.occupied = board.white_set.kings | board.white_set.rooks;
            board.white_set.castle_rooks = board.white_set.rooks; // Mark both rooks as castling rooks

            board.black_set.kings = 1u64 << 60; // King at e8
            board.black_set.rooks = (1u64 << 56) | (1u64 << 63); // Rooks at a8 and h8
            board.black_set.occupied = board.black_set.kings | board.black_set.rooks;
            board.black_set.castle_rooks = board.black_set.rooks; // Mark both rooks as castling rooks
        
        board
    }

    // Helper function to check if two bitboards are completely equal
    fn compare_boards(board1: &BitBoard, board2: &BitBoard) -> bool {
        // Compare white pieces
        board1.white_set.kings == board2.white_set.kings &&
        board1.white_set.queens == board2.white_set.queens &&
        board1.white_set.rooks == board2.white_set.rooks &&
        board1.white_set.bishops == board2.white_set.bishops &&
        board1.white_set.knights == board2.white_set.knights &&
        board1.white_set.pawns == board2.white_set.pawns &&
        board1.white_set.occupied == board2.white_set.occupied &&
        board1.white_set.castle_rooks == board2.white_set.castle_rooks &&
        
        // Compare black pieces
        board1.black_set.kings == board2.black_set.kings &&
        board1.black_set.queens == board2.black_set.queens &&
        board1.black_set.rooks == board2.black_set.rooks &&
        board1.black_set.bishops == board2.black_set.bishops &&
        board1.black_set.knights == board2.black_set.knights &&
        board1.black_set.pawns == board2.black_set.pawns &&
        board1.black_set.occupied == board2.black_set.occupied &&
        board1.black_set.castle_rooks == board2.black_set.castle_rooks
    }

    // Helper function to print board for debugging
    fn print_board_state(board: &BitBoard, side: bool) {
        let pieces = if side == false { &board.white_set } else { &board.black_set };
        println!("Kings: {:#066b}", pieces.kings);
        println!("Rooks: {:#066b}", pieces.rooks);
        println!("Occupied: {:#066b}", pieces.occupied);
        println!("Castle Rooks: {:#066b}", pieces.castle_rooks);
    }

    #[test]
    fn test_white_kingside_castle() {
        // Setup
        let mut board = setup_castling_board(false); // White castling
        let initial_board = setup_castling_board(false); // Keep a copy of the initial state
        let king_side_castle = CASTLE_KING << 12; // Create kingside castle move
        
        // Save the initial castle_rooks value
        let initial_castle_rooks = board.white_set.castle_rooks;
        
        // Act
        let saved_castle_rooks = apply_castle_move(&mut board, false, king_side_castle);
        
        // Assert
        assert_eq!(saved_castle_rooks, initial_castle_rooks); // Check that returned value matches initial value
        assert_eq!(board.white_set.kings, 1u64 << 6); // King should be at g1
        assert_eq!(board.white_set.rooks, (1u64 << 0) | (1u64 << 5)); // Rooks at a1 and f1
        assert_eq!(board.white_set.occupied, (1u64 << 6) | (1u64 << 0) | (1u64 << 5)); // King and rooks
        assert_eq!(board.white_set.castle_rooks, 0); // Castle rooks should be 0
        
        // Test unapply
        unapply_castle_move(&mut board, false, king_side_castle, saved_castle_rooks);
        
        assert_eq!(board.white_set.kings, 1u64 << 4); // King should be at e1   
        assert_eq!(board.white_set.rooks, (1u64 << 0) | (1u64 << 7)); // Rooks at a1 and h1
        assert_eq!(board.white_set.occupied, (1u64 << 4) | (1u64 << 0) | (1u64 << 7)); // King and rooks
        assert_eq!(board.white_set.castle_rooks, initial_castle_rooks); // Castle rooks should be restored

        assert!(board.white_set.kings == initial_board.white_set.kings);
        assert!(board.white_set.rooks == initial_board.white_set.rooks);
        assert!(board.white_set.occupied == initial_board.white_set.occupied);
        assert!(board.white_set.castle_rooks == initial_board.white_set.castle_rooks);
        // The board should be completely identical to the initial board
        assert!(compare_boards(&board, &initial_board));
    }

    #[test]
    fn test_white_queenside_castle() {
        // Setup
        let mut board = setup_castling_board(false); // White castling
        let initial_board = setup_castling_board(false); // Keep a copy of the initial state
        let queen_side_castle = CASTLE_QUEEN << 12; // Create queenside castle move
        
        // Save the initial castle_rooks value
        let initial_castle_rooks = board.white_set.castle_rooks;
        
        // Act
        let saved_castle_rooks = apply_castle_move(&mut board, false, queen_side_castle);
        
        // Assert
        assert_eq!(saved_castle_rooks, initial_castle_rooks); // Check that returned value matches initial value
        assert_eq!(board.white_set.kings, 1u64 << 2); // King should be at c1
        assert_eq!(board.white_set.rooks, (1u64 << 3) | (1u64 << 7)); // Rooks at d1 and h1
        assert_eq!(board.white_set.occupied, (1u64 << 2) | (1u64 << 3) | (1u64 << 7)); // King and rooks
        assert_eq!(board.white_set.castle_rooks, 0); // Castle rooks should be 0
        
        // Test unapply
        unapply_castle_move(&mut board, false, queen_side_castle, saved_castle_rooks);
        
        // The board should be completely identical to the initial board
        assert!(compare_boards(&board, &initial_board));
    }

    #[test]
    fn test_black_kingside_castle() {
        // Setup
        let mut board = setup_castling_board(true); // Black castling
        let initial_board = setup_castling_board(true); // Keep a copy of the initial state
        let king_side_castle = CASTLE_KING << 12; // Create kingside castle move
        
        // Save the initial castle_rooks value
        let initial_castle_rooks = board.black_set.castle_rooks;
        
        // Act
        let saved_castle_rooks = apply_castle_move(&mut board, true, king_side_castle);
        
        // Assert
        assert_eq!(saved_castle_rooks, initial_castle_rooks); // Check that returned value matches initial value
        assert_eq!(board.black_set.kings, 1u64 << 62); // King should be at g8
        assert_eq!(board.black_set.rooks, (1u64 << 56) | (1u64 << 61)); // Rooks at a8 and f8
        assert_eq!(board.black_set.occupied, (1u64 << 62) | (1u64 << 56) | (1u64 << 61)); // King and rooks
        assert_eq!(board.black_set.castle_rooks, 0); // Castle rooks should be 0
        
        // Test unapply
        unapply_castle_move(&mut board, true, king_side_castle, saved_castle_rooks);
        
        // The board should be completely identical to the initial board
        assert!(compare_boards(&board, &initial_board));
    }

    #[test]
    fn test_black_queenside_castle() {
        // Setup
        let mut board = setup_castling_board(true); // Black castling
        let initial_board = setup_castling_board(true); // Keep a copy of the initial state
        let queen_side_castle = CASTLE_QUEEN << 12; // Create queenside castle move
        
        // Save the initial castle_rooks value
        let initial_castle_rooks = board.black_set.castle_rooks;
        
        // Act
        let saved_castle_rooks = apply_castle_move(&mut board, true, queen_side_castle);
        
        // Assert
        assert_eq!(saved_castle_rooks, initial_castle_rooks); // Check that returned value matches initial value
        assert_eq!(board.black_set.kings, 1u64 << 58); // King should be at c8
        assert_eq!(board.black_set.rooks, (1u64 << 59) | (1u64 << 63)); // Rooks at d8 and h8
        assert_eq!(board.black_set.occupied, (1u64 << 58) | (1u64 << 59) | (1u64 << 63)); // King and rooks
        assert_eq!(board.black_set.castle_rooks, 0); // Castle rooks should be 0
        
        // Test unapply
        unapply_castle_move(&mut board, true, queen_side_castle, saved_castle_rooks);
        
        // The board should be completely identical to the initial board
        assert!(compare_boards(&board, &initial_board));
    }

    #[test]
    fn test_apply_unapply_sequence() {
        // This test verifies that applying and unapplying multiple castling moves works correctly
        let mut board = setup_castling_board(false); // Start with white
        let initial_board = setup_castling_board(false); // Keep a copy of the initial state
        
        // Save the initial castle_rooks values
        let white_initial_castle_rooks = board.white_set.castle_rooks;
        let black_initial_castle_rooks = board.black_set.castle_rooks;
        
        // White kingside castle
        let white_king_castle = CASTLE_KING << 12;
        let saved_white_castle_rooks = apply_castle_move(&mut board, false, white_king_castle);
        
        // Verify white castle_rooks is now 0
        assert_eq!(board.white_set.castle_rooks, 0);
        assert_eq!(saved_white_castle_rooks, white_initial_castle_rooks);
        
        // Black queenside castle
        let black_queen_castle = CASTLE_QUEEN << 12;
        let saved_black_castle_rooks = apply_castle_move(&mut board, true, black_queen_castle);
        
        // Verify black castle_rooks is now 0
        assert_eq!(board.black_set.castle_rooks, 0);
        assert_eq!(saved_black_castle_rooks, black_initial_castle_rooks);
        
        // Now unapply in reverse order
        unapply_castle_move(&mut board, true, black_queen_castle, saved_black_castle_rooks);
        
        // Verify black castle_rooks is restored
        assert_eq!(board.black_set.castle_rooks, black_initial_castle_rooks);
        
        unapply_castle_move(&mut board, false, white_king_castle, saved_white_castle_rooks);
        
        // Verify white castle_rooks is restored
        assert_eq!(board.white_set.castle_rooks, white_initial_castle_rooks);
        
        // Final board should match initial board completely
        assert!(compare_boards(&board, &initial_board));
    }
    
    #[test]
    fn test_partial_castle_rights() {
        // This test verifies handling of partial castling rights
        // (e.g. when only one rook can castle)
        
        // Setup board with only kingside castling rights
        let mut board = setup_castling_board(false);
        board.white_set.castle_rooks = 1u64 << 7; // Only h1 rook can castle
        
        let initial_board = board.clone(); // Keep a copy
        
        // Do kingside castle
        let king_side_castle = CASTLE_KING << 12;
        let saved_castle_rooks = apply_castle_move(&mut board, false, king_side_castle);
        
        // Verify saved value
        assert_eq!(saved_castle_rooks, 1u64 << 7);
        assert_eq!(board.white_set.castle_rooks, 0);
        
        // Unapply
        unapply_castle_move(&mut board, false, king_side_castle, saved_castle_rooks);
        
        // Verify restoration
        assert_eq!(board.white_set.castle_rooks, 1u64 << 7);
        assert!(compare_boards(&board, &initial_board));
    }

    #[test]
    fn test_apply_unapply_double_pawn_push() {
        // Create a test board
        let mut board = BitBoard::get_empty_board(); // Assuming you have a default constructor
        
        // Set up initial board state with a white pawn at e2 (index 12)
        let src = 12; // e2 square
        let dest = 28; // e4 square (double push)
        let mov = (dest as u16) << 6 | (src as u16);
        
        // Set initial state
        set_bit(&mut board.white_set.pawns, src);
        set_bit(&mut board.white_set.occupied, src);
        
        // Save initial state for comparison
        let initial_white_pawns = board.white_set.pawns;
        let initial_white_occupied = board.white_set.occupied;
        let initial_double_push_pawns = board.white_set.double_push_pawns;
        
        // Apply move
        apply_double_pawn_push(&mut board, false, mov);
        
        // Check that the double push was applied correctly
        assert_eq!(get_bit(board.white_set.pawns, src), false, "Pawn should be removed from src");
        assert_eq!(get_bit(board.white_set.occupied, src), false, "Occupied bit should be removed from src");
        assert_eq!(get_bit(board.white_set.pawns, dest), true, "Pawn should be added to dest");
        assert_eq!(get_bit(board.white_set.occupied, dest), true, "Occupied bit should be added to dest");
        
        // Check that the double push flag is set correctly
        // For white, the en passant square would be e3 (index 20)
        assert_eq!(board.white_set.double_push_pawns, 1u64 << 20, "Double push flag should be set at e3");
        
        // Unapply move
        unapply_doublw_pawn_push(&mut board, false, mov);
        
        // Check that the board is restored to its initial state
        assert_eq!(board.white_set.pawns, initial_white_pawns, "Pawns should be restored to initial state");
        assert_eq!(board.white_set.occupied, initial_white_occupied, "Occupied bits should be restored");
        assert_eq!(board.white_set.double_push_pawns, 0, "Double push flag should be reset to 0");
        
        // Now test for black pawn (from e7 to e5)
        let mut board = BitBoard::get_empty_board();
        
        // Set up initial board state with a black pawn at e7 (index 52)
        let src = 52; // e7 square
        let dest = 36; // e5 square (double push)
        let mov = (dest as u16) << 6 | (src as u16);
        
        // Set initial state
        set_bit(&mut board.black_set.pawns, src);
        set_bit(&mut board.black_set.occupied, src);
        
        // Save initial state for comparison
        let initial_black_pawns = board.black_set.pawns;
        let initial_black_occupied = board.black_set.occupied;
        let initial_double_push_pawns = board.black_set.double_push_pawns;
        
        // Apply move
        apply_double_pawn_push(&mut board, true, mov);
        
        // Check that the double push was applied correctly
        assert_eq!(get_bit(board.black_set.pawns, src), false, "Black pawn should be removed from src");
        assert_eq!(get_bit(board.black_set.occupied, src), false, "Black occupied bit should be removed from src");
        assert_eq!(get_bit(board.black_set.pawns, dest), true, "Black pawn should be added to dest");
        assert_eq!(get_bit(board.black_set.occupied, dest), true, "Black occupied bit should be added to dest");
        
        // Check that the double push flag is set correctly
        // For black, the en passant square would be e6 (index 44)
        assert_eq!(board.black_set.double_push_pawns, 1u64 << 44, "Double push flag should be set at e6");
        
        // Unapply move
        unapply_doublw_pawn_push(&mut board, true, mov);
        
        // Check that the board is restored to its initial state
        assert_eq!(board.black_set.pawns, initial_black_pawns, "Black pawns should be restored to initial state");
        assert_eq!(board.black_set.occupied, initial_black_occupied, "Black occupied bits should be restored");
        assert_eq!(board.black_set.double_push_pawns, 0, "Double push flag should be reset to 0");
    }


    

    fn preft_helper(mut board :  &mut BitBoard , turn : bool , depth : i32 , hash_map : & mut HashMap<BitBoard , u64>) -> u64{
        if depth == 0 {
            return 1;
        }

        board.white_set.attack_map = generate_attack_maps(&mut board, false);
        board.black_set.attack_map = generate_attack_maps(&mut board, true);
        
        let moves = generate_moves(&mut board, turn);
        let mut move_count = 0 ;

        for mov in moves {
            let mov_result = apply_move(&mut board, turn, mov);
            if king_in_check(&board, turn) {
                unapply_move(&mut board, turn, mov, mov_result);
                continue;
            }
            move_count += preft_helper(board, !turn, depth - 1 , hash_map);
            unapply_move(&mut board, turn, mov, mov_result);
        }

        move_count
    }

    #[test]
    fn perft_test(){
        let mut board = BitBoard::get_starting_board();
        let mut hash_map : HashMap<BitBoard, u64> = HashMap::new();


        let move_count = preft_helper(&mut board, false, 4 , &mut hash_map);
        println!("{}" , move_count);

    }


}
