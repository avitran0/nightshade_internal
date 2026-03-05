use crate::{
    interface::Interface,
    library::{Library, constants::CLIENT_LIB},
};

pub struct Client {
    library: Library,
}

impl Client {
    pub fn new() -> Option<Self> {
        let library = Library::new(CLIENT_LIB)?;

        Some(Self { library })
    }

    pub fn interface_client(&self) -> Option<Interface> {
        self.library.interface("VClient018")
    }

    pub fn interface_entity_list(&self) -> Option<Interface> {
        self.library.interface("VClientEntityList003")
    }
}
