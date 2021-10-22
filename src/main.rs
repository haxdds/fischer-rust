mod board;
mod structures;
mod enums;
use board::Board;
use enums::PieceColor;
use enums::PieceType;
use structures::Piece;
use structures::Square; 

fn main() {
    let b = Board::new();
    let NULL_PIECE: Piece = Piece::new(PieceType::NULL, PieceColor::NULL);

    for row in 1..9 {
        for col in 1..9 {
            let x = Square::new(col, row);
            let piece = b.squares.get(&x).unwrap();
            if piece == &NULL_PIECE {
                print!("-");
            }else{
                let p = &piece.ptype;
                match p {
                    PieceType::PAWN => print!("P"),
                    PieceType::ROOK => print!("R"),
                    PieceType::KNIGHT => print!("N"),
                    PieceType::BISHOP => print!("B"),
                    PieceType::QUEEN => print!("Q"),
                    PieceType::KING => print!("K"),
                    _ => print!("Err"),
                }
            }
            print!{"|"}
        }
        println!("");
    }
}


