use utils::log::{self, Logger, LoggerOptions};

use crate::library::Library;

mod ctor;
mod library;
mod sdl;

pub fn init() {
    Logger::install(
        LoggerOptions::default()
            .debug(true)
            .module(module_path!())
            .stdout(true)
            .file("nightshade.log"),
    );
    let sdl = Library::new("libSDL2-2.0.so.0").unwrap();
    log::info!("sdl: 0x{:X}", sdl.address());
}

pub fn exit() {}
