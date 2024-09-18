#[cfg(test)]
mod chess_tests {
    use crate::*;

    fn elements_eq<T: PartialEq>(left: Vec<T>, right: Vec<T>) -> bool {
        return left == right || left.iter().all(|elem| right.contains(elem));
    }

    #[test]
    fn import_fen() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ");

        let pieces = [
            Piece {
                color: Color::White,
                piece_type: PieceType::Rook,
                position: Position { x: 0, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 0, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 0, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
                position: Position { x: 0, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
                position: Position { x: 1, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 1, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 1, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
                position: Position { x: 1, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
                position: Position { x: 2, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 2, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 2, y: 4 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
                position: Position { x: 2, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Queen,
                position: Position { x: 3, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 3, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 3, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Queen,
                position: Position { x: 3, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::King,
                position: Position { x: 4, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 4, y: 3 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 4, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::King,
                position: Position { x: 4, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
                position: Position { x: 5, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 5, y: 1 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
                position: Position { x: 5, y: 2 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 5, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
                position: Position { x: 5, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 6, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 6, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
                position: Position { x: 6, y: 7 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Rook,
                position: Position { x: 7, y: 0 },
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 7, y: 1 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 7, y: 6 },
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
                position: Position { x: 7, y: 7 },
            },
        ];

        assert!(elements_eq(game.get_pieces(), pieces.into()))
    }

    #[test]
    fn pawn_moves() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pp1ppppp/8/8/4P3/2p2N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[1][1].unwrap()),
            [(1, 2).into(), (1, 3).into(), (2, 2).into()].into()
        ));
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[2][2].unwrap()),
            [(1, 1).into(), (3, 1).into()].into()
        ));
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[2][1].unwrap()),
            [].into()
        ));
    }

    #[test]
    fn knight_moves() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pp1ppppp/8/8/4P3/2p2N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");

        assert!(elements_eq(
            game.get_valid_moves(game.pieces[1][0].unwrap()),
            [(0, 2).into(), (2, 2).into()].into()
        ));
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[5][2].unwrap()),
            [
                (6, 0).into(),
                (7, 3).into(),
                (6, 4).into(),
                (4, 4).into(),
                (3, 3).into()
            ]
            .into()
        ));
    }

    #[test]
    fn check() {
        let mut game = Game::new();
        game.load_fen("rnb1kbnr/pp1ppppp/8/q1p5/4P3/3P4/PPP2PPP/RNBQKBNR");

        assert_eq!(game.is_check(), Some(Color::White));

        game.load_fen("rnb1kbnr/pp1ppppp/8/q1p5/4P3/8/PPPP1PPP/RNBQKBNR");

        assert_eq!(game.is_check(), None);
    }
}
