use crate::{interface::Interface, library::{Library, constants::CLIENT_LIB}};

use libc::c_void;

type CreateInterfaceFn = extern "C" fn() -> *mut c_void;

pub struct Client {
    library: Library,
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

    pub fn interface_client(&self) -> Option<Interface> {
        self.library.interface("VClient018")
    }

    pub fn interface_entity_list(&self) -> Option<Interface> {
        self.library.interface("VClientEntityList003")
    }
}
