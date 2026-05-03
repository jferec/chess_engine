use crate::board::*;
use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

pub static FILE_RANGE: Range<isize> = 0..8;
pub static RANK_RANGE: Range<isize> = 0..8;

pub trait MoveGenerator {
    fn generate_moves(&self, turn: Color) -> Vec<Move>;
}

impl MoveGenerator for Board {
    fn generate_moves(&self, turn: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let king_safety = self.compute_king_safety(turn);
        let friendly_pieces = self.get_pieces(turn);
        for square in friendly_pieces {
            let piece = self
                .maybe_piece_at_square(square)
                .expect("Piece must be present.");
            self.generate_moves_for_piece(
                &GenerateMovesRequest {
                    piece: piece,
                    square,
                },
                &mut moves,
                &king_safety,
            );
        }
        moves
    }
}

/// king          = king position
/// checkers      = enemy pieces checking our king
/// danger        = squares attacked by the enemy, for king moves
/// evation_mask  = squares that can block/capture a single check (for non-king moves)
struct CheckStatus {
    king: Square,
    checkers: BitBoard,
    danger: BitBoard,
    evation_mask: BitBoard,
}

struct PinStatus {
    pinned: BitBoard,
    pin_ray: HashMap<Square, BitBoard>,
}

impl Board {
    fn compute_king_safety(&self, color: Color) -> KingSafety {
        let check_status = self.compute_check_status(color);

        KingSafety {
            color: color,
            king: check_status.king,
            checkers: check_status.checkers,
            pinned: BitBoard(0),
            pin_ray: HashMap::new(),
            danger: check_status.danger,
            evation_mask: check_status.evation_mask,
        }
    }

