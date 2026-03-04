use libc::{c_char, c_int, c_void};
use sdl2::sys::SDL_MessageBoxFlags;

use crate::{interop::cstr, library::Library};

/// void SDL_GL_SwapWindow(SDL_Window *)
type GlSwapFn = extern "C" fn(*mut c_void);
type GLGetProcAddressFn = extern "C" fn(*const c_char) -> *mut c_void;
/// int SDL_PollEvent(SDL_Event *)
type PollEventFn = extern "C" fn(*mut c_void) -> c_int;
/// int SDL_ShowSimpleMessageBox(Uint32 flags, const char *title, const char *message, SDL_Window *window)
type ShowSimpleMessageBoxFn = extern "C" fn(u32, *const c_char, *const c_char, *mut c_void);

pub struct SDL {
    library: Library,
    gl_swap_fn: GlSwapFn,
    gl_get_proc_address_fn: GLGetProcAddressFn,
    message_box_fn: ShowSimpleMessageBoxFn,
}

impl SDL {
    pub fn new() -> Option<Self> {
        let library = Library::new("libSDL2-2.0.so.0")?;
        let gl_swap_fn = library.function("SDL_GL_SwapWindow")?.cast();
        let gl_get_proc_address_fn = library.function("SDL_GL_GetProcAddress")?.cast();
        let message_box_fn = library.function("SDL_ShowSimpleMessageBox")?.cast();

        Some(Self {
            library,
            gl_get_proc_address_fn,
            gl_swap_fn,
            message_box_fn,
        })
    }

    pub fn gl(&self) -> glow::Context {
        unsafe {
            glow::Context::from_loader_function_cstr(|s| (self.gl_get_proc_address_fn)(s.as_ptr()))
        }
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
