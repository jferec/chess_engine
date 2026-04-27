use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result;
use std::ops::RangeInclusive;
use std::vec;

pub static MIN_LINK: usize = 0;
pub static MAX_LINK: usize = 7;
pub static RANK_SIZE: usize = 8;
pub static TOP_RIGHT: usize = 9;
pub static TOP_LEFT: usize = 7;
pub static BOTTOM_LEFT: isize = -7;
pub static BOTTOM_RIGHT: isize = -9;
pub static WHITE_PAWN_START_RANK: usize = 1;
pub static BLACK_PAWN_START_RANK: usize = 6;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,

    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,

    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,

    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,

    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,

    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,

    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,

    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

impl Square {
    pub const ALL: [Square; 64] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ];

    pub fn from_index(index: usize) -> Self {
        assert!(index < 64);
        Self::ALL
            .get(index)
            .expect("Index must be in (0..64) range")
            .clone()
    }

    pub fn from_rank_file(rank: usize, file: usize) -> Self {
        assert!(rank < 8 && file < 8);
        return Self::from_index(8 * rank + file);
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn rank(self) -> usize {
        self.index() / RANK_SIZE
    }

    pub const fn file(self) -> usize {
        self.index() % RANK_SIZE
    }

    pub const fn is_first_file(self) -> bool {
        self.file() == 0
    }

    pub const fn is_last_file(self) -> bool {
        self.file() == 7
    }

    pub const fn is_lowest_rank(self) -> bool {
        self.rank() == 0
    }

    pub const fn is_highest_rank(self) -> bool {
        self.rank() == 7
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> Self {
        square.index() as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn enemy_color(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    fn pawn_start_rank(&self) -> usize {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    fn pawn_direction(&self) -> isize {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    fn promotion_rank(&self) -> usize {
        match self {
            Color::White => 6,
            Color::Black => 1,
        }
    }

    fn en_passant_target_rank(&self) -> usize {
        match self {
            Color::White => 3,
            Color::Black => 4,
        }
    }

    fn start_pawn_rank(&self) -> usize {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    fn to_fen(&self) -> &str {
        match self {
            Color::White => "w",
            Color::Black => "b",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    fn new(color: Color, kind: PieceKind) -> Piece {
        Piece {
            color: color,
            kind: kind,
        }
    }

    fn has_same_color(&self, color: Color) -> bool {
        self.color == color
    }

    fn has_other_color(&self, color: Color) -> bool {
        self.color != color
    }

    fn is_pawn(&self) -> bool {
        self.kind == PieceKind::Pawn
    }

    fn to_fen(&self) -> &str {
        match (self.color, self.kind) {
            (Color::White, PieceKind::Pawn) => "P",
            (Color::White, PieceKind::Bishop) => "B",
            (Color::White, PieceKind::Knight) => "N",
            (Color::White, PieceKind::Rook) => "R",
            (Color::White, PieceKind::Queen) => "Q",
            (Color::White, PieceKind::King) => "K",
            (Color::Black, PieceKind::Pawn) => "p",
            (Color::Black, PieceKind::Bishop) => "b",
            (Color::Black, PieceKind::Knight) => "n",
            (Color::Black, PieceKind::Rook) => "r",
            (Color::Black, PieceKind::Queen) => "q",
            (Color::Black, PieceKind::King) => "k",
        }
    }

    fn from_fen(fen: char) -> Self {
        match fen {
            'P' => Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            'B' => Piece {
                kind: PieceKind::Bishop,
                color: Color::White,
            },
            'N' => Piece {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            'R' => Piece {
                kind: PieceKind::Rook,
                color: Color::White,
            },
            'Q' => Piece {
                kind: PieceKind::Queen,
                color: Color::White,
            },
            'K' => Piece {
                kind: PieceKind::King,
                color: Color::White,
            },
            'p' => Piece {
                kind: PieceKind::Pawn,
                color: Color::Black,
            },
            'b' => Piece {
                kind: PieceKind::Bishop,
                color: Color::Black,
            },
            'n' => Piece {
                kind: PieceKind::Knight,
                color: Color::Black,
            },
            'r' => Piece {
                kind: PieceKind::Rook,
                color: Color::Black,
            },
            'q' => Piece {
                kind: PieceKind::Queen,
                color: Color::Black,
            },
            'k' => Piece {
                kind: PieceKind::King,
                color: Color::Black,
            },
            _ => {
                panic!("Invalid char {}", fen)
            }
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Piece: ({:?} {:?})", self.color, self.kind)
    }
}

pub struct Board {
    squares: [Option<Piece>; 64],
    moves: Vec<Move>,
}

trait Serializer {
    fn serialize_to_fen(&self) -> String;
}

impl Serializer for Board {
    fn serialize_to_fen(&self) -> String {
        let mut encoded: String = String::with_capacity(128);
        for r in (0..8).rev() {
            let mut empty_counter = 0;
            for f in 0..8 {
                let s = Square::from_rank_file(r, f);
                if self.is_empty(s) {
                    empty_counter += 1;
                } else {
                    if empty_counter > 0 {
                        encoded.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    encoded.push_str(
                        self.piece_at_square(Square::from_rank_file(r, f))
                            .expect("Piece must be present.")
                            .to_fen(),
                    );
                }
            }
            if empty_counter > 0 {
                encoded.push_str(&empty_counter.to_string());
            }
            encoded.push_str("/");
        }
        // Remove last slash
        encoded.pop();
        // TODO: finish
        encoded
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            squares: [None; 64],
            moves: vec![],
        };

        board.setup_back_rank(Color::White, 0);
        board.setup_pawns(Color::White, WHITE_PAWN_START_RANK);
        board.setup_pawns(Color::Black, BLACK_PAWN_START_RANK);
        board.setup_back_rank(Color::Black, 7);
        board
    }

    pub fn new_empty_board() -> Self {
        Self {
            squares: [None; 64],
            moves: vec![],
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new_empty_board();
        let mut i: usize = 0;
        for x in fen.chars() {
            if x == '/' || x == ' ' {
                continue;
            } else if x.is_alphabetic() {
                board.set_piece(i, Piece::from_fen(x));
                i += 1;
            } else if x.is_numeric() {
                let n = x.to_digit(10).expect("Must be a number") as usize;
                i += n;
            }
        }
        assert!(i == 64, "all squares allmust be covered");
        board
    }

    pub fn turn(&self) -> Color {
        if self.moves.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn find_king(&self, color: Color) -> Square {
        Square::from_index(
            self.squares
                .iter()
                .position(|&x| {
                    x == Some(Piece {
                        kind: PieceKind::King,
                        color: color,
                    })
                })
                .expect("King must be present on the board."),
        )
    }

    pub fn get_pieces(&self, color: Color) -> Vec<Square> {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, piece)| piece.as_ref().is_some_and(|piece| piece.color == color))
            .map(|(index, _)| Square::from_index(index))
            .collect()
    }

    pub fn piece_at(&self, file: usize, rank: usize) -> Option<Piece> {
        self.squares[self.get_index(rank, file)]
    }

    pub fn piece_at_square(&self, square: Square) -> Option<Piece> {
        self.squares[square.index()]
    }

    pub fn is_empty(&self, square: Square) -> bool {
        self.piece_at_square(square).is_none()
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.piece_at_square(square).is_some()
    }

    pub fn is_occupied_by_enemy(&self, square: Square, color: Color) -> bool {
        self.piece_at_square(square)
            .is_some_and(|f| f.color != color)
    }

    pub fn is_occupied_by_friend(&self, square: Square, color: Color) -> bool {
        self.piece_at_square(square)
            .is_some_and(|f| f.color == color)
    }

    pub fn remove_piece_at_square(&mut self, square: Square) -> Piece {
        self.squares[square.index()]
            .take()
            .expect("Piece must be present.")
    }

    pub fn set_piece_at_square(&mut self, square: Square, piece: Piece) {
        self.squares[square.index()] = Some(piece)
    }

    pub fn set_piece(&mut self, index: usize, piece: Piece) {
        assert!(index < 64);
        self.squares[index] = Some(piece);
    }

    fn setup_pawns(&mut self, color: Color, rank: usize) {
        for file in 0..8 {
            self.set_piece(
                self.get_index(rank, file),
                Piece {
                    kind: PieceKind::Pawn,
                    color,
                },
            );
        }
    }

    fn get_index(&self, rank: usize, file: usize) -> usize {
        assert!(file < RANK_SIZE && rank < RANK_SIZE);
        rank * 8 + file
    }

    fn setup_back_rank(&mut self, color: Color, rank: usize) {
        let pieces = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];
        for (file, kind) in pieces.into_iter().enumerate() {
            self.set_piece(self.get_index(rank, file), Piece { kind, color });
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CastleSide {
    QueenSide,
    KingSide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MoveKind {
    Quiet,
    Attack {
        captured: Piece,
    },
    EnPassant {
        captured_square_pawn: Square,
    },
    Promotion {
        promoted_to: PieceKind,
        captured: Option<Piece>,
    },
    Castling {
        side: CastleSide,
    },
}

// Debug: in chess notation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    move_kind: MoveKind,
}

impl Move {
    fn is_en_passantable(self) -> bool {
        self.piece.is_pawn()
            && self.move_kind == MoveKind::Quiet
            && self.from.file() == self.to.file()
            && self.piece.color.pawn_start_rank() == self.from.rank()
            && self.piece.color.en_passant_target_rank() == self.to.rank()
    }
}

#[test]
fn test_make_move() {
    let mut board = Board::new();
}

#[test]
fn test_board_initializes_pieces_correctly() {
    let board = Board::new();
    let white_back_rank = [
        PieceKind::Rook,
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Queen,
        PieceKind::King,
        PieceKind::Bishop,
        PieceKind::Knight,
        PieceKind::Rook,
    ];

    for (file, kind) in white_back_rank.into_iter().enumerate() {
        assert_eq!(
            board.piece_at(file, 0),
            Some(Piece::new(Color::White, kind)),
            "Expected white {:?} at file {}, rank 0",
            kind,
            file
        );
        assert_eq!(
            board.piece_at(file, 1),
            Some(Piece::new(Color::White, PieceKind::Pawn)),
            "Expected white pawn at file {}, rank 1",
            file
        );
        assert_eq!(
            board.piece_at(file, 6),
            Some(Piece::new(Color::Black, PieceKind::Pawn)),
            "Expected black pawn at file {}, rank 6",
            file
        );
        assert_eq!(
            board.piece_at(file, 7),
            Some(Piece::new(Color::Black, kind)),
            "Expected black {:?} at file {}, rank 7",
            kind,
            file
        );
    }

    for rank in 2..6 {
        for file in 0..8 {
            assert_eq!(
                board.piece_at(file, rank),
                None,
                "Expected empty square at file {}, rank {}",
                file,
                rank
            );
        }
    }
}

pub trait MoveMaker {
    fn make_move(&mut self, m: &Move);

    fn umake_move(&mut self);
}

impl MoveMaker for Board {
    fn make_move(&mut self, m: &Move) {
        match m.move_kind {
            MoveKind::Quiet | MoveKind::Attack { .. } => {
                let piece = self.remove_piece_at_square(m.from);
                self.set_piece_at_square(m.to, piece);
            }
            MoveKind::EnPassant {
                captured_square_pawn,
            } => {
                let _ = self.remove_piece_at_square(captured_square_pawn);
                let piece = self.remove_piece_at_square(m.from);
                self.set_piece_at_square(m.to, piece);
            }
            MoveKind::Promotion { promoted_to, .. } => {
                let mut piece = self.remove_piece_at_square(m.from);
                piece.kind = promoted_to;
                self.set_piece_at_square(m.to, piece);
            }
            MoveKind::Castling { side } => {
                let king = self.remove_piece_at_square(m.from);
                assert!(king.kind == PieceKind::King);
                let rook_square = match side {
                    CastleSide::KingSide => Square::from_index(m.from.index() + 3),
                    CastleSide::QueenSide => Square::from_index(m.from.index() - 4),
                };
                let rook = self.remove_piece_at_square(rook_square);
                let end_rook_square = match side {
                    CastleSide::QueenSide => Square::from_index(m.from.index() - 1),
                    CastleSide::KingSide => Square::from_index(m.from.index() + 1),
                };
                let end_king_square = match side {
                    CastleSide::QueenSide => Square::from_index(m.from.index() - 2),
                    CastleSide::KingSide => Square::from_index(m.from.index() + 2),
                };
                self.set_piece_at_square(end_rook_square, rook);
                self.set_piece_at_square(end_king_square, king);
            }
        }
        self.moves.push(*m);
    }

    fn umake_move(&mut self) {
        let m = self
            .moves
            .pop()
            .expect("Move must be present when calling unmake_move");
        match m.move_kind {
            MoveKind::Quiet => {
                let piece: Piece = self.remove_piece_at_square(m.to);
                self.set_piece_at_square(m.from, piece);
            }
            MoveKind::Attack { captured } => {
                let piece: Piece = self.remove_piece_at_square(m.to);
                self.set_piece_at_square(m.from, piece);
                self.set_piece_at_square(m.to, captured);
            }
            MoveKind::EnPassant {
                captured_square_pawn,
            } => {
                self.set_piece_at_square(
                    captured_square_pawn,
                    Piece {
                        kind: PieceKind::Pawn,
                        color: m.piece.color.enemy_color(),
                    },
                );
                let attacking_pawn = self.remove_piece_at_square(m.to);
                assert!(attacking_pawn.is_pawn());
                self.set_piece_at_square(m.from, attacking_pawn);
            }
            MoveKind::Promotion { captured, .. } => {
                let mut promoted_piece = self.remove_piece_at_square(m.to);
                promoted_piece.kind = PieceKind::Pawn;
                self.set_piece_at_square(m.from, promoted_piece);
                if let Some(captured_piece) = captured {
                    self.set_piece_at_square(m.to, captured_piece);
                }
            }
            MoveKind::Castling { side } => {
                let king_piece = self.remove_piece_at_square(m.to);
                assert!(king_piece.kind == PieceKind::King);
                let original_king = m.from;
                self.set_piece_at_square(original_king, king_piece);
                let rook_square = match side {
                    CastleSide::QueenSide => Square::from_index(original_king.index() - 1),
                    CastleSide::KingSide => Square::from_index(original_king.index() + 1),
                };
                let rook = self.remove_piece_at_square(rook_square);
                assert!(rook.kind == PieceKind::Rook);
                let original_rook_square = match side {
                    CastleSide::KingSide => Square::from_index(rook_square.index() + 2),
                    CastleSide::QueenSide => Square::from_index(rook_square.index() - 3),
                };
                self.set_piece_at_square(original_rook_square, rook);
            }
        }
    }
}

pub trait MoveGenerator {
    fn generate_moves(&self, turn: Color) -> Vec<Move>;
}

impl MoveGenerator for Board {
    fn generate_moves(&self, turn: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let squares = self.get_pieces(turn);
        for square in squares {
            let piece = self
                .piece_at_square(square)
                .expect("Piece must be present.");
            moves.extend(self.generate_moves_for_piece(&GenerateMovesRequest {
                piece: piece,
                square,
            }));
        }
        moves
    }
}

///
/// last_enemy_pawn_double_push - required for en passant resolution
///
struct GenerateMovesRequest {
    piece: Piece,
    square: Square,
}

impl Board {
    fn has_king_moved_yet(&self, color: Color) -> bool {
        self.moves.iter().any(|m| {
            m.piece
                == Piece {
                    kind: PieceKind::King,
                    color: color,
                }
        })
    }

    /// The was at least one rook move or queen-side castle
    fn has_left_rook_moved_yet(&self, color: Color) -> bool {
        self.moves.iter().any(|m| {
            (m.piece
                == Piece {
                    kind: PieceKind::Rook,
                    color: color,
                }
                && m.from.is_first_file())
                || (m.piece.color == color
                    && m.move_kind
                        == MoveKind::Castling {
                            side: CastleSide::QueenSide,
                        })
        })
    }

    fn has_right_rook_moved_yet(&self, color: Color) -> bool {
        self.moves.iter().any(|m| {
            (m.piece
                == Piece {
                    kind: PieceKind::Rook,
                    color: color,
                }
                && m.from.is_last_file())
                || (m.piece.color == color
                    && m.move_kind
                        == MoveKind::Castling {
                            side: CastleSide::KingSide,
                        })
        })
    }

    fn generate_moves_for_piece(&self, request: &GenerateMovesRequest) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        match request.piece.kind {
            PieceKind::Pawn => {
                self.generate_moves_for_pawn(&mut moves, request.square, request.piece.color)
            }
            PieceKind::Bishop => {
                self.generate_moves_for_bishop(&mut moves, request.square, request.piece.color)
            }
            PieceKind::Knight => {
                self.generate_moves_for_knight(&mut moves, request.square, request.piece.color)
            }
            PieceKind::Rook => {
                self.generate_moves_for_rook(&mut moves, request.square, request.piece.color);
            }
            PieceKind::Queen => {
                self.generate_moves_for_queen(&mut moves, request.square, request.piece.color);
            }
            PieceKind::King => {
                self.generate_moves_for_king(&mut moves, request.square, request.piece.color);
            }
        }
        moves
    }

    /// add check against King pin
    fn generate_moves_for_pawn(&self, moves: &mut Vec<Move>, square: Square, color: Color) {
        assert!(!square.is_highest_rank() && !square.is_lowest_rank());
        if color.promotion_rank() == square.rank() {
            self.generate_promotion_moves(moves, square, color);
        } else {
            self.generate_non_promotion_moves(moves, square, color);
        }
    }

    fn generate_non_promotion_moves(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        assert!(
            color == Color::White && from.rank() < 6 || color == Color::Black && from.rank() > 1
        );
        let piece = Piece {
            kind: PieceKind::Pawn,
            color,
        };
        // Single push
        let single_push_index =
            (from.index() as isize + color.pawn_direction() * RANK_SIZE as isize) as usize;
        let single_push = Square::from_index(single_push_index);
        let single_push_allowed = self.is_empty(single_push);
        if single_push_allowed {
            moves.push(Move {
                piece,
                from,
                to: single_push,
                move_kind: MoveKind::Quiet,
            })
        }
        // Double push
        (single_push_allowed && from.rank() == color.pawn_start_rank())
            .then(|| {
                let index = (from.index() as isize
                    + color.pawn_direction() * 2 * RANK_SIZE as isize)
                    as usize;
                Square::from_index(index)
            })
            .filter(|double_push| self.is_empty(*double_push))
            .inspect(|double_push| {
                moves.push(Move {
                    piece,
                    from,
                    to: *double_push,
                    move_kind: MoveKind::Quiet,
                })
            });
        // Left attack
        if !from.is_first_file() {
            let left_attack: Square = match color {
                Color::White => Square::from_index(from.index() + TOP_LEFT),
                Color::Black => Square::from_index(from.index() - TOP_RIGHT),
            };
            if self.is_occupied_by_enemy(left_attack, color) {
                moves.push(Move {
                    piece,
                    from,
                    to: left_attack,
                    move_kind: MoveKind::Attack {
                        captured: self
                            .piece_at_square(left_attack)
                            .expect("Attacked piece should be present."),
                    },
                })
            };
        }
        // Right attack
        if !from.is_last_file() {
            let right_attack: Square = match color {
                Color::White => Square::from_index(from.index() + TOP_RIGHT),
                Color::Black => Square::from_index(from.index() - TOP_LEFT),
            };
            if self.is_occupied_by_enemy(right_attack, color) {
                moves.push(Move {
                    piece,
                    from,
                    to: right_attack,
                    move_kind: MoveKind::Attack {
                        captured: self
                            .piece_at_square(right_attack)
                            .expect("Attacked piece should be present."),
                    },
                })
            };
        }
        // En passant: all conditions must be true
        // - the last enemy move was a double push of the pawn
        // - the current pawn is on the same level as the enemy pawn
        // - the space behind the enemy pawn is empty.
        self.moves.last().inspect(|last_enemy_move| {
            let last_move = *last_enemy_move;
            if !last_move.is_en_passantable() {
                return;
            }
            let en_passant_target = last_move.to;
            let landing_target = Square::from_index(
                (en_passant_target.index() as isize + (color.pawn_direction() * RANK_SIZE as isize))
                    as usize,
            );

            if (!from.is_first_file() && Square::from_index(from.index() - 1) == en_passant_target
                || !from.is_last_file()
                    && Square::from_index(from.index() + 1) == en_passant_target)
                && self.is_empty(landing_target)
            {
                moves.push(Move {
                    piece,
                    from,
                    to: landing_target,
                    move_kind: MoveKind::EnPassant {
                        captured_square_pawn: en_passant_target,
                    },
                });
            }
        });
    }

    fn generate_promotion_moves(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        assert!(
            color == Color::White && from.rank() == 6 || color == Color::Black && from.rank() == 1
        );
        let piece = Piece {
            kind: PieceKind::Pawn,
            color,
        };
        // Push promotion
        let single_push_index =
            (from.index() as isize + color.pawn_direction() * RANK_SIZE as isize) as usize;
        let single_push = Square::from_index(single_push_index);
        if self.is_empty(single_push) {
            push_promotion_moves(moves, piece, from, single_push, None);
        }
        // Left attack promotion
        if !from.is_first_file() {
            let left_attack: Square = match color {
                Color::White => Square::from_index(from.index() + TOP_LEFT),
                Color::Black => Square::from_index(from.index() - TOP_RIGHT),
            };
            if self.is_occupied_by_enemy(left_attack, color) {
                {
                    push_promotion_moves(
                        moves,
                        piece,
                        from,
                        left_attack,
                        self.piece_at_square(left_attack),
                    );
                };
            }
        }
        // Right attack promotion
        if !from.is_last_file() {
            let right_attack: Square = match color {
                Color::White => Square::from_index(from.index() + TOP_RIGHT),
                Color::Black => Square::from_index(from.index() - TOP_LEFT),
            };
            if self.is_occupied_by_enemy(right_attack, color) {
                push_promotion_moves(
                    moves,
                    piece,
                    from,
                    right_attack,
                    self.piece_at_square(right_attack),
                );
            };
        }
    }

    fn generate_moves_for_knight(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        let allowed_moves: [i32; 8] = [
            2 * 8 + 1,
            8 + 2,
            2 * 8 - 1,
            8 - 2,
            -2 * 8 - 1,
            -8 - 2,
            -2 * 8 + 1,
            -8 + 2,
        ];
        let allowed_ranks: [RangeInclusive<usize>; 8] =
            [0..=5, 0..=6, 0..=5, 0..=6, 2..=7, 1..=7, 2..=7, 1..=7];
        let allowed_files: [RangeInclusive<usize>; 8] =
            [0..=6, 0..=5, 1..=7, 2..=7, 1..=7, 2..=7, 0..=6, 0..=5];
        let piece = Piece {
            kind: PieceKind::Knight,
            color,
        };
        for (i, move_shift) in allowed_moves.into_iter().enumerate() {
            if !allowed_ranks[i].contains(&from.rank()) || !allowed_files[i].contains(&from.file())
            {
                continue;
            }
            let target = Square::from_index((from.index() as i32 + move_shift) as usize);
            if self.is_occupied_by_enemy(target, color) {
                moves.push(Move {
                    piece,
                    from,
                    to: target,
                    move_kind: MoveKind::Attack {
                        captured: self.piece_at_square(target).expect("Attacking a piece"),
                    },
                });
            } else if self.is_empty(target) {
                moves.push(Move {
                    piece,
                    from,
                    to: target,
                    move_kind: MoveKind::Quiet,
                });
            }
        }
    }

    fn generate_moves_for_directions(
        &self,
        moves: &mut Vec<Move>,
        directions: &[(isize, isize)],
        piece: Piece,
        from: Square,
    ) {
        for (rank_diff, file_diff) in directions {
            let mut r: isize = from.rank() as isize + rank_diff;
            let mut f: isize = from.file() as isize + file_diff;

            while (0..8).contains(&r) && (0..8).contains(&f) {
                let current = Square::from_rank_file(r as usize, f as usize);
                if self.is_occupied_by_friend(current, piece.color) {
                    break;
                } else if self.is_occupied_by_enemy(current, piece.color) {
                    moves.push(Move {
                        piece,
                        from,
                        to: current,
                        move_kind: MoveKind::Attack {
                            captured: self
                                .piece_at_square(current)
                                .expect("Attacking an enemy piece"),
                        },
                    });
                    break;
                } else {
                    moves.push(Move {
                        piece,
                        from,
                        to: current,
                        move_kind: MoveKind::Quiet,
                    });
                }
                r += rank_diff;
                f += file_diff;
            }
        }
    }

    fn generate_moves_for_rook(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        self.generate_moves_for_directions(
            moves,
            &directions,
            Piece {
                kind: PieceKind::Rook,
                color,
            },
            from,
        );
    }

    fn generate_moves_for_bishop(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        let directions: [(isize, isize); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
        self.generate_moves_for_directions(
            moves,
            &directions,
            Piece {
                kind: PieceKind::Bishop,
                color,
            },
            from,
        );
    }

    fn generate_moves_for_queen(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        self.generate_moves_for_directions(
            moves,
            &[
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ],
            Piece {
                kind: PieceKind::Bishop,
                color,
            },
            from,
        );
    }

    fn generate_moves_for_king(&self, moves: &mut Vec<Move>, from: Square, color: Color) {
        let directions: [(isize, isize); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        let king_controlled = self.controlled_by_king(color.enemy_color());
        let piece = Piece {
            kind: PieceKind::King,
            color,
        };
        for (rank_diff, file_diff) in directions {
            let r = from.rank() as isize + rank_diff;
            let f = from.file() as isize + file_diff;
            if !(0..8).contains(&r) || !(0..8).contains(&f) {
                continue;
            }
            let square = Square::from_rank_file(r as usize, f as usize);
            if king_controlled.contains(&square) {
                continue;
            }
            if self.is_occupied_by_enemy(square, color) {
                moves.push(Move {
                    piece,
                    from,
                    to: square,
                    move_kind: MoveKind::Attack {
                        captured: self
                            .piece_at_square(square)
                            .expect("Attacked piece has to be present."),
                    },
                });
            } else if self.is_empty(square) {
                moves.push(Move {
                    piece,
                    from,
                    to: square,
                    move_kind: MoveKind::Quiet,
                });
            }
        }
        // Castling
        if self.has_king_moved_yet(color) {
            return;
        }
        if !self.has_left_rook_moved_yet(color)
            && self.is_empty(Square::from_index(from.index() - 1))
            && self.is_empty(Square::from_index(from.index() - 2))
            && self.is_empty(Square::from_index(from.index() - 3))
        {
            moves.push(Move {
                piece,
                from,
                to: Square::from_index(from.index() - 2),
                move_kind: MoveKind::Castling {
                    side: CastleSide::QueenSide,
                },
            });
        }
        if !self.has_right_rook_moved_yet(color)
            && self.is_empty(Square::from_index(from.index() + 1))
            && self.is_empty(Square::from_index(from.index() + 2))
        {
            moves.push(Move {
                piece,
                from,
                to: Square::from_index(from.index() + 2),
                move_kind: MoveKind::Castling {
                    side: CastleSide::KingSide,
                },
            });
        }
    }

    fn has_queen_side_castling_rights(&self, color: Color) -> bool {
        if self.has_king_moved_yet(color) {
            return false;
        }
        let king = self.find_king(color);
        !self.has_left_rook_moved_yet(color)
            && self.is_empty(Square::from_index(king.index() - 1))
            && self.is_empty(Square::from_index(king.index() - 2))
            && self.is_empty(Square::from_index(king.index() - 3))
    }

    fn has_king_side_castling_rights(&self, color: Color) -> bool {
        if self.has_king_moved_yet(color) {
            return false;
        }
        let king = self.find_king(color);
        !self.has_right_rook_moved_yet(color)
            && self.is_empty(Square::from_index(king.index() + 1))
            && self.is_empty(Square::from_index(king.index() + 2))
    }

    fn controlled_by_king(&self, color: Color) -> Vec<Square> {
        let mut controlled = vec![];
        let king = self.find_king(color);
        let directions: [(isize, isize); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for (rank_diff, file_diff) in directions {
            let r = king.rank() as isize + rank_diff;
            let f = king.file() as isize + file_diff;
            if !(0..8).contains(&r) || !(0..8).contains(&f) {
                continue;
            }
            controlled.push(Square::from_rank_file(r as usize, f as usize));
        }
        controlled
    }
}

fn push_promotion_moves(
    moves: &mut Vec<Move>,
    piece: Piece,
    from: Square,
    to: Square,
    captured: Option<Piece>,
) {
    let promotion_pieces = [
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Rook,
        PieceKind::Queen,
    ];
    promotion_pieces.into_iter().for_each(|kind| {
        moves.push(Move {
            piece,
            from: from,
            to: to,
            move_kind: MoveKind::Promotion {
                promoted_to: kind,
                captured: captured,
            },
        })
    });
}

#[test]
fn test_pawn_move_generation() {
    let board: Board = Board::new();
    let mut moves: Vec<Move> = vec![];
    board.generate_moves_for_pawn(&mut moves, Square::A2, Color::White);
    let expected = vec![
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::A2,
            to: Square::A3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::A2,
            to: Square::A4,
            move_kind: MoveKind::Quiet,
        },
    ];
    assert_eq!(moves, expected);
}

#[test]
fn test_initial_moves_for_white() {
    let board: Board = Board::new();
    let result = board.generate_moves(Color::White);
    let expected = vec![
        Move {
            piece: Piece {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            from: Square::B1,
            to: Square::C3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            from: Square::B1,
            to: Square::A3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            from: Square::G1,
            to: Square::H3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Knight,
                color: Color::White,
            },
            from: Square::G1,
            to: Square::F3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::A2,
            to: Square::A3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::A2,
            to: Square::A4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::B2,
            to: Square::B3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::B2,
            to: Square::B4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::C2,
            to: Square::C3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::C2,
            to: Square::C4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D2,
            to: Square::D3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D2,
            to: Square::D4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::E2,
            to: Square::E3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::E2,
            to: Square::E4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::F2,
            to: Square::F3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::F2,
            to: Square::F4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::G2,
            to: Square::G3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::G2,
            to: Square::G4,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::H2,
            to: Square::H3,
            move_kind: MoveKind::Quiet,
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::H2,
            to: Square::H4,
            move_kind: MoveKind::Quiet,
        },
    ];
    assert_eq!(result, expected)
}

#[test]
fn test_simple_game() {
    let mut board = Board::new();
    let game = [
        (
            Square::E2,
            Square::E4,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR",
        ),
        (
            Square::E7,
            Square::E5,
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR",
        ),
        (
            Square::G1,
            Square::F3,
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R",
        ),
        (
            Square::B8,
            Square::C6,
            "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R",
        ),
        (
            Square::F1,
            Square::C4,
            "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R",
        ),
        (
            Square::D7,
            Square::D6,
            "r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R",
        ),
        (
            Square::B1,
            Square::C3,
            "r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R",
        ),
        (
            Square::C8,
            Square::G4,
            "r2qkbnr/ppp2ppp/2np4/4p3/2B1P1b1/2N2N2/PPPP1PPP/R1BQK2R",
        ),
        (
            Square::H2,
            Square::H3,
            "r2qkbnr/ppp2ppp/2np4/4p3/2B1P1b1/2N2N1P/PPPP1PP1/R1BQK2R",
        ),
        (
            Square::G4,
            Square::H5,
            "r2qkbnr/ppp2ppp/2np4/4p2b/2B1P3/2N2N1P/PPPP1PP1/R1BQK2R",
        ),
        (
            Square::F3,
            Square::E5,
            "r2qkbnr/ppp2ppp/2np4/4N2b/2B1P3/2N4P/PPPP1PP1/R1BQK2R",
        ),
        (
            Square::H5,
            Square::D1,
            "r2qkbnr/ppp2ppp/2np4/4N3/2B1P3/2N4P/PPPP1PP1/R1BbK2R",
        ),
        (
            Square::C4,
            Square::F7,
            "r2qkbnr/ppp2Bpp/2np4/4N3/4P3/2N4P/PPPP1PP1/R1BbK2R",
        ),
        (
            Square::E8,
            Square::E7,
            "r2q1bnr/ppp1kBpp/2np4/4N3/4P3/2N4P/PPPP1PP1/R1BbK2R",
        ),
        (
            Square::C3,
            Square::D5,
            "r2q1bnr/ppp1kBpp/2np4/3NN3/4P3/7P/PPPP1PP1/R1BbK2R",
        ),
    ];

    for (ply, (from, to, expected_fen)) in game.into_iter().enumerate() {
        let generated = board.generate_moves(board.turn());
        let selected = generated
            .into_iter()
            .find(|m| m.from == from && m.to == to)
            .unwrap_or_else(|| {
                panic!(
                    "Expected move {:?} -> {:?} to be generated at ply {}",
                    from,
                    to,
                    ply + 1
                )
            });
        println!("Executing move {:?}", selected);
        board.make_move(&selected);
        assert_eq!(
            board.serialize_to_fen(),
            expected_fen,
            "Unexpected FEN after ply {} ({:?} -> {:?})",
            ply + 1,
            from,
            to
        );
    }
}

#[cfg(test)]
fn find_generated_move(
    board: &Board,
    from: Square,
    to: Square,
    promotion: Option<PieceKind>,
) -> Move {
    board
        .generate_moves(board.turn())
        .into_iter()
        .find(|m| {
            m.from == from
                && m.to == to
                && match (promotion, m.move_kind) {
                    (Some(expected), MoveKind::Promotion { promoted_to, .. }) => {
                        promoted_to == expected
                    }
                    (None, MoveKind::Promotion { .. }) => false,
                    (None, _) => true,
                    _ => false,
                }
        })
        .unwrap_or_else(|| {
            panic!(
                "Expected generated move {:?} -> {:?} with promotion {:?}",
                from, to, promotion
            )
        })
}

#[cfg(test)]
fn place_test_piece(board: &mut Board, square: Square, kind: PieceKind, color: Color) {
    board.set_piece_at_square(square, Piece { kind, color });
}

#[cfg(test)]
fn castling_test_board() -> Board {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::A1, PieceKind::Rook, Color::White);
    place_test_piece(&mut board, Square::E1, PieceKind::King, Color::White);
    place_test_piece(&mut board, Square::H1, PieceKind::Rook, Color::White);
    place_test_piece(&mut board, Square::A8, PieceKind::Rook, Color::Black);
    place_test_piece(&mut board, Square::E8, PieceKind::King, Color::Black);
    place_test_piece(&mut board, Square::H8, PieceKind::Rook, Color::Black);
    board
}

#[cfg(test)]
fn castling_test_board_black_to_move() -> Board {
    let mut board = castling_test_board();
    board.moves.push(Move {
        piece: Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        },
        from: Square::A2,
        to: Square::A3,
        move_kind: MoveKind::Quiet,
    });
    board
}

#[test]
fn test_real_en_passant() {
    // Steinitz vs Fleissig, Vienna 1882:
    // 1. e4 e6 2. e5 d5 3. exd6 e.p.
    let mut board = Board::new();
    let opening = [
        (
            Square::E2,
            Square::E4,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR",
        ),
        (
            Square::E7,
            Square::E6,
            "rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR",
        ),
        (
            Square::E4,
            Square::E5,
            "rnbqkbnr/pppp1ppp/4p3/4P3/8/8/PPPP1PPP/RNBQKBNR",
        ),
        (
            Square::D7,
            Square::D5,
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR",
        ),
    ];

    for (from, to, expected_fen) in opening {
        let m = find_generated_move(&board, from, to, None);
        board.make_move(&m);
        assert_eq!(board.serialize_to_fen(), expected_fen);
    }

    let en_passant = find_generated_move(&board, Square::E5, Square::D6, None);
    assert_eq!(
        en_passant.move_kind,
        MoveKind::EnPassant {
            captured_square_pawn: Square::D5,
        }
    );
    board.make_move(&en_passant);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbqkbnr/ppp2ppp/3Pp3/8/8/8/PPPP1PPP/RNBQKBNR"
    );
}

#[test]
fn test_white_en_passant_to_left() {
    let mut board = Board::new();

    let e2e4 = find_generated_move(&board, Square::E2, Square::E4, None);
    board.make_move(&e2e4);

    let e7e6 = find_generated_move(&board, Square::E7, Square::E6, None);
    board.make_move(&e7e6);

    let e4e5 = find_generated_move(&board, Square::E4, Square::E5, None);
    board.make_move(&e4e5);

    let d7d5 = find_generated_move(&board, Square::D7, Square::D5, None);
    board.make_move(&d7d5);

    let en_passant = find_generated_move(&board, Square::E5, Square::D6, None);
    assert_eq!(
        en_passant.move_kind,
        MoveKind::EnPassant {
            captured_square_pawn: Square::D5,
        }
    );

    board.make_move(&en_passant);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbqkbnr/ppp2ppp/3Pp3/8/8/8/PPPP1PPP/RNBQKBNR"
    );
}

#[test]
fn test_white_en_passant_to_right() {
    let mut board = Board::new();

    let d2d4 = find_generated_move(&board, Square::D2, Square::D4, None);
    board.make_move(&d2d4);

    let g8f6 = find_generated_move(&board, Square::G8, Square::F6, None);
    board.make_move(&g8f6);

    let d4d5 = find_generated_move(&board, Square::D4, Square::D5, None);
    board.make_move(&d4d5);

    let e7e5 = find_generated_move(&board, Square::E7, Square::E5, None);
    board.make_move(&e7e5);

    let en_passant = find_generated_move(&board, Square::D5, Square::E6, None);
    assert_eq!(
        en_passant.move_kind,
        MoveKind::EnPassant {
            captured_square_pawn: Square::E5,
        }
    );

    board.make_move(&en_passant);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbqkb1r/pppp1ppp/4Pn2/8/8/8/PPP1PPPP/RNBQKBNR"
    );
}

#[test]
fn test_black_en_passant_to_left() {
    let mut board = Board::new();

    let g1f3 = find_generated_move(&board, Square::G1, Square::F3, None);
    board.make_move(&g1f3);

    let e7e5 = find_generated_move(&board, Square::E7, Square::E5, None);
    board.make_move(&e7e5);

    let f3g1 = find_generated_move(&board, Square::F3, Square::G1, None);
    board.make_move(&f3g1);

    let e5e4 = find_generated_move(&board, Square::E5, Square::E4, None);
    board.make_move(&e5e4);

    let d2d4 = find_generated_move(&board, Square::D2, Square::D4, None);
    board.make_move(&d2d4);

    let en_passant = find_generated_move(&board, Square::E4, Square::D3, None);
    assert_eq!(
        en_passant.move_kind,
        MoveKind::EnPassant {
            captured_square_pawn: Square::D4,
        }
    );

    board.make_move(&en_passant);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbqkbnr/pppp1ppp/8/8/8/3p4/PPP1PPPP/RNBQKBNR"
    );
}

#[test]
fn test_black_en_passant_to_right() {
    let mut board = Board::new();

    let g1f3 = find_generated_move(&board, Square::G1, Square::F3, None);
    board.make_move(&g1f3);

    let d7d5 = find_generated_move(&board, Square::D7, Square::D5, None);
    board.make_move(&d7d5);

    let f3g1 = find_generated_move(&board, Square::F3, Square::G1, None);
    board.make_move(&f3g1);

    let d5d4 = find_generated_move(&board, Square::D5, Square::D4, None);
    board.make_move(&d5d4);

    let e2e4 = find_generated_move(&board, Square::E2, Square::E4, None);
    board.make_move(&e2e4);

    let en_passant = find_generated_move(&board, Square::D4, Square::E3, None);
    assert_eq!(
        en_passant.move_kind,
        MoveKind::EnPassant {
            captured_square_pawn: Square::E4,
        }
    );

    board.make_move(&en_passant);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbqkbnr/ppp1pppp/8/8/8/4p3/PPPP1PPP/RNBQKBNR"
    );
}

#[test]
fn test_real_promotion() {
    // Amin-Erdene vs Ilandzis, World Amateur U2300, Rhodes 2024:
    // ... 51...Nf8 52. d8=Q
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::C8, PieceKind::Bishop, Color::Black);
    place_test_piece(&mut board, Square::F8, PieceKind::Knight, Color::Black);
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::H7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::A6, PieceKind::Pawn, Color::Black);
    place_test_piece(&mut board, Square::F6, PieceKind::King, Color::White);
    place_test_piece(&mut board, Square::D5, PieceKind::King, Color::Black);
    place_test_piece(&mut board, Square::E5, PieceKind::Pawn, Color::Black);
    place_test_piece(&mut board, Square::B3, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::A2, PieceKind::Pawn, Color::White);

    let promotion = find_generated_move(&board, Square::D7, Square::D8, Some(PieceKind::Queen));
    board.make_move(&promotion);
    assert_eq!(board.serialize_to_fen(), "2bQ1n2/7P/p4K2/3kp3/8/1P6/P7/8");
}

#[test]
fn test_promotion_generates_all_quiet_choices() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);

    let generated = board.generate_moves(Color::White);
    let expected = vec![
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::D8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Knight,
                captured: None,
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::D8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Bishop,
                captured: None,
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::D8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Rook,
                captured: None,
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::D8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Queen,
                captured: None,
            },
        },
    ];

    assert_eq!(generated, expected);
}

