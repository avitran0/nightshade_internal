use libc::{c_char, c_void};

pub type InterfaceRegisterFn = extern "C" fn() -> *const c_void;

#[repr(C)]
pub struct InterfaceRegistration {
    pub register_fn: InterfaceRegisterFn,
    pub name: *const c_char,
    pub next: *const InterfaceRegistration,
}

pub struct Interface {
    handle: *const c_void,
}

impl Interface {
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
}
