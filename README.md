# Schackmotor - Läxa 1

## `chess-engine`

```
chessengine
|   Cargo.toml
│
└───src
    │   board.rs ────── pub struct Board
    |                  | - new() -> Board
    |                  | - custom(custom_board: Vec<Option<ChessPiece>>) -> Board
    |                  | - get_active_color() -> Color
    |                  | - get_active_pieces() -> Vec<Option<ChessPiece>>
    |                  | - get_promotion_setting(color: Color) -> ChessRole
    |                  | - set_promotion_setting(color: Color, role: ChessRole)
    |                  | - get_possible_moves(piece_position: ChessSquare) -> Option<Vec<ChessSquare>>
    |                  | - move_piece_to(current_position: ChessSquare, target_position: ChessSquare) -> Result<Option<ChessPiece>, String>
    |
    │   color.rs ────── enum Color
    |
    |   comp.rs ─────── pub struct ChessPiece
    |                  | - new(color: Color, role: ChessRole) -> ChessPiece
    |                  | - get_color() -> Color
    |                  | - get_role() -> ChessRole
    |                  |
    |                   pub struct ChessSquare
    |                  | - new(file: u8, rank: u8) -> ChessSquare
    |                  | - get_file() -> u8
    |                  | - get_rank() -> u8
    |                  |
    |                   pub enum ChessRole
    |
    |   direction.rs ── enum Direction
    |
    |   lib.rs
    |
    |   piece.rs ────── struct Piece
    |                  | - new(color: Color, role: Role, file: u8, rank: u8) -> Piece
    |                  | - get_color() -> Color
    |                  | - get_role() -> Role
    |                  | - get_file() -> u8
    |                  | - get_rank() -> u8
    |                  | - has_moved() -> bool
    |                  | - get_possible_moves(board: [Square; 64]) -> Vec<Square>
    |                  | - move_to(board: [Square; 64], target_file: u8, target_rank: u8) -> (Result<Option<Piece>, String>, [Square; 64])
    |
    |   role.rs ─────── enum Role
    |                  | - is_position_checked(board: [Square; 64], current_file: u8, current_rank: u8, active_color: Color, has_moved: bool) -> bool
    |                  | - get_possible_moves(board: [Square; 64], current_file: u8, current_rank: u8, active_color: Color, has_moved: bool) -> Vec<Square>
    |                  | - move_to(mut board: [Square; 64], target_file: u8, target_rank: u8, piece: Piece, has_moved: bool) -> (Result<Option<Piece>, String>, [Square; 64])
    |
    |   square.rs ───── struct Square
                       | - new(file: u8, rank: u8, piece: Option<Piece>) -> Square
                       | - get_file() -> u8
                       | - get_rank() -> u8
                       | - get_piece() -> Option<Piece>
                       | - set_piece(piece: Piece) -> Option<Piece>
                       | - is_en_passent_enabled() -> bool
                       | - enable_en_passent(enable: bool)
                       | - next(direction: &Direction, board: [Square; 64]) -> Option<Square>
```