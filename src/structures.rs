use crate::enums:: {PieceType, PieceColor};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Square{
    pub col: u8, 
    pub row: u8,
}

impl Square {
    pub fn new_alg(col: char, row: u8) -> Square {
        
        // if row < 1 || row > 8 || col < 1 || col > 8 {
        //     Err("Square's coordinates are out of bound.")
        // }else {
        //     Ok(Square{
        //         row, 
        //         col
        //     })
        // }
        match col {
            'a' => Square {col:1, row},
            'b' => Square {col:2, row},
            'c' => Square {col:3, row},
            'd' => Square {col:4, row},
            'e' => Square {col:5, row},
            'f' => Square {col:6, row},
            'g' => Square {col:7, row},
            'h' => Square {col:8, row},
            _ => Square {col:0, row:0}
        }
        
    }

    pub fn new(col: u8, row: u8) -> Square {
        Square {col, row}
    }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Piece{
    pub ptype: PieceType,
    pub pcolor: PieceColor,
}

impl Piece{

    pub fn new(ptype: PieceType, pcolor: PieceColor) -> Piece {
        Piece {
            ptype,
            pcolor
        }
    }

}

#[derive(PartialEq, Eq, Clone)]
pub struct Move {
    pub start: Square,
    pub end: Square
}

impl Move {
    pub fn new(start: Square, end: Square) -> Move {
        Move{start, end}
    }
}

#[derive(Clone)]
pub struct MoveHistory {
    pub moves: Vec<Move>
}

impl MoveHistory{
    pub fn new() -> MoveHistory {
        let mut _moves = Vec::<Move>::new();

        MoveHistory{
            moves:_moves
        }
    }
}

