use std::fmt;
use std::mem;

use crate::color::Color;
use crate::direction::Direction;
use crate::square::Square;
use crate::piece::Piece;

enum Move {
    Single,
    Multi
}

fn get_possible_standard_moves(board: [Square; 64], origin_file: u8, origin_rank: u8, active_color: Color, paths: Vec<Vec<Direction>>, move_type: Move) -> Vec<Square> {
    let mut possible_moves: Vec<Square> = Vec::new();

    let origin_square = board[(origin_rank * 8 + origin_file) as usize];

    for directions in paths {
        let mut current_square = origin_square;

        loop {
            let mut next_square: Option<Square> = None;

            for direction in &directions {
                if next_square.is_some() {
                    next_square = next_square.unwrap().next(direction, board);
                } else {
                    next_square = current_square.next(direction, board);
                }
            }

            if next_square.is_some() {
                current_square = next_square.unwrap();
            } else {
                break;
            }

            let occuping_piece = current_square.get_piece();

            if occuping_piece.is_none() || (occuping_piece.is_some() && active_color != occuping_piece.unwrap().get_color()) {
                possible_moves.push(current_square);
            }

            if occuping_piece.is_some() {
                break;
            }

            let _break: bool;
            match move_type {
                Move::Single => _break = true,
                _ => _break = false,
            }

            if _break {
                break;
            }
        }
    }

    possible_moves
}

#[derive(Copy, Clone, Eq, Hash, Debug)]
pub enum Role {
    King,
    Queen,
    Bichop,
    Knight,
    Rook,
    Pawn
}

