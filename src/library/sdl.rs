use libc::{c_char, c_int, c_void};

use crate::{
    interop::cstr,
    library::{constants::SDL_LIB, Library},
};

/// void SDL_GL_SwapWindow(SDL_Window *)
type GlSwapFn = extern "C" fn(*mut c_void);
type GLGetProcAddressFn = extern "C" fn(*const c_char) -> *mut c_void;
/// int SDL_PollEvent(SDL_Event *)
type PollEventFn = extern "C" fn(*mut c_void) -> c_int;
/// int SDL_ShowSimpleMessageBox(Uint32 flags, const char *title, const char *message, SDL_Window *window)
type ShowSimpleMessageBoxFn = extern "C" fn(u32, *const c_char, *const c_char, *mut c_void);

#[allow(dead_code)]
pub struct Sdl {
    library: Library,
    gl_swap_fn: GlSwapFn,
    gl_get_proc_address_fn: GLGetProcAddressFn,
    poll_event_fn: PollEventFn,
    message_box_fn: ShowSimpleMessageBoxFn,
}

impl Sdl {
    pub fn new() -> Option<Self> {
        let library = Library::new(SDL_LIB)?;
        let gl_swap_fn = library.symbol("SDL_GL_SwapWindow")?.cast();
        let gl_get_proc_address_fn = library.symbol("SDL_GL_GetProcAddress")?.cast();
        let poll_event_fn = library.symbol("SDL_PollEvent")?.cast();
        let message_box_fn = library.symbol("SDL_ShowSimpleMessageBox")?.cast();

        Some(Self {
            library,
            gl_get_proc_address_fn,
            gl_swap_fn,
            poll_event_fn,
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

    pub fn gl_swap_window_ptr(&self) -> usize {
        self.gl_swap_fn as usize
    }
}

pub enum MessageBoxKind {
    Info,
    Warn,
    Error,
}

impl MessageBoxKind {
    // https://wiki.libsdl.org/SDL2/SDL_MessageBoxFlags
    fn value(&self) -> u32 {
        match self {
            Self::Info => 0x00000040,
            Self::Warn => 0x00000020,
            Self::Error => 0x00000010,
        }
    }
}
