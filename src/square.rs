use std::fmt;

use crate::direction::Direction;
use crate::piece::Piece;

#[derive(Copy, Clone)]
pub struct Square {
    piece: Option<Piece>,

    file: u8,
    rank: u8,

    en_passent_enabled: bool
}

impl Square {
    pub fn new(file: u8, rank: u8, piece: Option<Piece>) -> Square {
        Square {
            piece: piece,
            file: file, 
            rank: rank,
            en_passent_enabled: false
        }
    }

    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }

    pub fn set_piece(&mut self, piece: Option<Piece>) -> Option<Piece> {
        let captured_piece = self.piece;

        self.piece = piece;

        captured_piece
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }

    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    pub fn is_en_passent_enabled(&self) -> bool {
        self.en_passent_enabled
    }

    pub fn enable_en_passent(&mut self, enable: bool) {
        self.en_passent_enabled = enable;
    }

    pub fn next(&self, direction: &Direction, board: [Square; 64]) -> Option<Square> {
        let mut file: i8 = self.file as i8;
        let mut rank: i8 = self.rank as i8;

        if direction == &Direction::DownLeft || direction == &Direction::Down || direction == &Direction::DownRight {
            rank -= 1;
        } else if direction == &Direction::UpLeft || direction == &Direction::Up || direction == &Direction::UpRight {
            rank += 1;
        }

        if direction == &Direction::DownLeft || direction == &Direction::Left || direction == &Direction::UpLeft {
            file -= 1;
        } else if direction == &Direction::DownRight || direction == &Direction::Right || direction == &Direction::UpRight {
            file += 1;
        }

        let index = (rank * 8 + file) as usize;

        if file < 0 || rank < 0 || index >= 64 {
            None
        } else {
            Some(board[index])
        }
    }

    fn _format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: String;
        match self.piece {
            Some(_piece) => formatted = format!("Some({})", _piece),
            _ => formatted = "None".to_string()
        }

        write!(f, "Square [ piece: {}, file: {}, rank: {}, is_en_passent_enabled: {} ]", formatted, self.file, self.rank, self.en_passent_enabled)
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.get_file() == other.get_file() && self.get_rank() == other.get_rank()
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._format(f)
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._format(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::square::Square;
    use crate::piece::Piece;
    use crate::color::Color;
    use crate::role::Role;

    #[test]
    fn r#const() {
        let square = Square::new(0, 0, None);

        assert_eq!(square.get_file(), 0);
        assert_eq!(square.get_rank(), 0);
        assert_eq!(square.get_piece(), None);
        assert_eq!(square.is_en_passent_enabled(), false);
    }

    #[test]
    fn eq() {
        let square_1 = Square::new(0, 0, None);
        let square_2 = Square::new(1, 0, None);

        assert!(square_1 == square_1);
        assert!(square_1 != square_2);
    }

    #[test]
    fn sets() {
        let mut square = Square::new(0, 0, None);
        let piece = Some(Piece::new(Color::White, Role::King, 0, 0));

        let captured_piece = square.set_piece(piece);
        square.enable_en_passent(true);

        assert_eq!(square.get_piece(), piece);
        assert_eq!(captured_piece, None);
        assert_eq!(square.is_en_passent_enabled(), true);
    }
}