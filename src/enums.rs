#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PieceType {
    NULL,
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PieceColor {
    NULL,
    WHITE,
    BLACK
}
