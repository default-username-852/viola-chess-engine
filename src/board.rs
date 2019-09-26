use std::fmt;
use std::collections::HashMap;
use std::string::String;

use crate::color::Color;
use crate::piece::Piece;
use crate::role::Role;
use crate::square::Square;

pub use crate::comp::ChessPiece;
pub use crate::comp::ChessRole;
pub use crate::comp::ChessSquare;

pub struct Board {
    board: Vec<Square>,

    active_color: Color,

    promotions: HashMap<Color, Role>,
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Square> = Vec::new();

        // INIT BOARD AND PIECES
        for rank in 0..8 {
            for file in 0..8 {
                if rank == 0 || rank == 1 || rank == 6 || rank == 7 {
                    let color: Color;
                    match rank {
                        0 | 1 => color = Color::White,
                        _ => color = Color::Black,
                    }

                    let role: Role;
                    if rank == 1 || rank == 6 {
                        role = Role::Pawn;
                    } else {
                        match file {
                            4 => role = Role::King,
                            3 => role = Role::Queen,
                            2 | 5 => role = Role::Bichop,
                            1 | 6 => role = Role::Knight,
                            _ => role = Role::Rook,
                        }
                    }

                    board.push(Square::new(file as u8, rank as u8, Some(Piece::new(color, role, file as u8, rank as u8))));
                } else {
                    board.push(Square::new(file as u8, rank as u8, None));
                }
            }
        }

        let mut promotions = HashMap::new();
        promotions.insert(Color::White, Role::Queen);
        promotions.insert(Color::Black, Role::Queen);

