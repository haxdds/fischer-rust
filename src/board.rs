use std::collections::HashMap;

use crate::structures:: {Square, Piece, MoveHistory};
use crate::enums:: {PieceType, PieceColor};

use crate::board_util;

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

        let squares = board_util::setup_board();

        let mut history = MoveHistory::new();

        Board{  
            squares: squares,
            history: history
        }
    }


    fn movePiece(&self, s1: &Square, s2: &Square) -> bool {
        false
    }

    

}