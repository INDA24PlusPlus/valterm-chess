use std::{ops, str::FromStr};

use moves::{check_bounds, get_move_type, get_pseudo_moves, MoveType, Moves};

pub mod moves;
pub mod tests;

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Default)]
pub struct Game {
    pub pieces: Board,
    pub current_move: Color,
    status: GameStatus,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GameStatus {
    #[default]
    Active,
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Promotion(Piece),
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[None; 8]; 8],
            current_move: Color::White,
            status: GameStatus::Active,
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

    /// Returns the color of the currently checked player, None if no player is checked
    /// Will panic if any king is missing, please dont call if there is no king :(
    pub fn is_check(&self) -> Option<Color> {
        let pieces = self.get_pieces();
        let white_king = *pieces
            .iter()
            .find(|piece| piece.color == Color::White && piece.piece_type == PieceType::King)
            .unwrap();

        let black_king = *pieces
            .iter()
            .find(|piece| piece.color == Color::Black && piece.piece_type == PieceType::King)
            .unwrap();

        for piece in pieces {
            if piece.color == Color::White
                && get_pseudo_moves(self, piece)
                    .iter()
                    .any(|position| *position == black_king.position)
            {
                return Some(Color::Black);
            }

            if piece.color == Color::Black
                && get_pseudo_moves(self, piece)
                    .iter()
                    .any(|position| *position == white_king.position)
            {
                return Some(Color::White);
            }
        }

        None
    }

    pub fn move_piece(&mut self, piece: Piece, to: Position) -> MoveType {
        if piece.color != self.current_move {
            return MoveType::Invalid;
        }

        let possible_moves = self.get_valid_moves(piece);

        if !possible_moves.contains(&to) {
            return MoveType::Invalid;
        }

        let move_type = get_move_type(self, piece, to);

        self.force_move(piece.position, to);

        if piece.piece_type == PieceType::Pawn
            && ((to.y == 0 && piece.color == Color::Black)
                || (to.y == 7 && piece.color == Color::White))
        {
            // Pawn promotion
            self.status = GameStatus::Promotion(self.pieces[to.x as usize][to.y as usize].unwrap());
        }

        self.current_move = !self.current_move;

        move_type
    }

    /// Checks whether or not a move puts own player in check
    /// Kinda janky maybe rewrite?
    fn self_check(&mut self, piece: Piece, position: Position) -> bool {
        let old = self.pieces[position.x as usize][position.y as usize];
        self.force_move(piece.position, position); // Move piece to new position to see if checked

        if self.is_check().is_some_and(|color| color == piece.color) {
            self.force_move(position, piece.position); // Reset position
            self.pieces[position.x as usize][position.y as usize] = old;
            return true;
        }

        self.force_move(position, piece.position); // Reset position
        self.pieces[position.x as usize][position.y as usize] = old;
        false
    }

    /// Performs full validation of valid moves, including blocking checked moves etc
    pub fn get_valid_moves(&mut self, piece: Piece) -> Moves {
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
    fn escape_check(&mut self, piece: Piece, position: Position) -> bool {
        let old = self.pieces[position.x as usize][position.y as usize];
        self.force_move(piece.position, position); // Move piece to new position to see if checked

        if self.is_check().is_none() || self.is_check().is_some_and(|color| color != piece.color) {
            self.force_move(position, piece.position); // Reset position
            self.pieces[position.x as usize][position.y as usize] = old;
            return true;
        }

        self.force_move(position, piece.position); // Reset position
        self.pieces[position.x as usize][position.y as usize] = old;
        false
    }

    pub fn is_checkmate(&mut self) -> Option<Color> {
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

    pub fn is_stalemate(&mut self) -> bool {
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

        self.status = GameStatus::Active;
        self.status
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    position: Position,
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
