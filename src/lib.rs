use std::ops;

/*
Uncertain if 2D-array of Piece objects is the best way to go about representing the board...
Perhaps switch to some other system?
*/
pub mod moves;

pub type Board = [[Option<Piece>; 8]; 8];

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

                if c.is_digit(10) {
                    // Digit
                    x += c.to_digit(10).unwrap() as u8; // Should never fail so why not unwrap :)
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
                    None => '_',
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
        match self.pieces[position.x as usize][position.y as usize] {
            Some(piece) => Some(piece.color),
            None => None,
        }
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
    x: u8,
    y: u8,
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

impl ops::Add<(u8, u8)> for Position {
    type Output = Position;

    fn add(self, rhs: (u8, u8)) -> Position {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl ops::Sub<(u8, u8)> for Position {
    type Output = Position;

    fn sub(self, rhs: (u8, u8)) -> Position {
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

#[cfg(test)]
mod tests {
    use moves::get_pawn_moves;

    use super::*;

    fn verify_board(game: &Game) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let piece = match game.pieces[x][y] {
                    Some(piece) => piece,
                    None => continue,
                };

                if piece.position
                    != (Position {
                        x: x as u8,
                        y: y as u8,
                    })
                {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        game.force_move(Position { x: (1), y: (6) }, Position { x: (1), y: (2) });
        game.print_board();
        println!("{:?}", get_pawn_moves(&game, game.pieces[0][1].unwrap()));
        assert!(tests::verify_board(&game))
    }
}
