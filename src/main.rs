mod board;
mod board_util;
mod structures;
mod enums;
use board::Board;
use enums::PieceColor;
use enums::PieceType;
use structures::Piece;
use structures::Square; 

fn main() {
     let b = Board::new();
    
    board_util::print_board(&b);
}



