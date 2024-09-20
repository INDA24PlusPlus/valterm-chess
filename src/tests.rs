#[cfg(test)]
mod chess_tests {
    use crate::*;

    fn elements_eq<T: PartialEq>(left: Vec<T>, right: Vec<T>) -> bool {
        return left == right
            || (left.iter().all(|elem| right.contains(elem))
                && right.iter().all(|elem| left.contains(elem)));
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
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 0, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 0, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
                position: Position { x: 0, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
                position: Position { x: 1, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 1, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 1, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
                position: Position { x: 1, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
                position: Position { x: 2, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 2, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 2, y: 4 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
                position: Position { x: 2, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Queen,
                position: Position { x: 3, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 3, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 3, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Queen,
                position: Position { x: 3, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::King,
                position: Position { x: 4, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 4, y: 3 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 4, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::King,
                position: Position { x: 4, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
                position: Position { x: 5, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 5, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
                position: Position { x: 5, y: 2 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 5, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
                position: Position { x: 5, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 6, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 6, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
                position: Position { x: 6, y: 7 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Rook,
                position: Position { x: 7, y: 0 },
                num_moves: 0,
            },
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position { x: 7, y: 1 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
                position: Position { x: 7, y: 6 },
                num_moves: 0,
            },
            Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
                position: Position { x: 7, y: 7 },
                num_moves: 0,
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
    fn bishop_moves() {
        let mut game = Game::new();
        game.load_fen("7k/8/8/3p4/8/1B6/2P5/7K");

        assert!(elements_eq(
            game.get_valid_moves(game.pieces[1][2].unwrap()),
            [(0, 1).into(), (0, 3).into(), (2, 3).into(), (3, 4).into()].into()
        ));
    }

    #[test]
    fn rook_moves() {
        let mut game = Game::new();
        game.load_fen("7k/8/1p6/8/8/1R6/1P6/7K");

        assert!(elements_eq(
            game.get_valid_moves(game.pieces[1][2].unwrap()),
            [
                (0, 2).into(),
                (1, 3).into(),
                (1, 4).into(),
                (1, 5).into(),
                (2, 2).into(),
                (3, 2).into(),
                (4, 2).into(),
                (5, 2).into(),
                (6, 2).into(),
                (7, 2).into(),
            ]
            .into()
        ));
        //println!("{:?}", game.get_valid_moves(game.pieces[1][2].unwrap()));
    }

    #[test]
    fn queen_moves() {
        let mut game = Game::new();
        game.load_fen("7k/1p6/8/3P4/8/1Qr5/8/7K");

        assert!(elements_eq(
            game.get_valid_moves(game.pieces[1][2].unwrap()),
            [
                (0, 2).into(),
                (0, 3).into(),
                (1, 3).into(),
                (1, 4).into(),
                (1, 5).into(),
                (1, 6).into(),
                (2, 2).into(),
                (2, 3).into(),
                (2, 1).into(),
                (3, 0).into(),
                (1, 1).into(),
                (1, 0).into(),
                (0, 1).into(),
            ]
            .into()
        ));
    }

    #[test]
    fn king_moves() {
        let mut game = Game::new();
        game.load_fen("1r6/8/4k3/8/2K5/2P5/8/8");

        assert!(elements_eq(
            game.get_valid_moves(game.pieces[2][3].unwrap()),
            [(2, 4).into(), (3, 3).into(), (3, 2).into()].into()
        ));
        //println!("{:?}", game.get_valid_moves(game.pieces[2][3].unwrap()));
    }

    #[test]
    fn check() {
        let mut game = Game::new();
        game.load_fen("rnb1kbnr/pp1ppppp/8/q1p5/4P3/3P4/PPP2PPP/RNBQKBNR");

        assert_eq!(game.is_check(), Some(Color::White));

        game.load_fen("rnb1kbnr/pp1ppppp/8/q1p5/4P3/8/PPPP1PPP/RNBQKBNR");

        assert_eq!(game.is_check(), None);
    }

    #[test]
    fn checkmate() {
        let mut game = Game::new();
        game.load_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR");

        assert_eq!(game.is_checkmate(), Some(Color::White));

        game.load_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/4PP2/PPPP3P/RNBQKBNR");

        assert_eq!(game.is_check(), Some(Color::White));
        assert_eq!(game.is_checkmate(), None);

        game.load_fen("8/8/8/5K1k/8/8/8/7R");

        // This fails because current_move is white and it thinks that white AND black is checkmated so it prioritises white
        // Easy fix by setting current_move before which will always happen in a real game, however this bug may have
        // other consequences which are unknown at this time
        game.current_move = Color::Black;
        assert_eq!(game.is_checkmate(), Some(Color::Black));
    }

    #[test]
    fn stalemate() {
        let mut game = Game::new();
        game.load_fen("5k2/5P2/5K2/8/8/8/8/8");
        game.current_move = Color::Black;
        assert!(game.is_stalemate());
        game.current_move = Color::White;
        assert!(!game.is_stalemate());

        game.load_fen("8/8/8/8/8/8/4p1pp/4Kbrk");
        game.current_move = Color::Black;
        assert!(game.is_stalemate());
        game.current_move = Color::White;
        assert!(!game.is_stalemate());

        game.load_fen("k7/P7/K7/8/5B2/8/8/8");
        game.current_move = Color::Black;
        assert!(game.is_stalemate());
    }

    #[test]
    fn promotion() {
        let mut game = Game::new();
        game.load_fen("7k/2P5/8/8/8/8/8/7K");
        assert_eq!(game.update_game(), GameStatus::Active);
        game.move_piece((2, 6).into(), (2, 7).into());
        assert_eq!(
            game.update_game(),
            GameStatus::Promotion(game.pieces[2][7].unwrap())
        );
        assert_eq!(game.current_move, Color::Black);
    }

    #[test]
    fn en_passant() {
        let mut game = Game::new();
        game.load_fen("rnbqkbnr/pppp1ppp/8/8/4p3/8/PPPPPPPP/RNBQKBNR");

        game.move_piece((3, 1).into(), (3, 2).into());
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[4][3].unwrap()),
            [(4, 2).into(), (3, 2).into()].into()
        ));
        game.en_passant_possible = None;
        //println!("{:?}", game.get_valid_moves(game.pieces[4][3].unwrap()));

        game.load_fen("rnbqkbnr/ppp1pppp/3p4/3P4/8/8/PPP1PPPP/RNBQKBNR");
        game.current_move = Color::Black;
        game.move_piece((4, 6).into(), (4, 4).into());
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[3][4].unwrap()),
            [(4, 5).into()].into()
        ));
        game.move_piece((3, 4).into(), (4, 5).into());
        assert_eq!(game.en_passant_possible, None);
    }

    #[test]
    fn castling() {
        let mut game = Game::new();
        game.load_fen("2k5/8/8/3r4/8/8/8/R3K2R");
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[4][0].unwrap()),
            [(4, 1).into(), (5, 1).into(), (5, 0).into(), (6, 0).into()].into()
        ));
        game.move_piece((4, 0).into(), (6, 0).into());
        assert!(game.pieces[5][0].is_some_and(|piece| piece.piece_type == PieceType::Rook));

        game.load_fen("r3k2r/8/8/8/8/8/5R1R/4K3");
        game.current_move = Color::Black;
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[4][7].unwrap()),
            [(4, 6).into(), (3, 7).into(), (3, 6).into(), (2, 7).into()].into()
        ));
        game.move_piece((4, 7).into(), (2, 7).into());
        assert!(game.pieces[3][7].is_some_and(|piece| piece.piece_type == PieceType::Rook));

        /* This test case currently overflows the stack because the king cannot castle out of
        check and so the check check invokes get_pseudo_moves for the opposing king which
        creates infinite recursion if it can castle */
        // Fixed, hope this fix has no other unwanted implications
        game.load_fen("r3k2r/8/8/8/8/8/4R2R/4K3");
        game.current_move = Color::Black;
        println!("{:?}", game.get_valid_moves(game.pieces[4][7].unwrap()));
        assert!(elements_eq(
            game.get_valid_moves(game.pieces[4][7].unwrap()),
            [(5, 7).into(), (3, 7).into(), (3, 6).into(), (5, 6).into()].into()
        ));
    }
}
