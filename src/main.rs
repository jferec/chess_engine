use eframe::egui::{self, Align2, Color32, FontFamily, FontId, PointerButton, Pos2, Rect, Vec2};
use engine::{Board, Color, Piece, PieceKind, RANK_SIZE};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([920.0, 760.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Board",
        options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);
            Ok(Box::new(ChessApp::default()))
        }),
    )
}

struct ChessApp {
    board_state: UiBoardState,
    active_drag: Option<DragState>,
    move_events: Vec<MoveEvent>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board_state: UiBoardState::from_board(&Board::new()),
            active_drag: None,
            move_events: Vec::new(),
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("controls_panel")
            .resizable(false)
            .exact_width(280.0)
            .frame(egui::Frame::default().fill(Color32::from_rgb(31, 38, 45)))
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Interaction")
                        .size(22.0)
                        .color(Color32::from_rgb(240, 235, 220)),
                );
                ui.add_space(8.0);

                if ui.button("Reset board").clicked() {
                    self.board_state = UiBoardState::from_board(&Board::new());
                    self.active_drag = None;
                    self.move_events.clear();
                }

                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Last move event").color(Color32::from_rgb(190, 198, 208)),
                );
                if let Some(last_event) = self.move_events.last() {
                    ui.monospace(last_event.describe());
                } else {
                    ui.monospace("No moves yet");
                }

                ui.add_space(12.0);
                ui.label(egui::RichText::new("Event log").color(Color32::from_rgb(190, 198, 208)));
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for event in self.move_events.iter().rev() {
                            ui.monospace(event.describe());
                        }
                    });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(24, 30, 36)))
            .show(ctx, |ui| {
                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.heading(
                        egui::RichText::new("Chess UI Sandbox")
                            .size(28.0)
                            .color(Color32::from_rgb(240, 235, 220)),
                    );
                    ui.label(
                        egui::RichText::new("Drag a piece to another square to emit a move event.")
                            .color(Color32::from_rgb(190, 198, 208)),
                    );
                });
                ui.add_space(18.0);

                let available = ui.available_size();
                let board_size = available.x.min(available.y).min(640.0).max(320.0);

                ui.vertical_centered(|ui| {
                    let (response, painter) =
                        ui.allocate_painter(Vec2::splat(board_size), egui::Sense::click_and_drag());
                    let board_rect = response.rect;
                    let tile_size = board_rect.width() / RANK_SIZE as f32;

                    self.handle_drag(ctx, board_rect, tile_size);
                    self.paint_board(&painter, board_rect, tile_size);
                });
            });
    }
}

impl ChessApp {
    fn paint_board(&self, painter: &egui::Painter, board_rect: Rect, tile_size: f32) {
        let hovered_square = painter
            .ctx()
            .pointer_hover_pos()
            .and_then(|pos| square_from_pos(board_rect, tile_size, pos));

        for rank in 0..RANK_SIZE {
            for file in 0..RANK_SIZE {
                let square = Square { file, rank };
                let tile_rect = square_rect(board_rect, tile_size, square);
                let is_light = (file + rank) % 2 == 0;
                let tile_color = if Some(square) == hovered_square {
                    Color32::from_rgb(207, 210, 120)
                } else if is_light {
                    Color32::from_rgb(240, 217, 181)
                } else {
                    Color32::from_rgb(181, 136, 99)
                };

                painter.rect_filled(tile_rect, 0.0, tile_color);
                painter.text(
                    egui::pos2(tile_rect.left() + 6.0, tile_rect.top() + 6.0),
                    Align2::LEFT_TOP,
                    square.label(),
                    FontId::new(12.0, FontFamily::Monospace),
                    Color32::from_rgba_unmultiplied(20, 20, 20, 140),
                );

                let Some(piece) = self.board_state.piece_at(square) else {
                    continue;
                };

                if self
                    .active_drag
                    .as_ref()
                    .is_some_and(|drag| drag.from == square)
                {
                    continue;
                }

                painter.text(
                    tile_rect.center(),
                    Align2::CENTER_CENTER,
                    piece_glyph(piece),
                    FontId::new(tile_size * 0.72, FontFamily::Proportional),
                    piece_color(piece),
                );
            }
        }

        if let Some(drag) = &self.active_drag {
            if let Some(pointer_pos) = painter.ctx().pointer_hover_pos() {
                painter.text(
                    pointer_pos,
                    Align2::CENTER_CENTER,
                    piece_glyph(drag.piece),
                    FontId::new(tile_size * 0.72, FontFamily::Proportional),
                    piece_color(drag.piece),
                );
            }
        }
    }

