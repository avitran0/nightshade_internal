use crate::{
    interface::engine::EngineInterface,
    library::{Library, constants::ENGINE_LIB},
};

pub struct Engine {
    library: Library,
}

impl Engine {
    pub fn new() -> Option<Self> {
        let library = Library::new(ENGINE_LIB)?;

        Some(Self { library })
    }

    pub fn interface_engine(&self) -> Option<EngineInterface> {
        Some(EngineInterface::new(
            self.library.interface("VEngineClient014")?,
        ))
    }
}
