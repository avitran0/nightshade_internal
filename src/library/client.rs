use crate::library::{Library, constants::CLIENT_LIB};

use libc::c_void;

type CreateInterfaceFn = extern "C" fn() -> *mut c_void;

pub struct Client {
    pub library: Library,
    create_interface_fn: CreateInterfaceFn,
}

impl Client {
    pub fn new() -> Option<Self> {
        let library = Library::new(CLIENT_LIB)?;
        let create_interface_fn = library.symbol("CreateInterface")?.cast();

        Some(Self {
            library,
            create_interface_fn,
        })
    }
}