    fn handle_drag(&mut self, ctx: &egui::Context, board_rect: Rect, tile_size: f32) {
        let pointer_pos = ctx.pointer_interact_pos();

        if ctx.input(|input| input.pointer.button_pressed(PointerButton::Primary))
            && let Some(pos) = pointer_pos
            && let Some(square) = square_from_pos(board_rect, tile_size, pos)
            && let Some(piece) = self.board_state.piece_at(square)
        {
            self.active_drag = Some(DragState {
                from: square,
                piece,
            });
        }

        if ctx.input(|input| input.pointer.button_released(PointerButton::Primary)) {
            if let Some(drag) = self.active_drag.take()
                && let Some(pos) = pointer_pos
                && let Some(to) = square_from_pos(board_rect, tile_size, pos)
            {
                self.board_state.move_piece(drag.from, to);
                self.move_events.push(MoveEvent {
                    piece: drag.piece,
                    from: drag.from,
                    to,
                });
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Square {
    file: usize,
    rank: usize,
}

impl Square {
    fn label(self) -> String {
        let file = (b'a' + self.file as u8) as char;
        let rank = (b'1' + self.rank as u8) as char;
        format!("{file}{rank}")
    }
}

#[derive(Clone, Copy, Debug)]
struct DragState {
    from: Square,
    piece: Piece,
}

#[derive(Clone, Copy, Debug)]
struct MoveEvent {
    piece: Piece,
    from: Square,
    to: Square,
}

impl MoveEvent {
    fn describe(self) -> String {
        format!(
            "{} {} -> {}",
            piece_name(self.piece),
            self.from.label(),
            self.to.label()
        )
    }
}

#[derive(Clone)]
struct UiBoardState {
    squares: [[Option<Piece>; RANK_SIZE]; RANK_SIZE],
}

impl UiBoardState {
    fn from_board(board: &Board) -> Self {
        let mut squares = [[None; RANK_SIZE]; RANK_SIZE];

        for (rank, row) in squares.iter_mut().enumerate() {
            for (file, square) in row.iter_mut().enumerate() {
                *square = board.piece_at(file, rank);
            }
        }

        Self { squares }
    }

    fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square.rank][square.file]
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.squares[from.rank][from.file].take();
        self.squares[to.rank][to.file] = piece;
    }
}

fn square_rect(board_rect: Rect, tile_size: f32, square: Square) -> Rect {
    let x = board_rect.left() + square.file as f32 * tile_size;
    let y = board_rect.top() + (RANK_SIZE - 1 - square.rank) as f32 * tile_size;
    Rect::from_min_size(egui::pos2(x, y), Vec2::splat(tile_size))
}

fn square_from_pos(board_rect: Rect, tile_size: f32, pos: Pos2) -> Option<Square> {
    if !board_rect.contains(pos) {
        return None;
    }

    let file = ((pos.x - board_rect.left()) / tile_size).floor() as usize;
    let row_from_top = ((pos.y - board_rect.top()) / tile_size).floor() as usize;
    let rank = RANK_SIZE.checked_sub(row_from_top + 1)?;

    if file < RANK_SIZE && rank < RANK_SIZE {
        Some(Square { file, rank })
    } else {
        None
    }
}

fn piece_glyph(piece: Piece) -> &'static str {
    match (piece.color, piece.kind) {
        (Color::White, PieceKind::King) => "♔",
        (Color::White, PieceKind::Queen) => "♕",
        (Color::White, PieceKind::Rook) => "♖",
        (Color::White, PieceKind::Bishop) => "♗",
        (Color::White, PieceKind::Knight) => "♘",
        (Color::White, PieceKind::Pawn) => "♙",
        (Color::Black, PieceKind::King) => "♚",
        (Color::Black, PieceKind::Queen) => "♛",
        (Color::Black, PieceKind::Rook) => "♜",
        (Color::Black, PieceKind::Bishop) => "♝",
        (Color::Black, PieceKind::Knight) => "♞",
        (Color::Black, PieceKind::Pawn) => "♟",
    }
}

fn piece_name(piece: Piece) -> &'static str {
    match piece.kind {
        PieceKind::King => "king",
        PieceKind::Queen => "queen",
        PieceKind::Rook => "rook",
        PieceKind::Bishop => "bishop",
        PieceKind::Knight => "knight",
        PieceKind::Pawn => "pawn",
    }
}

fn piece_color(piece: Piece) -> Color32 {
    match piece.color {
        Color::White => Color32::from_rgb(250, 250, 250),
        Color::Black => Color32::from_rgb(24, 24, 24),
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(28.0, FontFamily::Proportional),
    );
    ctx.set_style(style);
}
