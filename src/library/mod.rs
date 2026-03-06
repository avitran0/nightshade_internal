use utils::log;

use crate::{
    interface::{Interface, InterfaceRegistration},
    interop::{cstr, str},
    library::{client::Client, engine::Engine, material_system::MaterialSystem, sdl::Sdl},
};

pub mod client;
pub mod constants;
pub mod engine;
pub mod material_system;
pub mod sdl;

pub struct Symbol {
    pub ptr: *mut libc::c_void,
}

impl Symbol {
    pub fn cast<T>(&self) -> T {
        unsafe { std::mem::transmute_copy(&self.ptr) }
    }
}

pub struct Library {
    handle: *mut libc::c_void,
}

impl Library {
    pub fn new(name: &str) -> Option<Self> {
        let handle =
            unsafe { libc::dlopen(cstr(name).as_ptr(), libc::RTLD_LAZY | libc::RTLD_NOLOAD) };
        if handle.is_null() {
            log::error!("failed to find {name}");
            None
        } else {
            Some(Self { handle })
        }
    }

    pub fn symbol(&self, name: &str) -> Option<Symbol> {
        let func = unsafe { libc::dlsym(self.handle, cstr(name).as_ptr()) };
        if func.is_null() {
            None
        } else {
            Some(Symbol { ptr: func })
        }
    }

    pub fn interface(&self, name: &str) -> Option<Interface> {
        let interface_reg_ptr: *const *const InterfaceRegistration =
            self.symbol("s_pInterfaceRegs")?.cast();
        let mut interface_reg = unsafe { *interface_reg_ptr };

        while !interface_reg.is_null() {
            let cur = unsafe { &*interface_reg };
            let interface_name = str(cur.name);

            // log::info!("interface: {interface_name}");
            if interface_name == name {
                let interface = (cur.register_fn)();
                log::info!("found interface {name} at {interface:?}");
                return Some(Interface::new(interface));
            }

            interface_reg = cur.next;
        }

        log::error!("failed to find interface {name}");
        None
    }

    pub fn address(&self) -> usize {
        self.handle as usize
    }
}

pub struct Libraries {
    sdl: Sdl,
    client: Client,
    engine: Engine,
    material_system: MaterialSystem,
}

impl Libraries {
    pub fn new() -> Option<Self> {
        let sdl = Sdl::new()?;
        let client = Client::new()?;
        let engine = Engine::new()?;
        let material_system = MaterialSystem::new()?;

        Some(Self {
            sdl,
            client,
            engine,
            material_system,
        })
    }

    pub fn sdl(&self) -> &Sdl {
        &self.sdl
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn material_system(&self) -> &MaterialSystem {
        &self.material_system
    }
}
