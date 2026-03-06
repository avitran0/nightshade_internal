use libc::{c_char, c_void};

pub mod engine;

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

    pub fn handle(&self) -> *const c_void {
        self.handle
    }

    pub fn vtable(&self) -> *const *const c_void {
        unsafe { *(self.handle as *const *const *const c_void) }
    }

    pub fn vfunc_raw(&self, index: usize) -> *const c_void {
        unsafe { *self.vtable().add(index) }
    }

    pub fn vfunc<T>(&self, index: usize) -> T {
        unsafe { std::mem::transmute_copy(&*self.vtable().add(index)) }
    }
}
