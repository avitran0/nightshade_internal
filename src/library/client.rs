use crate::library::Library;

use libc::c_void;

type CreateInterfaceFn = extern "C" fn() -> *mut c_void;

pub struct Client {
    library: Library,
    create_interface_fn: CreateInterfaceFn,
}