        Board {
            board: board,
            active_color: Color::White,
            promotions: promotions
        }
    }

    pub fn custom(custom_board: Vec<Option<ChessPiece>>) -> Board {
        if custom_board.len() != 64 {
            panic!("Parameter board must be of length 64.");
        }

        let mut board: Vec<Square> = Vec::new();

        // INIT BOARD AND PIECES
        for rank in 0..8 {
            for file in 0..8 {
                let piece: Option<Piece>;
                match custom_board[rank * 8 + file] {
                    Some(_piece) => piece = Some(Piece::new(_piece.get_color(), Role::from(format!("{}", _piece.get_role())), file as u8, rank as u8)),
                    None => piece = None
                }

                board.push(Square::new(file as u8, rank as u8, piece));
            }
        }

        let mut promotions = HashMap::new();
        promotions.insert(Color::White, Role::Queen);
        promotions.insert(Color::Black, Role::Queen);

        Board {
            board: board,
            active_color: Color::White,
            promotions: promotions
        }
    }

    pub fn get_active_color(&self) -> Color {
        self.active_color
    }

    pub fn get_promotion_setting(&self, color: Color) -> ChessRole {
        ChessRole::from(format!("{}", *self.promotions.get(&color).unwrap()))
    }

    pub fn set_promotion_setting(&mut self, color: Color, role: ChessRole) {
        if role == ChessRole::King || role == ChessRole::Pawn {
            panic!("A pawn cannot get upgraded to a new pawn or a king!");
        }

        self.promotions.remove(&color);
        self.promotions.insert(color, Role::from(format!("{}", role)));
    }

    fn _get_active_pieces(&self) -> Vec<Option<Piece>> {
        self.board.iter().map(|&_square| _square.get_piece()).collect()
    }

    pub fn get_active_pieces(&self) -> Vec<Option<ChessPiece>> {
        self.board.iter().map(|&_square| {
                match _square.get_piece() {
                    Some(_piece) => Some(ChessPiece::new(_piece.get_color(), ChessRole::from(format!("{}", _piece.get_role())))),
                    None => None
                }
            }).collect()
    }

    fn get_active_piece(&self, piece: ChessSquare) -> Option<Piece> {
        let active_pieces = self._get_active_pieces();

        for i in 0..active_pieces.len() {
            let _active_piece = active_pieces[i];

            if _active_piece.is_some() {
                let active_piece = _active_piece.unwrap();

                if active_piece.equal(piece.get_file(), piece.get_rank()) {
                    return Some(active_piece);
                }
            }
        }

        println!("HEJ!");

        return None;
    }

    fn copy_board(&self) -> [Square; 64] {
        let mut board = [self.board[0]; 64];

        for i in 1..self.board.len() {
            board[i] = self.board[i];
        }

        board
    }

    pub fn get_possible_moves(&self, piece_position: ChessSquare) -> Option<Vec<ChessSquare>> {
        match self.get_active_piece(piece_position) {
            Some(_piece) => Some(_piece.get_possible_moves(self.copy_board()).iter().map(|&_square| ChessSquare::new(_square.get_file(), _square.get_rank())).collect()),
            None => None
        }
    }

    pub fn move_piece_to(&mut self, current_position: ChessSquare, target_position: ChessSquare) -> Result<Option<ChessPiece>, String> {

        let target_rank = target_position.get_rank();
        let target_file = target_position.get_file();

        let mut piece: Piece;
        match self.get_active_piece(current_position) {
            Some(_piece) => piece = _piece,
            None => return Err("No piece found on current position.".to_string()),
        }

        let result = piece.move_to(self.copy_board(), target_file, target_rank);

        match result.0 {
            Ok(_captured_piece) => {
                // APPLY BOARD CHANGES
                // TODO: test without application
                self.board.clear();
                for _square in result.1.iter() {
                    self.board.push(*_square);
                }

                // PROMOTION
                if piece.get_role() == Role::Pawn && (current_position.get_rank() == 7 || current_position.get_rank() == 0) {
                    self.board[(target_rank * 8 + target_file) as usize].set_piece(Some(Piece::new(self.active_color, *self.promotions.get(&self.active_color).unwrap(), target_file, target_rank)));
                }

                let active_pieces = self._get_active_pieces();

                // CHECK FOR CHECK MATE
                for i in 0..active_pieces.len() {
                    let _active_piece = active_pieces[i];

                    if _active_piece.is_some() {
                        let active_piece = _active_piece.unwrap();

                        if active_piece.get_color() != self.active_color && active_piece.get_role() == Role::King {
                            if self.get_possible_moves(ChessSquare::new(active_piece.get_file(), active_piece.get_rank())).unwrap().len() == 0 &&
                                active_piece.get_role().is_position_checked(self.copy_board(), active_piece.get_file(), active_piece.get_rank(), self.active_color, active_piece.has_moved()) {
                                return Ok(Some(ChessPiece::new(active_piece.get_color(), ChessRole::from(format!("{}", active_piece.get_role())))));
                            }

                            break;
                        }
                    }
                }

                // TOGGLE ACTIVE COLOR
                match self.active_color {
                    Color::White => self.active_color = Color::Black,
                    _ => self.active_color = Color::White,
                }

                match _captured_piece {
                    Some(_piece) => Ok(Some(ChessPiece::new(_piece.get_color(), ChessRole::from(format!("{}", _piece.get_role()))))),
                    None => Ok(None)
                }
            },
            Err(_err) => Err(_err)
        }
    }

    fn _format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut active_pieces = "[ ".to_string();

        let board_active_pieces = self.get_active_pieces();

        for i in 0..board_active_pieces.len() {
            active_pieces.push_str(format!("{:?}, ", board_active_pieces[i]).as_str());
        }

        active_pieces.push_str(" ]");

        write!(f, "Board [ board: [Square; 64], active_pieces: {}, active_color: {}, promotions: [Color::White -> {}, Color::Black -> {}] ]", active_pieces, self.active_color, self.promotions.get(&Color::White).unwrap(), self.promotions.get(&Color::Black).unwrap())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._format(f)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._format(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::color::Color;
    use crate::comp::ChessRole;
    use crate::comp::ChessSquare;
    use crate::comp::ChessPiece;

    #[test]
    fn r#const() {
        let board = Board::new();

        let active_pieces = board.get_active_pieces();

        assert_eq!(board.get_active_color(), Color::White);
        assert_eq!(active_pieces.len(), 64);
        assert_eq!(active_pieces[0].unwrap(), ChessPiece::new(Color::White, ChessRole::Rook));
        assert_eq!(board.get_promotion_setting(Color::White), ChessRole::Queen);
        assert_eq!(board.get_promotion_setting(Color::Black), ChessRole::Queen);
    }

    #[test]
    fn sets() {
        let mut board = Board::new();

        board.set_promotion_setting(Color::White, ChessRole::Bichop);

        assert_eq!(board.get_promotion_setting(Color::White), ChessRole::Bichop);
        assert_eq!(board.get_promotion_setting(Color::Black), ChessRole::Queen);
    }

    #[test]
    #[should_panic(expected = "A pawn cannot get upgraded to a new pawn or a king!")]
    fn sets_promotion_err_king() {
        let mut board = Board::new();

        board.set_promotion_setting(Color::White, ChessRole::King);
    }

    #[test]
    #[should_panic(expected = "A pawn cannot get upgraded to a new pawn or a king!")]
    fn sets_promotion_err_pawn() {
        let mut board = Board::new();

        board.set_promotion_setting(Color::White, ChessRole::Pawn);
    }

    #[test]
    #[ignore]
    fn get_possible_moves() {
        let board = Board::new();

        let result = board.get_possible_moves(ChessSquare::new(0, 1)).unwrap();
        let expected = vec![ChessSquare::new(0, 3), ChessSquare::new(0, 2)];

        for i in 0..result.len() {
            assert_eq!(result[i], expected[i])
        }
    }

    #[test]
    #[ignore]
    fn move_piece_to() {
        let mut board = Board::new();

        let result = board.move_piece_to(ChessSquare::new(0, 1), ChessSquare::new(0, 2));

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