    fn compute_check_status(&self, color: Color) -> CheckStatus {
        let king = self.find_king(color);
        let enemy_color = color.enemy_color();
        let enemies = self.get_pieces(enemy_color);
        let mut check_status = CheckStatus {
            king: king,
            checkers: BitBoard::new(),
            danger: BitBoard::new(),
            evation_mask: BitBoard::new(),
        };
        for square in enemies {
            let piece = self.piece_at_square(square);
            match piece.kind {
                PieceKind::Pawn => {
                    let att_dir = enemy_color.pawn_direction();
                    let attacks: [(isize, isize); 2] = [(att_dir, -1), (att_dir, 1)];
                    for attack in &attacks {
                        let rank = square.rank() as isize + attack.0;
                        let file = square.file() as isize + attack.1;
                        if RANK_RANGE.contains(&rank) && FILE_RANGE.contains(&file) {
                            let attacked_field =
                                BitBoard::from_rank_file(rank as usize, file as usize);
                            check_status.danger |= attacked_field;
                            if attacked_field.has_square(king) {
                                check_status.checkers |= BitBoard::from_square(square);
                                check_status.evation_mask |= BitBoard::from_square(square);
                            }
                        }
                    }
                }
                PieceKind::Knight => {
                    let attacks: [(isize, isize); 8] = [
                        (2, 1),
                        (1, 2),
                        (-1, 2),
                        (-2, 1),
                        (-2, -1),
                        (-1, -2),
                        (1, -2),
                        (2, -1),
                    ];
                    for attack in &attacks {
                        let rank = square.rank() as isize + attack.0;
                        let file = square.file() as isize + attack.1;
                        if RANK_RANGE.contains(&rank) && FILE_RANGE.contains(&file) {
                            let attacked_field =
                                BitBoard::from_rank_file(rank as usize, file as usize);
                            check_status.danger |= attacked_field;
                            if attacked_field.has_square(king) {
                                check_status.checkers |= BitBoard::from_square(square);
                                check_status.evation_mask |= BitBoard::from_square(square);
                            }
                        }
                    }
                }
                PieceKind::Bishop => {
                    let dir = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
                    self.evaluate_check_status_for_sliding_figures(
                        &dir,
                        square,
                        color,
                        &mut check_status,
                    );
                }
                PieceKind::Rook => {
                    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
                    self.evaluate_check_status_for_sliding_figures(
                        &dir,
                        square,
                        color,
                        &mut check_status,
                    );
                }
                PieceKind::Queen => {
                    let dir = [
                        (1, 0),
                        (0, 1),
                        (-1, 0),
                        (0, -1),
                        (1, 1),
                        (-1, -1),
                        (1, -1),
                        (-1, 1),
                    ];
                    self.evaluate_check_status_for_sliding_figures(
                        &dir,
                        square,
                        color,
                        &mut check_status,
                    );
                }
                PieceKind::King => {
                    let attacks: [(isize, isize); 8] = [
                        (1, 0),
                        (0, 1),
                        (-1, 0),
                        (0, -1),
                        (1, 1),
                        (-1, -1),
                        (1, -1),
                        (-1, 1),
                    ];
                    for attack in &attacks {
                        let r = square.rank() as isize + attack.0;
                        let f = square.file() as isize + attack.1;
                        if RANK_RANGE.contains(&r) && FILE_RANGE.contains(&f) {
                            let attacked_field = BitBoard::from_rank_file(r as usize, f as usize);
                            check_status.danger |= attacked_field;
                            if attacked_field.has_square(king) {
                                check_status.checkers |= BitBoard::from_square(square);
                                check_status.evation_mask |= BitBoard::from_square(square);
                            }
                        }
                    }
                }
            }
        }
        check_status
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

    fn generate_moves_for_piece(
        &self,
        request: &GenerateMovesRequest,
        moves: &mut Vec<Move>,
        king_safety: &KingSafety,
    ) {
        match request.piece.kind {
            PieceKind::Pawn => {
                self.generate_moves_for_pawn(moves, request.square, request.piece.color)
            }
            PieceKind::Bishop => {
                self.generate_moves_for_bishop(moves, request.square, request.piece.color)
            }
            PieceKind::Knight => {
                self.generate_moves_for_knight(moves, request.square, request.piece.color)
            }
            PieceKind::Rook => {
                self.generate_moves_for_rook(moves, request.square, request.piece.color);
            }
            PieceKind::Queen => {
                self.generate_moves_for_queen(moves, request.square, request.piece.color);
            }
            PieceKind::King => {
                self.generate_moves_for_king(moves, request.square, request.piece.color);
            }
        }
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
                            .maybe_piece_at_square(left_attack)
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
                            .maybe_piece_at_square(right_attack)
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
                        self.maybe_piece_at_square(left_attack),
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
                    self.maybe_piece_at_square(right_attack),
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
                        captured: self
                            .maybe_piece_at_square(target)
                            .expect("Attacking a piece"),
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
                                .maybe_piece_at_square(current)
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

    /// Evaluates KingCheckStatus for sliding enemy figures: Rook, Bishop and Queen.
    fn evaluate_check_status_for_sliding_figures(
        &self,
        directions: &[(isize, isize)],
        enemy_square: Square,
        king_color: Color,
        check_status: &mut CheckStatus,
    ) {
        let king = self.find_king(king_color);
        for (rank_diff, file_diff) in directions {
            let mut r: isize = enemy_square.rank() as isize + rank_diff;
            let mut f: isize = enemy_square.file() as isize + file_diff;
            let mut dir_danger = BitBoard::new();

            while RANK_RANGE.contains(&r) && FILE_RANGE.contains(&f) {
                let current = Square::from_rank_file(r as usize, f as usize);
                dir_danger |= BitBoard::from_square(current);
                if current == king {
                    // If king is attacked by an enemy piece, we've got a checker.
                    check_status.checkers |= BitBoard::from_square(enemy_square);
                    // Since this is a sliding attack, entire ray in between the enemy
                    // and the king becomes an evasion mask, including the attacker itself
                    check_status.evation_mask |= dir_danger | BitBoard::from_square(enemy_square);
                }
                if self.is_occupied(current) {
                    break;
                }
                r += rank_diff;
                f += file_diff;
            }
            check_status.danger |= dir_danger
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
                            .maybe_piece_at_square(square)
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
    let generated = board.generate_moves(board.turn());
    generated
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
fn find_generated_pawn_move(
    board: &Board,
    from: Square,
    to: Square,
    promotion: Option<PieceKind>,
) -> Move {
    let piece = board.piece_at_square(from);
    assert_eq!(piece.kind, PieceKind::Pawn);

    let mut generated = vec![];
    board.generate_moves_for_pawn(&mut generated, from, piece.color);
    generated
        .into_iter()
        .find(|m| {
            m.to == to
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
                "Expected generated pawn move {:?} -> {:?} with promotion {:?}",
                from, to, promotion
            )
        })
}

#[cfg(test)]
fn place_test_piece(board: &mut Board, square: Square, kind: PieceKind, color: Color) {
    board.set_piece_at_square(square, Piece { kind, color });
}

#[cfg(test)]
fn bitboard_from_squares(squares: &[Square]) -> BitBoard {
    let mut bitboard = BitBoard::new();
    for &square in squares {
        bitboard |= BitBoard::from_square(square);
    }
    bitboard
}

#[test]
fn test_find_king_attackers_single_pawn_check() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::E4, PieceKind::King, Color::White);
    place_test_piece(&mut board, Square::D5, PieceKind::Pawn, Color::Black);
    place_test_piece(&mut board, Square::H8, PieceKind::King, Color::Black);

    let check_status = board.compute_check_status(Color::White);

    assert_eq!(check_status.king, Square::E4);
    assert_eq!(check_status.checkers, BitBoard::from_square(Square::D5));
    assert_eq!(check_status.evation_mask, BitBoard::from_square(Square::D5));
    assert!(check_status.danger.has_square(Square::E4));
}

#[test]
fn test_find_king_attackers_single_sliding_piece_check() {
    let cases = [
        (
            PieceKind::Bishop,
            Square::B5,
            Square::E2,
            bitboard_from_squares(&[Square::B5, Square::C4, Square::D3, Square::E2]),
        ),
        (
            PieceKind::Rook,
            Square::E8,
            Square::E1,
            bitboard_from_squares(&[
                Square::E8,
                Square::E7,
                Square::E6,
                Square::E5,
                Square::E4,
                Square::E3,
                Square::E2,
                Square::E1,
            ]),
        ),
        (
            PieceKind::Queen,
            Square::A4,
            Square::E4,
            bitboard_from_squares(&[Square::A4, Square::B4, Square::C4, Square::D4, Square::E4]),
        ),
    ];

    for (attacker_kind, attacker_square, king_square, expected_evasion_mask) in cases {
        let mut board = Board::new_empty_board();
        place_test_piece(&mut board, king_square, PieceKind::King, Color::White);
        place_test_piece(&mut board, attacker_square, attacker_kind, Color::Black);
        place_test_piece(&mut board, Square::H8, PieceKind::King, Color::Black);

        let check_status = board.compute_check_status(Color::White);

        assert_eq!(check_status.king, king_square);
        assert_eq!(
            check_status.checkers,
            BitBoard::from_square(attacker_square),
            "expected {:?} on {:?} to check king on {:?}",
            attacker_kind,
            attacker_square,
            king_square,
        );
        assert_eq!(check_status.evation_mask, expected_evasion_mask);
        assert!(check_status.danger.has_square(king_square));
    }
}

#[test]
fn test_find_king_attackers_double_check() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::E4, PieceKind::King, Color::White);
    place_test_piece(&mut board, Square::E8, PieceKind::Rook, Color::Black);
    place_test_piece(&mut board, Square::H1, PieceKind::Bishop, Color::Black);
    place_test_piece(&mut board, Square::A8, PieceKind::King, Color::Black);

