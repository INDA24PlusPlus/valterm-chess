use crate::{Color, Game, Piece, PieceType, Position};

type Moves = Vec<Position>;

pub fn check_bounds(position: Position) -> bool {
    if position.x > 7 || position.y > 7 || position.x < 0 || position.y < 0 {
        return false;
    }
    true
}

fn push_if_valid(moves: &mut Moves, position: Position) {
    if check_bounds(position) {
        moves.push(position);
    }
}

fn push_if_valid_check_collision(moves: &mut Moves, piece: Piece, delta: Position) {
    todo!()
}

pub fn get_piece_moves(game: &Game, piece: Piece) -> Moves {
    match piece.piece_type {
        PieceType::Pawn => get_pawn_moves(game, piece),
        PieceType::Knight => get_knight_moves(game, piece),
        _ => panic!("Unimplemented!"),
    }
}

// Returns all possible moves for a certain piece, does not perform extensive checking (collisions with own pieces etc)
pub fn get_pawn_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    let mut left = piece.position;
    let mut right = piece.position;
    let mut forward = piece.position;
    if piece.color == Color::White {
        left = left + (1, 1);
        right = right + (-1, 1);
        forward = forward + (0, 1);
    } else {
        left = left - (1, 1);
        right = right - (-1, 1);
        forward = forward - (0, 1);
    }

    let mut forward_valid = false;

    // Single step forward
    if check_bounds(forward) && game.color_at(forward).is_none() {
        moves.push(forward);
        forward_valid = true;
    }

    // Sideways capture
    if check_bounds(left)
        && game
            .color_at(left)
            .is_some_and(|color| color != piece.color)
    {
        moves.push(left);
    }
    if check_bounds(right)
        && game
            .color_at(right)
            .is_some_and(|color| color != piece.color)
    {
        moves.push(right);
    }

    let initial_move = (piece.position.y == 1 && piece.color == Color::White)
        || (piece.position.y == 6 && piece.color == Color::Black);

    // Double step forward
    if initial_move && forward_valid {
        match piece.color {
            Color::White => push_if_valid(&mut moves, piece.position + (0, 2)),
            Color::Black => push_if_valid(&mut moves, piece.position - (0, 2)),
        }
    }

    // TODO: En passant

    moves
}

pub fn get_knight_moves(_game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    push_if_valid(&mut moves, piece.position + (2, 1));
    push_if_valid(&mut moves, piece.position + (2, -1));
    push_if_valid(&mut moves, piece.position + (-2, 1));
    push_if_valid(&mut moves, piece.position + (-2, -1));
    push_if_valid(&mut moves, piece.position + (1, 2));
    push_if_valid(&mut moves, piece.position + (1, -2));
    push_if_valid(&mut moves, piece.position + (-1, 2));
    push_if_valid(&mut moves, piece.position + (-1, -2));

    moves
}
