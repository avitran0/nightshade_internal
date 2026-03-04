use crate::{exit, init};

#[allow(dead_code)]
#[unsafe(link_section = ".init_array")]
static INIT: extern "C" fn() = constructor;

#[unsafe(no_mangle)]
pub extern "C" fn constructor() {
    init();
}

#[allow(dead_code)]
#[unsafe(link_section = ".fini_array")]
static EXIT: extern "C" fn() = destructor;

#[unsafe(no_mangle)]
pub extern "C" fn destructor() {
    exit();
}
