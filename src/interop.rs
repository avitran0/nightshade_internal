use std::ffi::{CStr, CString};

pub fn cstr(string: &str) -> CString {
    CString::new(string).unwrap()
}

pub fn str(string: *const i8) -> &'static str {
    unsafe { CStr::from_ptr(string) }.to_str().unwrap()
}
