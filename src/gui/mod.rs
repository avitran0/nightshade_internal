use std::sync::Arc;

use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Ui};

use crate::library::sdl_event::SdlEvent;

pub struct Gui {
    pub ctx: egui::Context,
    pub painter: egui_glow::Painter,
    pub input: egui::RawInput,
}

impl Gui {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        let ctx = egui::Context::default();
        let painter = egui_glow::Painter::new(gl, "", None, true).unwrap();
        let input = egui::RawInput::default();

        Self {
            ctx,
            painter,
            input,
        }
    }

    pub fn add_event(&mut self, event: &SdlEvent) {
        if let Some(ev) = event.egui() {
            self.input.events.push(ev);
        }
    }

    pub fn start_frame(&mut self, screen_rect: egui::Rect) {
        let input = egui::RawInput {
            screen_rect: Some(screen_rect),
            ..self.input.clone()
        };
        self.ctx.begin_pass(input);
    }

    pub fn end_frame(&mut self, screen_size: [u32; 2]) {
        let output = self.ctx.end_pass();
        // todo: handle platform output (cursors, etc)
        let clipped_primitives = self.ctx.tessellate(output.shapes, output.pixels_per_point);
        self.painter.paint_and_update_textures(
            screen_size,
            output.pixels_per_point,
            &clipped_primitives,
            &output.textures_delta,
        );
    }

    pub fn screen_painter(&self) -> egui::Painter {
        self.ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("gui_painter"),
        ))
    }

    #[allow(dead_code)]
    pub fn draw_line(&self, points: [Pos2; 2], stroke: Stroke) {
        self.screen_painter().line_segment(points, stroke);
    }

    pub fn draw_text(&self, pos: Pos2, text: &str, color: Color32) {
        self.screen_painter().text(
            pos,
            egui::Align2::LEFT_TOP,
            text,
            egui::FontId::proportional(24.0),
            color,
        );
    }

    #[allow(dead_code)]
    pub fn draw_rect(&self, rect: Rect, stroke: Stroke, fill: Color32) {
        self.screen_painter()
            .rect(rect, 0.0, fill, stroke, StrokeKind::Inside);
    }

    pub fn window(&self, title: &str, func: impl FnOnce(&mut Ui)) {
        egui::Window::new(title).show(&self.ctx, func);
    }
}
