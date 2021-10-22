mod board;

use board::Board;

fn main() {
    let b = Board::new();
   
}



// pub fn print(&self){
        
//     let NULL_PIECE: Piece = Piece::new(PieceType::NULL, PieceColor::NULL);

//     for (col, row) in self.squares {
//         if row == NULL_PIECE {
//             print!("-");
//         }else{
//             let p = row.ptype;
//             match p {
//                 PieceType::PAWN => print!("p"),
//                 PieceType::ROOK => print!("p"),
//                 PieceType::KNIGHT => print!("p"),
//                 PieceType::BISHOP => print!("p"),
//                 PieceType::QUEEN => print!("p"),
//                 PieceType::KING => print!("p"),
//                 _ => print!("Err"),
//             }
//         }
//     }
// }