    let check_status = board.compute_check_status(Color::White);

    assert_eq!(check_status.king, Square::E4);
    assert_eq!(
        check_status.checkers,
        bitboard_from_squares(&[Square::E8, Square::H1])
    );
    assert!(check_status.danger.has_square(Square::E4));
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct AttackStateCase {
    attacker: Piece,
    attacker_square: Square,
    attacker_king_square: Option<Square>,
    defender_king_square: Square,
}

#[cfg(test)]
fn assert_attack_state(case: AttackStateCase) {
    let defender = case.attacker.color.enemy_color();
    let mut board = Board::new_empty_board();

    place_test_piece(
        &mut board,
        case.defender_king_square,
        PieceKind::King,
        defender,
    );
    place_test_piece(
        &mut board,
        case.attacker_square,
        case.attacker.kind,
        case.attacker.color,
    );
    if let Some(attacker_king_square) = case.attacker_king_square {
        place_test_piece(
            &mut board,
            attacker_king_square,
            PieceKind::King,
            case.attacker.color,
        );
    }

    let king_safety = board.compute_king_safety(defender);

    assert_eq!(king_safety.color, defender);
    assert_eq!(king_safety.king, case.defender_king_square);
    assert_eq!(
        king_safety.checkers,
        BitBoard::from_square(case.attacker_square),
        "expected {:?} {:?} on {:?} to check {:?} king on {:?}",
        case.attacker.color,
        case.attacker.kind,
        case.attacker_square,
        defender,
        case.defender_king_square,
    );
    assert_eq!(
        king_safety.danger & BitBoard::from_square(case.defender_king_square),
        BitBoard::from_square(case.defender_king_square),
        "expected {:?} king on {:?} to be in danger from {:?} {:?} on {:?}",
        defender,
        case.defender_king_square,
        case.attacker.color,
        case.attacker.kind,
        case.attacker_square,
    );
}

#[cfg(test)]
fn assert_attack_states(cases: [AttackStateCase; 3]) {
    for case in cases {
        assert_attack_state(case);
    }
}

#[cfg(test)]
fn attack_case(
    attacker_color: Color,
    attacker_kind: PieceKind,
    attacker_square: Square,
    attacker_king_square: Square,
    defender_king_square: Square,
) -> AttackStateCase {
    AttackStateCase {
        attacker: Piece {
            kind: attacker_kind,
            color: attacker_color,
        },
        attacker_square,
        attacker_king_square: Some(attacker_king_square),
        defender_king_square,
    }
}

#[cfg(test)]
fn king_attack_case(
    attacker_color: Color,
    attacker_square: Square,
    defender_king_square: Square,
) -> AttackStateCase {
    AttackStateCase {
        attacker: Piece {
            kind: PieceKind::King,
            color: attacker_color,
        },
        attacker_square,
        attacker_king_square: None,
        defender_king_square,
    }
}

#[test]
fn test_white_pawn_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::White,
            PieceKind::Pawn,
            Square::C4,
            Square::H1,
            Square::B5,
        ),
        attack_case(
            Color::White,
            PieceKind::Pawn,
            Square::F2,
            Square::A1,
            Square::G3,
        ),
        attack_case(
            Color::White,
            PieceKind::Pawn,
            Square::G6,
            Square::A1,
            Square::H7,
        ),
    ]);
}