impl Role {
    pub fn is_position_checked(&self, board: [Square; 64], current_file: u8, current_rank: u8, active_color: Color, has_moved: bool) -> bool {
        for i in 0..board.len() {
            let square = board[i];
            let _piece = square.get_piece();

            if _piece.is_some() {
                let piece = _piece.unwrap();

                if piece.get_color() != active_color {
                    if piece.get_role() != Role::King {
                        for square in piece.get_role()._get_possible_moves(board, piece.get_file(), piece.get_rank(), piece.get_color(), has_moved) {
                            if square.get_file() == current_file && square.get_rank() == current_rank {
                                return true;
                            }
                        }
                    } else {
                        for direction in vec![
                            Direction::UpLeft,
                            Direction::Up,
                            Direction::UpRight,
                            Direction::Right,
                            Direction::DownRight,
                            Direction::Down,
                            Direction::DownLeft,
                            Direction::Left] {
                            let next_square = square.next(&direction, board);

                            if next_square.is_some() && next_square.unwrap() == square {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        return false;
    }

    fn _get_possible_moves(&self, board: [Square; 64], current_file: u8, current_rank: u8, active_color: Color, has_moved: bool) -> Vec<Square> {

        let all_single_directions: Vec<Vec<Direction>> = vec![
            vec![Direction::Up],
            vec![Direction::UpLeft],
            vec![Direction::UpRight],
            vec![Direction::Down],
            vec![Direction::DownLeft],
            vec![Direction::DownRight],
            vec![Direction::Left],
            vec![Direction::Right]
        ];

        match self {
            Role::King => {
                let mut possible_moves: Vec<Square> = Vec::new();

                let in_check = self.is_position_checked(board, current_file, current_rank, active_color, has_moved);

                let mut standard_moves = get_possible_standard_moves(board, current_file, current_rank, active_color, all_single_directions, Move::Single);
                possible_moves.append(&mut standard_moves);

                // CASTLING
                if !in_check && !has_moved {
                    let check_color_rank: u8;
                    match active_color {
                        Color::White => check_color_rank = 0,
                        _ => check_color_rank = 7
                    }

                    if current_rank == check_color_rank {
                        // QUEENSIDE
                        let mut castling_allowed = true;

                        for _file in 1..current_file {
                            if board[(current_rank * 8 + _file) as usize].get_piece().is_some() {
                                castling_allowed = false;
                            }
                        }

                        let mut involved_rook = board[(current_rank * 8 + 0) as usize].get_piece();

                        if involved_rook.is_none() || (involved_rook.is_some() && involved_rook.unwrap().has_moved()) {
                            castling_allowed = false;
                        }

                        if castling_allowed {
                            possible_moves.push(board[(current_rank * 8 + 2) as usize])
                        }

                        // KINGSIDE
                        castling_allowed = true;

                        for _file in (current_file + 1)..8 {
                            if board[(current_rank * 8 + _file) as usize].get_piece().is_some() {
                                castling_allowed = false;
                            }
                        }

                        involved_rook = board[(current_rank * 8 + 7) as usize].get_piece();

                        if involved_rook.is_none() || (involved_rook.is_some() && involved_rook.unwrap().has_moved()) {
                            castling_allowed = false;
                        }

                        if castling_allowed {
                            possible_moves.push(board[(current_rank * 8 + 2) as usize])
                        }
                    }
                }

                possible_moves
            },

            Role::Queen => get_possible_standard_moves(board, current_file, current_rank, active_color, all_single_directions, Move::Multi),

            Role::Bichop => {
                get_possible_standard_moves(board, current_file, current_rank, active_color, vec![
                    vec![Direction::UpLeft],
                    vec![Direction::UpRight],
                    vec![Direction::DownLeft],
                    vec![Direction::DownRight]
                    ], Move::Multi)
            },

            Role::Knight => {
                get_possible_standard_moves(board, current_file, current_rank, active_color, vec![
                    vec![Direction::UpLeft, Direction::Up],
                    vec![Direction::UpRight, Direction::Up],
                    vec![Direction::DownLeft, Direction::Down],
                    vec![Direction::DownRight, Direction::Down],
                    vec![Direction::Left, Direction::UpLeft],
                    vec![Direction::Left, Direction::DownLeft],
                    vec![Direction::Right, Direction::UpRight],
                    vec![Direction::Right, Direction::DownRight]
                ], Move::Single)
            },

            Role::Rook => {
                get_possible_standard_moves(board, current_file, current_rank, active_color, vec![
                    vec![Direction::Up],
                    vec![Direction::Down],
                    vec![Direction::Left],
                    vec![Direction::Right]
                ], Move::Multi)
            },

            _ => {
                let mut possible_moves: Vec<Square> = Vec::new();

                let multiplier: i8;
                let catch_directions: Vec<Direction>;
                let en_passent_directions: Vec<Direction>;

                match active_color {
                    Color::White => {
                        multiplier = 1;
                        catch_directions = vec![Direction::UpLeft, Direction::UpRight];
                        en_passent_directions = vec![Direction::DownLeft, Direction::DownRight];
                    },
                    _ => {
                        multiplier = -1;
                        catch_directions = vec![Direction::DownLeft, Direction::DownRight];
                        en_passent_directions = vec![Direction::UpLeft, Direction::UpRight];
                    }
                }

                // 2 STEP INITIAL MOVE
                if current_rank == 1 || current_rank == 6 {
                    possible_moves.push(board[(((current_rank as i8) + (2 * multiplier)) * 8 + (current_file as i8)) as usize]);
                }

                let current_square = board[(current_rank * 8 + current_file) as usize];

                // CAPTURE MOVE
                for direction in catch_directions {
                    let _next_square = current_square.next(&direction, board);

                    if _next_square.is_some() {
                        let next_square = _next_square.unwrap();

                        if next_square.get_piece().is_some() {
                            possible_moves.push(board[(next_square.get_rank() * 8 + next_square.get_file()) as usize]);
                        }
                    }
                }

                // EN PASSENT
                if current_square.is_en_passent_enabled() {
                    for direction in en_passent_directions {
                        let _next_square = current_square.next(&direction, board);

                        if _next_square.is_some() {
                            let next_square = _next_square.unwrap();

                            if next_square.get_piece().is_some() {
                                possible_moves.push(board[(next_square.get_rank() * 8 + next_square.get_file()) as usize]);

                                break;
                            }
                        }
                    }
                }

                // SINGLE MOVE
                possible_moves.push(board[(((current_rank as i8) + multiplier) * 8 + (current_file as i8)) as usize]);

                possible_moves
            },
        }
    }

    pub fn get_possible_moves(&self, board: [Square; 64], current_file: u8, current_rank: u8, active_color: Color, has_moved: bool) -> Vec<Square> {
        let mut output_moves: Vec<Square> = Vec::new();

        let possible_moves = self._get_possible_moves(board, current_file, current_rank, active_color, has_moved);

        for i in 0..possible_moves.len() {
            let board_copy: [Square; 64] = board.clone();

            let move_test_file = possible_moves[i].get_file();
            let move_test_rank = possible_moves[i].get_rank();

            let mut castling_possible = true;

            // CASTLING
            if self == &Role::King && ((move_test_file as i8) - (current_file as i8)).abs() == 2 {
                if !(((move_test_file as i8) - (current_file as i8) < 0 && !self.is_position_checked(board_copy, move_test_file + 1, move_test_rank, active_color, has_moved)) ||
                    ((move_test_file as i8) - (current_file as i8) > 0 && !self.is_position_checked(board_copy, move_test_file - 1, move_test_rank, active_color, has_moved))) {
                    castling_possible = false;
                }
            }

            let mut move_test_board = [board[0]; 64];

            for j in 1..move_test_board.len() {
                move_test_board[j] = board[j];
            }

            move_test_board[(move_test_rank * 8 + move_test_file) as usize].set_piece(move_test_board[(current_rank * 8 + current_file) as usize].get_piece());
            move_test_board[(current_rank * 8 + current_file) as usize].set_piece(None);

            for j in 0..move_test_board.len() {
                let _piece = move_test_board[j].get_piece();

                if _piece.is_some() {
                    let piece = _piece.unwrap();

                    if piece.get_color() == active_color && piece.get_role() == Role::King {
                        if castling_possible && !self.is_position_checked(board_copy, piece.get_file(), piece.get_rank(), active_color, has_moved) {
                            output_moves.push(possible_moves[i]);
                        }

                        break;
                    }
                }
            }
        }

        output_moves
    }

    pub fn move_to(&self, mut board: [Square; 64], target_file: u8, target_rank: u8, piece: Piece) -> (Result<Option<Piece>, String>, [Square; 64]) {

        let current_file = piece.get_file();
        let current_rank = piece.get_rank();
        let active_color = piece.get_color();

        for possible_move in self.get_possible_moves(board, current_file, current_rank, active_color, piece.has_moved()) {
            if possible_move.get_file() == target_file && possible_move.get_rank() == target_rank {
                // CASTLING
                if self == &Role::King && ((target_file as i8) - (piece.get_file() as i8)).abs() == 2 {
                    let rook_file: u8;
                    let rook_target_file: u8;

                    if (target_file as i8) - (piece.get_file() as i8) < 0 {
                        rook_file = 0;
                        rook_target_file = 3;
                    } else {
                        rook_file = 7;
                        rook_target_file = 5;
                    }

                    if !board[(target_rank * 8 + rook_file) as usize].get_piece().unwrap().move_to(board, rook_target_file, target_rank).0.is_ok() {
                        return (Err("Failed to move rook in context of castling!".to_string()), board)
                    }
                }

                else if self == &Role::Pawn && (target_rank - 2 == current_rank || target_rank + 2 == current_rank) {
                    // SET EN PASSENT MARKERS
                    match active_color {
                        Color::White => {
                            board[((target_rank - 1) * 8 + current_file + 1) as usize].enable_en_passent(true);
                            board[((target_rank - 1) * 8 + current_file - 1) as usize].enable_en_passent(true);
                        },
                        Color::Black => {
                            board[((target_rank + 1) * 8 + current_file + 1) as usize].enable_en_passent(true);
                            board[((target_rank + 1) * 8 + current_file - 1) as usize].enable_en_passent(true);
                        }
                    }
                }

                // REMOVE OLD EN PASSENT MARKERS
                for _file in 0..8 {
                    match piece.get_color() {
                        Color::White => board[(6 * 8 + _file) as usize].enable_en_passent(false),
                        Color::Black => board[(3 * 8 + _file) as usize].enable_en_passent(false),
                    }
                }

                // REMOVE OLD PIECE REFERENCE
                board[(piece.get_rank() * 8 + piece.get_file()) as usize].set_piece(None);

                // SET NEW PIECE REFERENCE
                let captured_piece = board[(target_rank * 8 + target_file) as usize].set_piece(Some(piece));

                return (Ok(captured_piece), board);
            }
        }

        return (Err("Target square is not a posssible move for current role.".to_string()), board);
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "King" => Role::King,
            "Queen" => Role::Queen,
            "Bichop" => Role::Bichop,
            "Knight" => Role::Knight,
            "Rook" => Role::Rook,
            "Pawn" | _ => Role::Pawn,
        }
    }
}
