/*
Uncertain if 2D-array of Piece objects is the best way to go about representing the board...
Perhaps switch to some other system?
*/

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
                    x: x,
                    y: y,
                };

                if c >= 'a' && c <= 'z' {
                    // Lowercase letter
                    piece.color = Color::Black;
                }

                if c >= '0' && c <= '9' {
                    // Digit
                    x += c.to_digit(10).unwrap() as u8; // Should never fail so why not unwrap :)
                    continue;
                }

                match c {
                    'p' | 'P' => piece.piece_type = PieceType::Pawn,
                    'n' | 'N' => piece.piece_type = PieceType::Knight,
                    'b' | 'B' => piece.piece_type = PieceType::Bishop,
                    'r' | 'R' => piece.piece_type = PieceType::Rook,
                    'q' | 'Q' => piece.piece_type = PieceType::Queen,
                    'k' | 'K' => piece.piece_type = PieceType::King,
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
                    // .to_lowercase() returns an iterator. Really..?
                    c = c.to_lowercase().collect::<Vec<char>>()[0];
                }

                print!("{} ", c);
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        game.print_board();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
