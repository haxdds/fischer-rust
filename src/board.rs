use std::collections::HashMap;

use crate::structures:: {Square, Piece, MoveHistory};
use crate::enums:: {PieceType, PieceColor};

pub struct Board{

    pub squares: HashMap<Square, Piece>,
    pub history: MoveHistory

}

// impl Clone for Board{

//     fn clone(&self) -> Self {

//         let new_history = self.history.clone();
//         let mut new_squares = HashMap::<Square, Piece>::new();

//         for (square, piece) in self.squares {
//             let new_square = square.clone();
//             let new_piece = piece.clone();
//             new_squares.insert(new_square, new_piece);
//         }

//         Board{ squares: new_squares, history: new_history }

//     }
// }


impl Board{

    pub fn new() -> Board{

        let squares = Board::setup_board();

        let mut history = MoveHistory::new();

        Board{  
            squares: squares,
            history: history
        }
    }


    fn movePiece(&self, s1: &Square, s2: &Square) -> bool {
        false
    }

    fn setup_board() -> HashMap<Square, Piece>{

        let mut squares = HashMap::<Square, Piece>::new();
        let white_pawn = Piece::new(PieceType::PAWN, PieceColor::WHITE);
        let white_rook = Piece::new(PieceType::ROOK, PieceColor::WHITE);
        let white_knight = Piece::new(PieceType::KNIGHT, PieceColor::WHITE);
        let white_bishop = Piece::new(PieceType::BISHOP, PieceColor::WHITE);
        let white_queen = Piece::new(PieceType::QUEEN, PieceColor::WHITE);
        let white_king = Piece::new(PieceType::KING, PieceColor::WHITE);

        let black_pawn = Piece::new(PieceType::PAWN, PieceColor::BLACK);
        let black_rook = Piece::new(PieceType::ROOK, PieceColor::BLACK);
        let black_knight = Piece::new(PieceType::KNIGHT, PieceColor::BLACK);
        let black_bishop = Piece::new(PieceType::BISHOP, PieceColor::BLACK);
        let black_queen = Piece::new(PieceType::QUEEN, PieceColor::BLACK);
        let black_king = Piece::new(PieceType::KING, PieceColor::BLACK);

        squares.insert(Square::new_alg('a', 2), white_pawn.clone());
        squares.insert(Square::new_alg('b', 2), white_pawn.clone());
        squares.insert(Square::new_alg('c', 2), white_pawn.clone());
        squares.insert(Square::new_alg('d', 2), white_pawn.clone());
        squares.insert(Square::new_alg('e', 2), white_pawn.clone());
        squares.insert(Square::new_alg('f', 2), white_pawn.clone());
        squares.insert(Square::new_alg('g', 2), white_pawn.clone());
        squares.insert(Square::new_alg('h', 2), white_pawn.clone());

        squares.insert(Square::new_alg('a', 1), white_rook.clone());
        squares.insert(Square::new_alg('b', 1), white_knight.clone());
        squares.insert(Square::new_alg('c', 1), white_bishop.clone());
        squares.insert(Square::new_alg('d', 1), white_queen.clone());
        squares.insert(Square::new_alg('e', 1), white_king.clone());
        squares.insert(Square::new_alg('f', 1), white_bishop.clone());
        squares.insert(Square::new_alg('g', 1), white_knight.clone());
        squares.insert(Square::new_alg('h', 1), white_rook.clone());

        squares.insert(Square::new_alg('a', 7), black_pawn.clone());
        squares.insert(Square::new_alg('b', 7), black_pawn.clone());
        squares.insert(Square::new_alg('c', 7), black_pawn.clone());
        squares.insert(Square::new_alg('d', 7), black_pawn.clone());
        squares.insert(Square::new_alg('e', 7), black_pawn.clone());
        squares.insert(Square::new_alg('f', 7), black_pawn.clone());
        squares.insert(Square::new_alg('g', 7), black_pawn.clone());
        squares.insert(Square::new_alg('h', 7), black_pawn.clone());

        squares.insert(Square::new_alg('a', 8), black_rook.clone());
        squares.insert(Square::new_alg('b', 8), black_knight.clone());
        squares.insert(Square::new_alg('c', 8), black_bishop.clone());
        squares.insert(Square::new_alg('d', 8), black_queen.clone());
        squares.insert(Square::new_alg('e', 8), black_king.clone());
        squares.insert(Square::new_alg('f', 8), black_bishop.clone());
        squares.insert(Square::new_alg('g', 8), black_knight.clone());
        squares.insert(Square::new_alg('h', 8), black_rook.clone());

        let NULL_PIECE: Piece = Piece::new(PieceType::NULL, PieceColor::NULL);
        
        for col in 1..9 {
            for row in 3..7 {
                squares.insert(Square::new(col, row), NULL_PIECE.clone());
            }
        }

        squares

    }   

}