#[test]
fn test_promotion_generates_all_left_capture_choices() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::D8, PieceKind::Rook, Color::White);
    place_test_piece(&mut board, Square::C8, PieceKind::Bishop, Color::Black);

    let generated = board.generate_moves(Color::White);
    let left_capture_promotions: Vec<Move> = generated
        .into_iter()
        .filter(|m| m.from == Square::D7 && m.to == Square::C8)
        .collect();

    let expected = vec![
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::C8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Knight,
                captured: Some(Piece {
                    kind: PieceKind::Bishop,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::C8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Bishop,
                captured: Some(Piece {
                    kind: PieceKind::Bishop,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::C8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Rook,
                captured: Some(Piece {
                    kind: PieceKind::Bishop,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::C8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Queen,
                captured: Some(Piece {
                    kind: PieceKind::Bishop,
                    color: Color::Black,
                }),
            },
        },
    ];

    assert_eq!(left_capture_promotions, expected);
}

#[test]
fn test_promotion_generates_all_right_capture_choices() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::D8, PieceKind::Rook, Color::White);
    place_test_piece(&mut board, Square::E8, PieceKind::Rook, Color::Black);

    let generated = board.generate_moves(Color::White);
    let right_capture_promotions: Vec<Move> = generated
        .into_iter()
        .filter(|m| m.from == Square::D7 && m.to == Square::E8)
        .collect();

    let expected = vec![
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::E8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Knight,
                captured: Some(Piece {
                    kind: PieceKind::Rook,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::E8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Bishop,
                captured: Some(Piece {
                    kind: PieceKind::Rook,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::E8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Rook,
                captured: Some(Piece {
                    kind: PieceKind::Rook,
                    color: Color::Black,
                }),
            },
        },
        Move {
            piece: Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            },
            from: Square::D7,
            to: Square::E8,
            move_kind: MoveKind::Promotion {
                promoted_to: PieceKind::Queen,
                captured: Some(Piece {
                    kind: PieceKind::Rook,
                    color: Color::Black,
                }),
            },
        },
    ];

    assert_eq!(right_capture_promotions, expected);
}

#[test]
fn test_quiet_promotion_to_knight() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);

    let promotion = find_generated_move(&board, Square::D7, Square::D8, Some(PieceKind::Knight));
    board.make_move(&promotion);

    assert_eq!(board.serialize_to_fen(), "3N4/8/8/8/8/8/8/8");
}

