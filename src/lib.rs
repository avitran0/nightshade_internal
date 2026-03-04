use std::sync::LazyLock;
use utils::{
    log::{self, Logger, LoggerOptions},
    sync::Mutex,
};

use crate::cheat::Cheat;

mod cheat;
mod ctor;
mod gui;
mod hook;
mod interop;
mod library;

pub static CHEAT: LazyLock<Mutex<Option<Cheat>>> = LazyLock::new(|| Mutex::new(None));

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

    let cheat = match Cheat::new() {
        Some(cheat) => cheat,
        None => {
            log::error!("failed to initialize nightshade");
            return;
        }
    };

    *CHEAT.lock() = Some(cheat);
    log::info!("nightshade initialized successfully");
}

pub fn exit() {
    log::info!("unloading nightshade");
    if let Some(cheat) = CHEAT.lock().take() {
        drop(cheat);
    }
}
