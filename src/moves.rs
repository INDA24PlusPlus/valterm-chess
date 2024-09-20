use crate::{Color, Game, Piece, PieceType, Position};

pub type Moves = Vec<Position>;

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
    EnPassant,
    Castling,
}

/// Does not check piece-specific movement requirements
pub fn get_move_type(game: &Game, piece: Piece, position: Position) -> MoveType {
    if !check_bounds(position) {
        return MoveType::Invalid;
    }

    // En passant
    if piece.piece_type == PieceType::Pawn
        && game
            .en_passant_possible
            .is_some_and(|piece2| piece2.color != piece.color)
    {
        let to_pass = game.en_passant_possible.unwrap();
        let mut behind = to_pass.position;
        if to_pass.color == Color::White {
            behind = behind - (0, 1);
        } else {
            behind = behind + (0, 1);
        }

        if position == behind && game.color_at(behind).is_none() {
            return MoveType::EnPassant;
        }
    }

    // Castling (king moved 2 squares <=> castling)
    if piece.piece_type == PieceType::King && (piece.position - position).x.abs() == 2 {
        return MoveType::Castling;
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

pub fn get_pseudo_moves(game: &Game, piece: Piece) -> Moves {
    match piece.piece_type {
        PieceType::Pawn => get_pawn_moves(game, piece),
        PieceType::Knight => get_knight_moves(game, piece),
        PieceType::Bishop => get_bishop_moves(game, piece),
        PieceType::Rook => get_rook_moves(game, piece),
        PieceType::Queen => get_queen_moves(game, piece),
        PieceType::King => get_king_moves(game, piece),
    }
}

fn get_pawn_moves(game: &Game, piece: Piece) -> Moves {
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

    // Sideways capture or en passant
    if get_move_type(game, piece, left) == MoveType::Attack
        || get_move_type(game, piece, left) == MoveType::EnPassant
    {
        moves.push(left);
    }

    if get_move_type(game, piece, right) == MoveType::Attack
        || get_move_type(game, piece, right) == MoveType::EnPassant
    {
        moves.push(right);
    }

    /* let initial_move = (piece.position.y == 1 && piece.color == Color::White)
    || (piece.position.y == 6 && piece.color == Color::Black); */
    let initial_move = piece.num_moves == 0;

    // Double step forward
    // FIX: Bug here where player can capture own pieces at double step forward
    // Update: FIXED!
    if initial_move && forward_valid {
        forward = match piece.color {
            Color::White => piece.position + (0, 2),
            Color::Black => piece.position - (0, 2),
        };

        if get_move_type(game, piece, forward) == MoveType::Regular {
            push_if_valid_bounds(&mut moves, forward);
        }
    }

    moves
}

fn get_knight_moves(game: &Game, piece: Piece) -> Moves {
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

fn get_moves_direction(game: &Game, piece: Piece, direction: Position) -> Moves {
    let mut moves: Moves = vec![];

    let mut pos = piece.position + direction;
    while check_bounds(pos) {
        match get_move_type(game, piece, pos) {
            MoveType::Attack => {
                moves.push(pos);
                break;
            }
            MoveType::Regular => (),
            _ => break,
        }

        moves.push(pos);
        pos = pos + direction;
    }

    moves
}

fn get_bishop_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    moves.append(&mut get_moves_direction(game, piece, (1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (1, -1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, -1).into()));

    moves
}

fn get_rook_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    moves.append(&mut get_moves_direction(game, piece, (1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, 1).into()));
    moves.append(&mut get_moves_direction(game, piece, (-1, 0).into()));
    moves.append(&mut get_moves_direction(game, piece, (0, -1).into()));

    moves
}

fn get_queen_moves(game: &Game, piece: Piece) -> Moves {
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

fn get_king_moves(game: &Game, piece: Piece) -> Moves {
    let mut moves: Moves = vec![];

    push_if_valid_attack(&mut moves, game, piece, (1, 0).into());
    push_if_valid_attack(&mut moves, game, piece, (0, 1).into());
    push_if_valid_attack(&mut moves, game, piece, (-1, 0).into());
    push_if_valid_attack(&mut moves, game, piece, (0, -1).into());

    push_if_valid_attack(&mut moves, game, piece, (1, 1).into());
    push_if_valid_attack(&mut moves, game, piece, (-1, 1).into());
    push_if_valid_attack(&mut moves, game, piece, (1, -1).into());
    push_if_valid_attack(&mut moves, game, piece, (-1, -1).into());

    // Time for castling :skull:
    // Makes the most sense to define castling as a kings move
    // If you dont agree im sorry but you're wrong
    // PS: dont read any of the code below it is absolutely horrendous

    let pieces = game.get_pieces();
    let king = pieces
        .iter()
        .find(|piece2| piece2.color == piece.color && piece2.num_moves == 0)
        .cloned();

    if king.is_none() {
        // King has moved or something else has gone terribly wrong
        return moves;
    }

    // Currently causes stack overflow :skull:
    // Check if king is checked without generating moves for opposing king
    // The opposing king cannot interfere with the castling anyway
    if game.is_color_checked_exclude_king(piece.color) {
        return moves;
    }

    let long_rook = pieces
        .iter()
        .find(|piece2| {
            piece2.piece_type == PieceType::Rook
                && piece2.num_moves == 0
                && piece2.position.x == 0
                && piece2.color == piece.color
        })
        .cloned();
    let short_rook = pieces
        .iter()
        .find(|piece2| {
            piece2.piece_type == PieceType::Rook
                && piece2.num_moves == 0
                && piece2.position.x == 7
                && piece2.color == piece.color
        })
        .cloned();

    // Short castling
    if let Some(rook) = short_rook {
        if game.color_at(rook.position + (-1, 0)).is_none()
            && game.color_at(rook.position + (-2, 0)).is_none()
        {
            // Squares are not occupied
            // Make sure the king does not "pass through check"

            if !game.self_check(piece, rook.position + (-1, 0))
                && !game.self_check(piece, rook.position + (-2, 0))
            {
                moves.push(rook.position + (-1, 0));
            }
        }
    }

    // Long castling
    if let Some(rook) = long_rook {
        if game.color_at(rook.position + (1, 0)).is_none()
            && game.color_at(rook.position + (2, 0)).is_none()
            && game.color_at(rook.position + (3, 0)).is_none()
        {
            // Squares are not occupied
            // Make sure the king does not "pass through check"

            // Here the rook.position + (1, 0) does not matter since the king never passes through it even though the rook does
            if !game.self_check(piece, rook.position + (2, 0))
                && !game.self_check(piece, rook.position + (3, 0))
            {
                moves.push(rook.position + (2, 0));
            }
        }
    }

    moves
}
