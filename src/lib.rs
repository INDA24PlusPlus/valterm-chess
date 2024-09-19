use std::{ops, str::FromStr};

use moves::{check_bounds, get_move_type, get_pseudo_moves, MoveType, Moves};

pub mod moves;
pub mod tests;

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Default, Clone, Copy)]
pub struct Game {
    pub pieces: Board,
    pub current_move: Color,
    status: GameStatus,
    en_passant_possible: Option<Piece>,
    moves_since_capture: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GameStatus {
    #[default]
    Active,
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Promotion(Piece),
    FiftyMoveRule,
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[None; 8]; 8],
            current_move: Color::White,
            status: GameStatus::Active,
            en_passant_possible: None,
            moves_since_capture: 0,
        }
    }

    pub fn default_board(&mut self) {
        self.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    pub fn clear_board(&mut self) {
        for x in 0..8 {
            for y in 0..8 {
                self.pieces[x][y] = None;
            }
        }
    }

    // TODO: More checking of FEN string to assure valid board
    pub fn load_fen(&mut self, fen: &str) {
        let fields = fen.split(" ").collect::<Vec<&str>>();

        // Parse piece placements
        let mut rows = fields[0].split("/").collect::<Vec<&str>>();
        rows.reverse();

        self.clear_board();

        let mut y = 0;
        for row in rows {
            let mut x = 0;
            for c in row.chars() {
                let mut piece = Piece {
                    color: Color::White,
                    piece_type: PieceType::Pawn,
                    position: Position { x, y },
                    num_moves: 0,
                };

                if c.is_ascii_lowercase() {
                    // Lowercase letter
                    piece.color = Color::Black;
                }

                if c.is_ascii_digit() {
                    // Digit
                    x += c.to_digit(10).unwrap() as i8; // Should never fail so why not unwrap :)
                    continue;
                }

                match c.to_ascii_lowercase() {
                    'p' => piece.piece_type = PieceType::Pawn,
                    'n' => piece.piece_type = PieceType::Knight,
                    'b' => piece.piece_type = PieceType::Bishop,
                    'r' => piece.piece_type = PieceType::Rook,
                    'q' => piece.piece_type = PieceType::Queen,
                    'k' => piece.piece_type = PieceType::King,
                    _ => panic!("Bruh"),
                }

                self.pieces[x as usize][y as usize] = Some(piece);

                x += 1;
            }
            y += 1
        }
    }

    pub fn print_board(&self) {
        for y in (0..8).rev() {
            for x in 0..8 {
                let mut c = match self.pieces[x][y] {
                    None => '.',
                    Some(piece) => match piece.piece_type {
                        PieceType::King => 'K',
                        PieceType::Queen => 'Q',
                        PieceType::Bishop => 'B',
                        PieceType::Knight => 'N',
                        PieceType::Rook => 'R',
                        PieceType::Pawn => 'P',
                    },
                };

                if self.pieces[x][y].is_some_and(|piece| piece.color == Color::Black) {
                    c = c.to_ascii_lowercase();
                }

                print!("{} ", c);
            }
            println!();
        }
    }

    fn force_move(&mut self, from: Position, to: Position) {
        if !check_bounds(from) || !check_bounds(to) {
            return;
        }

        let mut piece = match self.pieces[from.x as usize][from.y as usize] {
            Some(piece) => piece,
            None => return,
        };

        piece.position = to;
        self.pieces[from.x as usize][from.y as usize] = None;
        self.pieces[to.x as usize][to.y as usize] = Some(piece);
    }

    pub fn get_pieces(&self) -> Vec<Piece> {
        self.pieces
            .iter()
            .flatten()
            .filter_map(|piece| *piece)
            .collect::<Vec<Piece>>()
    }

    pub fn color_at(&self, position: Position) -> Option<Color> {
        self.pieces[position.x as usize][position.y as usize].map(|piece| piece.color)
    }

    pub fn is_color_checked(&self, color: Color) -> bool {
        let pieces = self.get_pieces();
        let king = *pieces
            .iter()
            .find(|piece| piece.color == color && piece.piece_type == PieceType::King)
            .unwrap();

        for piece in pieces {
            if piece.color != color
                && get_pseudo_moves(self, piece)
                    .iter()
                    .any(|position| *position == king.position)
            {
                return true;
            }
        }

        false
    }

    /// Returns the color of the currently checked player, None if no player is checked
    /// Will panic if any king is missing, please dont call if there is no king :(
    pub fn is_check(&self) -> Option<Color> {
        let white_checked = self.is_color_checked(Color::White);
        let black_checked = self.is_color_checked(Color::Black);

        if white_checked && black_checked {
            // Weird edge case
            return Some(self.current_move);
        } else if white_checked {
            return Some(Color::White);
        } else if black_checked {
            return Some(Color::Black);
        }

        None
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> MoveType {
        let piece = match self.pieces[from.x as usize][from.y as usize] {
            Some(p) => p,
            None => return MoveType::Invalid,
        };

        if piece.color != self.current_move {
            return MoveType::Invalid;
        }

        let possible_moves = self.get_valid_moves(piece);

        if !possible_moves.contains(&to) {
            return MoveType::Invalid;
        }
        let move_type = get_move_type(self, piece, to);

        // Reset en passant thingy
        let saved_en_passant = self.en_passant_possible; // please end my suffering
        self.en_passant_possible = None;

        self.force_move(piece.position, to);

        // Increase move counter
        self.pieces[to.x as usize][to.y as usize]
            .as_mut()
            .unwrap()
            .num_moves += 1;

        // Copy updated piece for later use
        let new_piece = self.pieces[to.x as usize][to.y as usize].unwrap();

        if piece.piece_type == PieceType::Pawn
            && ((to.y == 0 && piece.color == Color::Black)
                || (to.y == 7 && piece.color == Color::White))
        {
            // Pawn promotion
            self.status = GameStatus::Promotion(new_piece);
        }

        // Pawn moved two steps, en passant is now possible
        if piece.piece_type == PieceType::Pawn && (piece.position - to).y.abs() == 2 {
            self.en_passant_possible = Some(new_piece);
        }
        // Capture if en passant happened or something

        // Capture if en passant happened or something idk at this point
        if move_type == MoveType::EnPassant {
            let passed = saved_en_passant.unwrap(); // Should not fail :sunglasses:
            self.pieces[passed.position.x as usize][passed.position.y as usize] = None;
            // goodbye bozo
        }

        // 50 move rule
        if get_move_type(self, piece, to) == MoveType::Attack {
            self.moves_since_capture = 0;
        } else {
            self.moves_since_capture += 1;
        }

        // Castling
        if move_type == MoveType::Castling {
            // Find rook who's nuts just got played with
            // x is the target position for the rook
            let (rook, x) = match new_piece.position.x {
                2 => (self.pieces[0][new_piece.position.y as usize].unwrap(), 3),
                6 => (self.pieces[7][new_piece.position.y as usize].unwrap(), 5),
                _ => panic!("Castling has gone very wrong :("),
            };

            // Move it
            self.force_move(rook.position, (x, piece.position.y).into());
        }

        self.current_move = !self.current_move;

        move_type
    }

    /// Checks whether or not a move puts own player in check
    // Creates a copy of the board instead of mutating it (to avoid shit going horribly wrong)
    fn self_check(&self, piece: Piece, position: Position) -> bool {
        let mut copy_game = *self;
        copy_game.force_move(piece.position, position); // Move piece to new position to see if checked

        if copy_game.is_color_checked(piece.color) {
            return true;
        }

        false
    }

    /// Performs full validation of valid moves, including blocking checked moves etc
    pub fn get_valid_moves(&self, piece: Piece) -> Moves {
        let moves = get_pseudo_moves(self, piece);

        // Filter out moves that puts own player in check
        let valid_moves: Vec<Position> = moves
            .iter()
            .filter(|mov| !self.self_check(piece, **mov))
            .cloned()
            .collect();

        valid_moves
    }

    /// Checks if a certain moves the player out of a checked position
    fn escape_check(&self, piece: Piece, position: Position) -> bool {
        let mut copy_game = *self;
        copy_game.force_move(piece.position, position); // Move piece to new position to see if checked

        if copy_game.is_check().is_none()
            || copy_game
                .is_check()
                .is_some_and(|color| color != piece.color)
        {
            return true;
        }

        false
    }

    pub fn is_checkmate(&self) -> Option<Color> {
        // Color that is checked
        let checked = match self.is_check() {
            Some(color) => color,
            None => return None,
        };

        // All pieces of that color
        let pieces: Vec<Piece> = self
            .get_pieces()
            .iter()
            .filter(|piece| piece.color == checked)
            .cloned()
            .collect();

        for piece in pieces {
            let moves = self.get_valid_moves(piece);
            for mov in moves {
                if self.escape_check(piece, mov) {
                    return None;
                }
            }
        }

        Some(checked)
    }

    pub fn is_stalemate(&self) -> bool {
        if self.is_check().is_some() {
            return false;
        }

        let pieces: Vec<Piece> = self
            .get_pieces()
            .iter()
            .filter(|piece| piece.color == self.current_move)
            .cloned()
            .collect();

        for piece in pieces {
            if !self.get_valid_moves(piece).is_empty() {
                return false;
            }
        }

        true
    }

    pub fn promote(&mut self, target: PieceType) {
        let to_upgrade = match self.status {
            GameStatus::Promotion(piece) => piece,
            _ => panic!("Error!"),
        };

        self.pieces[to_upgrade.position.x as usize][to_upgrade.position.y as usize]
            .as_mut()
            .unwrap()
            .piece_type = target;

        self.status = GameStatus::Active;
    }

    // Updates internal game status and returns it, to be called after each move by a player
    pub fn update_game(&mut self) -> GameStatus {
        if let GameStatus::Promotion(_) = self.status {
            return self.status;
        }

        if let Some(color) = self.is_checkmate() {
            self.status = GameStatus::Checkmate(color);
            return self.status;
        };

        if let Some(color) = self.is_check() {
            self.status = GameStatus::Check(color);
            return self.status;
        };

        if self.is_stalemate() {
            self.status = GameStatus::Stalemate;
            return self.status;
        }

        if self.moves_since_capture == 50 {
            self.status = GameStatus::FiftyMoveRule;
            return self.status;
        }

        self.status = GameStatus::Active;
        self.status
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    position: Position,
    num_moves: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePositionError;

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        let x = chars[0].to_ascii_lowercase() as u8 - b'a';
        let y = chars[1].to_digit(10).unwrap_or(0) - 1;

        Ok((x as i8, y as i8).into())
    }
}

use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = ((self.x as u8 + b'A') as char).to_string();
        let y = (self.y + 1).to_string();

        write!(f, "{}{}", x, y)
    }
}

impl From<(i8, i8)> for Position {
    fn from(pos: (i8, i8)) -> Position {
        Position { x: pos.0, y: pos.1 }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add<(i8, i8)> for Position {
    type Output = Position;

    fn add(self, rhs: (i8, i8)) -> Position {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl ops::Sub<(i8, i8)> for Position {
    type Output = Position;

    fn sub(self, rhs: (i8, i8)) -> Position {
        Position {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
