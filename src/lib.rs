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
mod interface;
mod interop;
mod library;

#[cfg(not(target_os = "linux"))]
compile_error!("only linux is supported.");

pub static CHEAT: LazyLock<Mutex<Option<Cheat>>> = LazyLock::new(|| Mutex::new(None));

pub fn init() {
    Logger::install(
        LoggerOptions::default()
            .file("/tmp/nightshade.log")
            .debug(true)
            .truncate(true)
            .module(module_path!()),
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
