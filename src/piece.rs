use std::fmt;

use crate::role::Role;
use crate::color::Color;
use crate::square::Square;

#[derive(Copy, Clone)]
pub struct Piece {
    color: Color,
    role: Role,

    file: u8,
    rank: u8,

    moved: bool
}

impl Piece {
    pub fn new(color: Color, role: Role, file: u8, rank: u8) -> Piece {
        Piece {
            color: color,
            role: role,
            file: file,
            rank: rank,
            moved: false
        }
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }

    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    pub fn get_role(&self) -> Role {
        self.role
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn has_moved(&self) -> bool {
        self.moved
    }

    pub fn get_possible_moves(&self, board: [Square; 64]) -> Vec<Square> {
        self.role.get_possible_moves(board, self.file, self.rank, self.color, self.moved)
    }

    pub fn move_to(&mut self, board: [Square; 64], target_file: u8, target_rank: u8) -> (Result<Option<Piece>, String>, [Square; 64]) {
        let result = self.role.move_to(board, target_file, target_rank, *self);

        match result.0 {
            Ok(_captured_piece) => {
                self.file = target_file;
                self.rank = target_rank;

                self.moved = true;

                (Ok(_captured_piece), result.1)
            },
            Err(err) => (Err(err), result.1)
        }
    }

    pub fn equal(&self, file: u8, rank: u8) -> bool {
        self.file == file && self.rank == rank
    }

    fn _format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece [ color: {}, role: {}, file: {}, rank: {}, moved: {} ]", self.color, self.role, self.file, self.rank, self.moved)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other.get_file(), other.get_rank())
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._format(f)
    }
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._format(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::Piece;
    use crate::color::Color;
    use crate::role::Role;

    #[test]
    fn r#const() {
        let piece = Piece::new(Color::White, Role::King, 0, 0);

        assert_eq!(piece.get_file(), 0);
        assert_eq!(piece.get_rank(), 0);
        assert_eq!(piece.get_role(), Role::King);
        assert_eq!(piece.get_color(), Color::White);
        assert_eq!(piece.has_moved(), false);
    }

    #[test]
    fn eq() {
        let piece_1 = Piece::new(Color::White, Role::King, 0, 0);
        let piece_2 = Piece::new(Color::Black, Role::Queen, 1, 0);

        assert!(piece_1.equal(piece_1.get_file(), piece_1.get_rank()));
        assert!(!piece_1.equal(piece_2.get_file(), piece_2.get_rank()));

        assert!(piece_1 == piece_1);
        assert!(piece_1 != piece_2);
    }
}
