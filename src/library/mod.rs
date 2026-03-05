use utils::log;

use crate::{
    interface::{Interface, InterfaceRegistration},
    interop::{cstr, str},
    library::{client::Client, sdl::SDL},
};

pub mod client;
mod constants;
pub mod sdl;

pub struct Symbol {
    ptr: *mut libc::c_void,
}

impl Symbol {
    pub fn cast<T>(self) -> T {
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

            log::info!("interface: {name}");
            if interface_name == name {
                
            }
            // if matches, cur.create_fn() as Interface *

            interface_reg = cur.next;
        }

        None
    }

    pub fn address(&self) -> usize {
        self.handle as usize
    }
}

pub struct Libraries {
    sdl: SDL,
    client: Client,
}

impl Libraries {
    pub fn new() -> Option<Self> {
        let sdl = SDL::new()?;
        let client = Client::new()?;

        Some(Self { sdl, client })
    }

    pub fn sdl(&self) -> &SDL {
        &self.sdl
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
