use std::fmt;
use std::mem;

use crate::color::Color;

/* 
 * Public Declarations:
 * - struct ChessSquare
 * - struct ChessPiece
 * - enum ChessRole
 */

#[derive(Copy, Clone)]
pub struct ChessSquare {
    file: u8,
    rank: u8
}

impl ChessSquare {
    pub fn new(file: u8, rank: u8) -> ChessSquare {
        if file >= 8 || rank >= 8 {
            panic!("Parameters must have values less than 8.");
        }

        ChessSquare {
            file: file,
            rank: rank
        }
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }

    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    fn _format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChessSquare [ file: {}, rank: {} ]", self.file, self.rank)
    }
}

impl PartialEq for ChessSquare {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.get_file() && self.rank == other.get_rank()
    }
}

impl fmt::Display for ChessSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._format(f)
    }
}
impl fmt::Debug for ChessSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._format(f)
    }
}

#[cfg(test)]
mod chess_square_tests {
    use crate::comp::ChessSquare;

    #[test]
    fn r#const() {
        let square = ChessSquare::new(0, 0);

        assert_eq!(square.get_file(), 0);
        assert_eq!(square.get_rank(), 0);
    }

    #[test]
    #[should_panic(expected = "Parameters must have values less than 8.")]
    fn const_err_file() {
        let piece = ChessSquare::new(8, 0);
    }

    #[test]
    #[should_panic(expected = "Parameters must have values less than 8.")]
    fn const_err_rank() {
        let piece = ChessSquare::new(0, 8);
    }
}

// ---

#[derive(Copy, Clone)]
pub struct ChessPiece {
    color: Color,
    role: ChessRole
}

impl ChessPiece {
    pub fn new(color: Color, role: ChessRole) -> ChessPiece {
        ChessPiece {
            color: color,
            role: role
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_role(&self) -> ChessRole {
        self.role
    }

    fn _format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChessPiece [ color: {}, role: {} ]", self.color, self.role)
    }
}

impl PartialEq for ChessPiece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.get_color() && self.role == other.get_role()
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._format(f)
    }
}
impl fmt::Debug for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._format(f)
    }
}

#[cfg(test)]
mod chess_piece_tests {
    use crate::comp::ChessPiece;
    use crate::comp::ChessRole;
    use crate::color::Color;

    #[test]
    fn r#const() {
        let piece = ChessPiece::new(Color::White, ChessRole::King);

        assert_eq!(piece.get_color(), Color::White);
        assert_eq!(piece.get_role(), ChessRole::King);
    }
}

// ---

#[derive(Copy, Clone, Debug)]
pub enum ChessRole {
    King,
    Queen,
    Bichop,
    Knight,
    Rook,
    Pawn
}

impl PartialEq for ChessRole {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl fmt::Display for ChessRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for ChessRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "King" => ChessRole::King,
            "Queen" => ChessRole::Queen,
            "Bichop" => ChessRole::Bichop,
            "Knight" => ChessRole::Knight,
            "Rook" => ChessRole::Rook,
            "Pawn" | _ => ChessRole::Pawn,
        }
    }
}