#[test]
fn test_capture_promotion_to_rook() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::C8, PieceKind::Bishop, Color::Black);

    let promotion = find_generated_move(&board, Square::D7, Square::C8, Some(PieceKind::Rook));
    board.make_move(&promotion);

    assert_eq!(board.serialize_to_fen(), "2R5/8/8/8/8/8/8/8");
}

#[test]
fn test_real_castling() {
    // Becx vs Blok, Open NK Rapid 2025:
    // 1. f4 d6 2. Nf3 Nf6 3. d3 g6 4. e4 Bg7 5. g3 O-O
    let mut board = Board::new();
    let setup = [
        (
            Square::F2,
            Square::F4,
            "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR",
        ),
        (
            Square::D7,
            Square::D6,
            "rnbqkbnr/ppp1pppp/3p4/8/5P2/8/PPPPP1PP/RNBQKBNR",
        ),
        (
            Square::G1,
            Square::F3,
            "rnbqkbnr/ppp1pppp/3p4/8/5P2/5N2/PPPPP1PP/RNBQKB1R",
        ),
        (
            Square::G8,
            Square::F6,
            "rnbqkb1r/ppp1pppp/3p1n2/8/5P2/5N2/PPPPP1PP/RNBQKB1R",
        ),
        (
            Square::D2,
            Square::D3,
            "rnbqkb1r/ppp1pppp/3p1n2/8/5P2/3P1N2/PPP1P1PP/RNBQKB1R",
        ),
        (
            Square::G7,
            Square::G6,
            "rnbqkb1r/ppp1pp1p/3p1np1/8/5P2/3P1N2/PPP1P1PP/RNBQKB1R",
        ),
        (
            Square::E2,
            Square::E4,
            "rnbqkb1r/ppp1pp1p/3p1np1/8/4PP2/3P1N2/PPP3PP/RNBQKB1R",
        ),
        (
            Square::F8,
            Square::G7,
            "rnbqk2r/ppp1ppbp/3p1np1/8/4PP2/3P1N2/PPP3PP/RNBQKB1R",
        ),
        (
            Square::G2,
            Square::G3,
            "rnbqk2r/ppp1ppbp/3p1np1/8/4PP2/3P1NP1/PPP4P/RNBQKB1R",
        ),
    ];

    for (from, to, expected_fen) in setup {
        let m = find_generated_move(&board, from, to, None);
        board.make_move(&m);
        assert_eq!(board.serialize_to_fen(), expected_fen);
    }

    let castle = find_generated_move(&board, Square::E8, Square::G8, None);
    assert_eq!(
        castle.move_kind,
        MoveKind::Castling {
            side: CastleSide::KingSide,
        }
    );
    board.make_move(&castle);
    assert_eq!(
        board.serialize_to_fen(),
        "rnbq1rk1/ppp1ppbp/3p1np1/8/4PP2/3P1NP1/PPP4P/RNBQKB1R"
    );
}

