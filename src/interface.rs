use libc::c_char;

pub type InterfaceRegisterFn = extern "C" fn();

#[repr(C)]
pub struct InterfaceRegistration {
    pub register_fn: InterfaceRegisterFn,
    pub name: *const c_char,
    pub next: *const InterfaceRegistration,
}

pub struct Interface {
    handle: usize,
}
