use utils::log::{self, Logger, LoggerOptions};

use crate::library::sdl::{MessageBoxKind, SDL};

mod ctor;
mod interop;
mod library;

pub fn init() {
    Logger::install(
        LoggerOptions::default()
            .debug(true)
            .module(module_path!())
            .stdout(true)
            .file("/tmp/nightshade.log")
            .truncate(true),
    );
    log::info!("loading nightshade");
    let sdl = SDL::new().unwrap();
    sdl.message_box(
        MessageBoxKind::Info,
        "Title",
        "This is a very captivating message",
    );
}

pub fn exit() {}