#[test]
fn test_castling_round_trip_white_king_side() {
    let mut board = castling_test_board();

    let original_fen = board.serialize_to_fen();
    let generated = board.generate_moves(Color::White);
    let castle = generated
        .into_iter()
        .find(|m| m.from == Square::E1 && m.to == Square::G1)
        .expect("Expected white king-side castle to be generated");

    board.make_move(&castle);
    assert_eq!(board.serialize_to_fen(), "r3k2r/8/8/8/8/8/8/R4RK1");
    board.umake_move();
    assert_eq!(board.serialize_to_fen(), original_fen);
}

#[test]
fn test_castling_round_trip_white_queen_side() {
    let mut board = castling_test_board();

    let original_fen = board.serialize_to_fen();
    let generated = board.generate_moves(Color::White);
    let castle = generated
        .into_iter()
        .find(|m| m.from == Square::E1 && m.to == Square::C1)
        .expect("Expected white queen-side castle to be generated");

    board.make_move(&castle);
    assert_eq!(board.serialize_to_fen(), "r3k2r/8/8/8/8/8/8/2KR3R");
    board.umake_move();
    assert_eq!(board.serialize_to_fen(), original_fen);
}

#[test]
fn test_castling_round_trip_black_king_side() {
    let mut board = castling_test_board_black_to_move();

    let original_fen = board.serialize_to_fen();
    let generated = board.generate_moves(Color::Black);
    let castle = generated
        .into_iter()
        .find(|m| m.from == Square::E8 && m.to == Square::G8)
        .expect("Expected black king-side castle to be generated");

    board.make_move(&castle);
    assert_eq!(board.serialize_to_fen(), "r4rk1/8/8/8/8/8/8/R3K2R");
    board.umake_move();
    assert_eq!(board.serialize_to_fen(), original_fen);
}

#[test]
fn test_castling_round_trip_black_queen_side() {
    let mut board = castling_test_board_black_to_move();

    let original_fen = board.serialize_to_fen();
    let generated = board.generate_moves(Color::Black);
    let castle = generated
        .into_iter()
        .find(|m| m.from == Square::E8 && m.to == Square::C8)
        .expect("Expected black queen-side castle to be generated");

    board.make_move(&castle);
    assert_eq!(board.serialize_to_fen(), "2kr3r/8/8/8/8/8/8/R3K2R");
    board.umake_move();
    assert_eq!(board.serialize_to_fen(), original_fen);
}
