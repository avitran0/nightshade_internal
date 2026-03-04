use std::ffi::CString;

use utils::log;

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

    pub fn address(&self) -> usize {
        self.handle as usize
    }
}
