use crate::{Color, Game, Piece, Position};

type Moves = Vec<Position>;

pub fn check_bounds(position: Position) -> bool {
    if position.x > 8 || position.y > 8 {
        return false;
    }
    true
}

fn push_if_valid(moves: &mut Moves, position: Position) {
    if check_bounds(position) {
        moves.push(position);
    }
}

// Returns all possible moves for a certain piece, does not perform extensive checking
pub fn get_pawn_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    let initial_move = (piece.position.y == 1 && piece.color == Color::White)
        || (piece.position.y == 6 && piece.color == Color::Black);

    match piece.color {
        Color::White => push_if_valid(&mut moves, piece.position + (0, 1)),
        Color::Black => push_if_valid(&mut moves, piece.position - (0, 1)),
    }

    if initial_move {
        match piece.color {
            Color::White => push_if_valid(&mut moves, piece.position + (0, 2)),
            Color::Black => push_if_valid(&mut moves, piece.position - (0, 2)),
        }
    }

    if piece.color == Color::White {
        let left = piece.position + (1, 1);
        let right = piece.position + (0, 1) - (1, 0);
        if check_bounds(left) && game.color_at(left).is_some() {
            moves.push(left);
        }
        if check_bounds(right) && game.color_at(right).is_some() {
            moves.push(right);
        }
    }

    moves
}