#[test]
fn test_black_pawn_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::Black,
            PieceKind::Pawn,
            Square::C5,
            Square::H8,
            Square::B4,
        ),
        attack_case(
            Color::Black,
            PieceKind::Pawn,
            Square::F7,
            Square::A8,
            Square::G6,
        ),
        attack_case(
            Color::Black,
            PieceKind::Pawn,
            Square::B3,
            Square::H8,
            Square::C2,
        ),
    ]);
}

#[test]
fn test_white_knight_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::White,
            PieceKind::Knight,
            Square::D4,
            Square::A1,
            Square::F5,
        ),
        attack_case(
            Color::White,
            PieceKind::Knight,
            Square::B1,
            Square::H1,
            Square::C3,
        ),
        attack_case(
            Color::White,
            PieceKind::Knight,
            Square::G6,
            Square::A1,
            Square::E7,
        ),
    ]);
}

#[test]
fn test_black_knight_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::Black,
            PieceKind::Knight,
            Square::E5,
            Square::H8,
            Square::F3,
        ),
        attack_case(
            Color::Black,
            PieceKind::Knight,
            Square::A7,
            Square::H8,
            Square::B5,
        ),
        attack_case(
            Color::Black,
            PieceKind::Knight,
            Square::H2,
            Square::A8,
            Square::F1,
        ),
    ]);
}

