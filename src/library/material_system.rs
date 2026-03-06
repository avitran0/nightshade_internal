use crate::{
    interface::Interface,
    library::{Library, constants::MATERIAL_SYSTEM_LIB},
};

pub struct MaterialSystem {
    library: Library,
}

impl MaterialSystem {
    pub fn new() -> Option<Self> {
        let library = Library::new(MATERIAL_SYSTEM_LIB)?;

        Some(Self { library })
    }

    pub fn interface_material_system(&self) -> Option<Interface> {
        self.library.interface("VMaterialSystem080")
    }
}
