use std::sync::Arc;

use utils::log;

use crate::{
    gui::Gui,
    hook::Hooks,
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
}

// just single-threaded access, i hope rust does not rip my head off
unsafe impl Send for Cheat {}
unsafe impl Sync for Cheat {}
