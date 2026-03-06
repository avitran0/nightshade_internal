use std::sync::Arc;

use egui::RawInput;
use utils::log;

use crate::{
    gui::Gui,
    hook::{ClientFrameStage, Hooks},
    library::{Libraries, sdl::MessageBoxKind},
};

pub struct Cheat {
    gui: Gui,
    gl: Arc<glow::Context>,
    libraries: Libraries,
    pub hooks: Hooks,
}

impl Cheat {
    pub fn new() -> Option<Self> {
        let libraries = Libraries::new()?;
        let gl = Arc::new(libraries.sdl().gl());
        let gui = Gui::new(gl.clone());
        let hooks = Hooks::hook(&libraries)?;

        log::info!("{:?}", libraries.engine().interface_engine()?.screen_size());

        libraries.sdl().message_box(
            MessageBoxKind::Info,
            "nightshade",
            "initialized successfully",
        );

        Some(Self {
            gui,
            libraries,
            gl,
            hooks,
        })
    }

    pub fn frame_stage_notify(&mut self, _stage: ClientFrameStage) {}

    pub fn gl_swap_buffers(&mut self) {
        let Some(engine_interface) = self.libraries.engine().interface_engine() else {
            return;
        };
        let (width, height) = engine_interface.screen_size();

        let input = RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32),
            )),
            ..Default::default()
        };

        self.gui.start_frame(input);
        self.gui
            .draw_text(egui::pos2(50.0, 50.0), "text here", egui::Color32::WHITE);
        self.gui.end_frame([width as u32, height as u32]);
    }
}

// just single-threaded access, i hope rust does not rip my head off
unsafe impl Send for Cheat {}
unsafe impl Sync for Cheat {}
