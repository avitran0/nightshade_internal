use std::sync::{Arc, OnceLock};

use utils::sync::Mutex;

static GUI: OnceLock<Mutex<Gui>> = OnceLock::new();

struct Gui {
    ctx: egui::Context,
    painter: egui_glow::Painter,
}

impl Gui {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        let ctx = egui::Context::default();
        let painter = egui_glow::Painter::new(gl, "", None, true).unwrap();

        Self { ctx, painter }
    }

    pub fn start_frame(&mut self) {
        
    }
}
