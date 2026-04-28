use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result;
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
    pub(crate) fn enemy_color(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub(crate) fn pawn_start_rank(&self) -> usize {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    pub(crate) fn pawn_direction(&self) -> isize {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    pub(crate) fn promotion_rank(&self) -> usize {
        match self {
            Color::White => 6,
            Color::Black => 1,
        }
    }

    pub(crate) fn en_passant_target_rank(&self) -> usize {
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

    pub(crate) fn is_pawn(&self) -> bool {
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
    pub(crate) moves: Vec<Move>,
}

pub trait Serializer {
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
pub enum CastleSide {
    QueenSide,
    KingSide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveKind {
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
pub struct Move {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub move_kind: MoveKind,
}

impl Move {
    pub(crate) fn is_en_passantable(self) -> bool {
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
