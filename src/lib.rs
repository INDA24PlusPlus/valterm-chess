use std::ops;

use moves::{check_bounds, get_piece_moves};

pub mod moves;
pub mod tests;

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Default)]
pub struct Game {
    pieces: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            pieces: [[None; 8]; 8],
        }
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
                && get_piece_moves(self, piece)
                    .iter()
                    .any(|position| *position == black_king.position)
            {
                return Some(Color::Black);
            }

            if piece.color == Color::Black
                && get_piece_moves(self, piece)
                    .iter()
                    .any(|position| *position == white_king.position)
            {
                return Some(Color::White);
            }
        }

        None
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
    x: i8,
    y: i8,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
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