#[test]
fn test_white_bishop_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::White,
            PieceKind::Bishop,
            Square::C1,
            Square::H1,
            Square::H6,
        ),
        attack_case(
            Color::White,
            PieceKind::Bishop,
            Square::F4,
            Square::A1,
            Square::B8,
        ),
        attack_case(
            Color::White,
            PieceKind::Bishop,
            Square::G2,
            Square::A1,
            Square::B7,
        ),
    ]);
}

#[test]
fn test_black_bishop_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::Black,
            PieceKind::Bishop,
            Square::C8,
            Square::H8,
            Square::H3,
        ),
        attack_case(
            Color::Black,
            PieceKind::Bishop,
            Square::A7,
            Square::H8,
            Square::D4,
        ),
        attack_case(
            Color::Black,
            PieceKind::Bishop,
            Square::F6,
            Square::A8,
            Square::B2,
        ),
    ]);
}

#[test]
fn test_white_rook_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::White,
            PieceKind::Rook,
            Square::A1,
            Square::H1,
            Square::A8,
        ),
        attack_case(
            Color::White,
            PieceKind::Rook,
            Square::D4,
            Square::A1,
            Square::H4,
        ),
        attack_case(
            Color::White,
            PieceKind::Rook,
            Square::F6,
            Square::A1,
            Square::F2,
        ),
    ]);
}

#[test]
fn test_black_rook_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::Black,
            PieceKind::Rook,
            Square::H8,
            Square::A8,
            Square::H1,
        ),
        attack_case(
            Color::Black,
            PieceKind::Rook,
            Square::E5,
            Square::H8,
            Square::B5,
        ),
        attack_case(
            Color::Black,
            PieceKind::Rook,
            Square::C6,
            Square::H8,
            Square::C2,
        ),
    ]);
}

#[test]
fn test_white_queen_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::White,
            PieceKind::Queen,
            Square::D1,
            Square::H1,
            Square::D8,
        ),
        attack_case(
            Color::White,
            PieceKind::Queen,
            Square::C4,
            Square::A1,
            Square::H4,
        ),
        attack_case(
            Color::White,
            PieceKind::Queen,
            Square::B2,
            Square::H1,
            Square::G7,
        ),
    ]);
}

#[test]
fn test_black_queen_attack_state_examples() {
    assert_attack_states([
        attack_case(
            Color::Black,
            PieceKind::Queen,
            Square::D8,
            Square::H8,
            Square::D1,
        ),
        attack_case(
            Color::Black,
            PieceKind::Queen,
            Square::F5,
            Square::H8,
            Square::A5,
        ),
        attack_case(
            Color::Black,
            PieceKind::Queen,
            Square::G7,
            Square::A8,
            Square::B2,
        ),
    ]);
}

#[test]
fn test_white_king_attack_state_examples() {
    assert_attack_states([
        king_attack_case(Color::White, Square::D4, Square::E5),
        king_attack_case(Color::White, Square::A1, Square::A2),
        king_attack_case(Color::White, Square::H8, Square::G7),
    ]);
}

#[test]
fn test_black_king_attack_state_examples() {
    assert_attack_states([
        king_attack_case(Color::Black, Square::E5, Square::D4),
        king_attack_case(Color::Black, Square::H1, Square::G1),
        king_attack_case(Color::Black, Square::A8, Square::B7),
    ]);
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

    let mut generated = vec![];
    board.generate_moves_for_pawn(&mut generated, Square::D7, Color::White);
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

    let mut generated = vec![];
    board.generate_moves_for_pawn(&mut generated, Square::D7, Color::White);
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

    let mut generated = vec![];
    board.generate_moves_for_pawn(&mut generated, Square::D7, Color::White);
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

    let promotion =
        find_generated_pawn_move(&board, Square::D7, Square::D8, Some(PieceKind::Knight));
    board.make_move(&promotion);

    assert_eq!(board.serialize_to_fen(), "3N4/8/8/8/8/8/8/8");
}

#[test]
fn test_capture_promotion_to_rook() {
    let mut board = Board::new_empty_board();
    place_test_piece(&mut board, Square::D7, PieceKind::Pawn, Color::White);
    place_test_piece(&mut board, Square::C8, PieceKind::Bishop, Color::Black);

    let promotion = find_generated_pawn_move(&board, Square::D7, Square::C8, Some(PieceKind::Rook));
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
