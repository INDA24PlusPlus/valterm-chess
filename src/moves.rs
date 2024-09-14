use crate::{Color, Game, Piece, PieceType, Position};

type Moves = Vec<Position>;

pub fn check_bounds(position: Position) -> bool {
    if position.x > 7 || position.y > 7 || position.x < 0 || position.y < 0 {
        return false;
    }
    true
}

fn push_if_valid_bounds(moves: &mut Moves, position: Position) {
    if check_bounds(position) {
        moves.push(position);
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Invalid,
    Regular,
    Attack,
}

fn get_move_type(game: &Game, piece: Piece, position: Position) -> MoveType {
    if !check_bounds(position) {
        return MoveType::Invalid;
    }

    match game.color_at(position) {
        Some(color) => {
            if color != piece.color {
                return MoveType::Attack;
            }
        }
        None => return MoveType::Regular,
    }

    MoveType::Invalid
}

fn push_if_valid_attack(moves: &mut Moves, game: &Game, piece: Piece, delta: Position) {
    let new_position = piece.position + delta;
    match get_move_type(game, piece, new_position) {
        MoveType::Invalid => (),
        _ => moves.push(new_position),
    }
}

/* fn filter_if_checkable(game: &Game, piece: Piece, position: Move) {

} */

pub fn get_piece_moves(game: &Game, piece: Piece) -> Moves {
    match piece.piece_type {
        PieceType::Pawn => get_pawn_moves(game, piece),
        PieceType::Knight => get_knight_moves(game, piece),
        PieceType::Bishop => get_bishop_moves(game, piece),
        PieceType::Rook => get_rook_moves(game, piece),
        PieceType::Queen => get_queen_moves(game, piece),
        _ => panic!("Unimplemented!"),
    }
}

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
    if get_move_type(game, piece, forward) == MoveType::Regular {
        moves.push(forward);
        forward_valid = true;
    }

    // Sideways capture
    if get_move_type(game, piece, left) == MoveType::Attack {
        moves.push(left);
    }
    if get_move_type(game, piece, right) == MoveType::Attack {
        moves.push(right);
    }

    let initial_move = (piece.position.y == 1 && piece.color == Color::White)
        || (piece.position.y == 6 && piece.color == Color::Black);

    // Double step forward
    if initial_move && forward_valid {
        match piece.color {
            Color::White => push_if_valid_bounds(&mut moves, piece.position + (0, 2)),
            Color::Black => push_if_valid_bounds(&mut moves, piece.position - (0, 2)),
        }
    }

    // TODO: En passant

    moves
}

pub fn get_knight_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    push_if_valid_attack(&mut moves, game, piece, (2, 1).into());
    push_if_valid_attack(&mut moves, game, piece, (2, -1).into());
    push_if_valid_attack(&mut moves, game, piece, (-2, 1).into());
    push_if_valid_attack(&mut moves, game, piece, (-2, -1).into());
    push_if_valid_attack(&mut moves, game, piece, (1, 2).into());
    push_if_valid_attack(&mut moves, game, piece, (1, -2).into());
    push_if_valid_attack(&mut moves, game, piece, (-1, 2).into());
    push_if_valid_attack(&mut moves, game, piece, (-1, -2).into());

    moves
}

pub fn get_moves_direction(game: &Game, piece: Piece, direction: Position) -> Moves {
    let mut moves: Moves = vec![];

    let mut pos = piece.position + direction;
    while check_bounds(pos) {
        match get_move_type(game, piece, pos) {
            MoveType::Attack => {
                moves.push(pos);
                break;
            }
            MoveType::Invalid => break,
            MoveType::Regular => (),
        }

        moves.push(pos);
        pos = pos + direction;
    }

    moves
}

pub fn get_bishop_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    moves.append(&mut get_moves_direction(game, piece, (1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (1, -1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, -1).into()));

    moves
}

pub fn get_rook_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    moves.append(&mut get_moves_direction(game, piece, (1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, -1).into()));

    // TODO: Castling

    moves
}

pub fn get_queen_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    moves.append(&mut get_moves_direction(game, piece, (1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, -1).into()));

    moves.append(&mut get_moves_direction(game, piece, (1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (1, -1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, -1).into()));

    moves
}
