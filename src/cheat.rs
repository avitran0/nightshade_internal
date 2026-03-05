use std::sync::Arc;

use crate::{
    gui::Gui,
    library::{Libraries, sdl::MessageBoxKind},
};

pub struct Cheat {
    gui: Gui,
    gl: Arc<glow::Context>,
    libraries: Libraries,
}

impl Cheat {
    pub fn new() -> Option<Self> {
        let libraries = Libraries::new()?;
        let gl = Arc::new(libraries.sdl().gl());
        let gui = Gui::new(gl.clone());

        libraries.sdl().message_box(
            MessageBoxKind::Info,
            "nightshade",
            "initialized successfully",
        );

        Some(Self { gui, libraries, gl })
    }
}

// just single-threaded access, i hope rust does not rip my head off
unsafe impl Send for Cheat {}
unsafe impl Sync for Cheat {}
