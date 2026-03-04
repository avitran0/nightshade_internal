use crate::library::{Library, constants::CLIENT_LIB};

use libc::c_void;

type CreateInterfaceFn = extern "C" fn() -> *mut c_void;

pub struct Client {
    library: Library,
    create_interface_fn: CreateInterfaceFn,
}

impl Client {
    pub fn new() -> Option<Self> {
        let library = Library::new(CLIENT_LIB)?;
        let create_interface_fn = library.function("CreateInterface")?.cast();

        Some(Self {
            library,
            create_interface_fn,
        })
    }
}
