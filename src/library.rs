use std::ffi::CString;

use utils::log;

pub struct Symbol {
    ptr: *mut libc::c_void,
}

impl Symbol {
    pub fn cast<T>(self) -> T {
        unsafe { std::mem::transmute_copy(&self.ptr) }
    }
}

pub struct Library {
    handle: *mut libc::c_void,
}

impl Library {
    pub fn new(name: &str) -> Option<Self> {
        let handle = unsafe {
            libc::dlopen(
                CString::new(name).unwrap().as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_NOLOAD,
            )
        };
        if handle.is_null() {
            log::error!("failed to find {name}");
            None
        } else {
            Some(Self { handle })
        }
    }

    pub fn function(&self, name: &str) -> Option<Symbol> {
        let func = unsafe { libc::dlsym(self.handle, CString::new(name).unwrap().as_ptr()) };
        if func.is_null() {
            None
        } else {
            Some(Symbol { ptr: func })
        }
    }

    pub fn address(&self) -> usize {
        self.handle as usize
    }
}
