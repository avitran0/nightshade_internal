use libc::{c_char, c_void};
use sdl2::sys::SDL_MessageBoxFlags;

use crate::{interop::cstr, library::Library};

pub type ShowSimpleMessageBoxFn = extern "C" fn(u32, *const c_char, *const c_char, *mut c_void);

pub struct SDL {
    library: Library,
    message_box_fn: ShowSimpleMessageBoxFn,
}

impl SDL {
    pub fn new() -> Option<Self> {
        let library = Library::new("libSDL2-2.0.so.0")?;
        let message_box_fn = library.function("SDL_ShowSimpleMessageBox")?.cast();

        Some(Self {
            library,
            message_box_fn,
        })
    }

    pub fn message_box(&self, kind: MessageBoxKind, title: &str, message: &str) {
        (self.message_box_fn)(
            kind.value(),
            cstr(title).as_ptr(),
            cstr(message).as_ptr(),
            std::ptr::null_mut(),
        );
    }
}

pub enum MessageBoxKind {
    Info,
    Warn,
    Error,
}

impl MessageBoxKind {
    fn value(&self) -> u32 {
        match self {
            Self::Info => SDL_MessageBoxFlags::SDL_MESSAGEBOX_INFORMATION as u32,
            Self::Warn => SDL_MessageBoxFlags::SDL_MESSAGEBOX_WARNING as u32,
            Self::Error => SDL_MessageBoxFlags::SDL_MESSAGEBOX_ERROR as u32,
        }
    }
}
