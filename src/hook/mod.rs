use libc::{c_int, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};
use utils::log;

use crate::{
    CHEAT,
    hook::sdl_hook::SdlHook,
    interface::Interface,
    library::{
        Libraries,
        sdl::{GlSwapFn, PollEventFn},
        sdl_event::SdlEvent,
    },
};

pub mod pattern;
pub mod sdl_hook;

pub struct Hook {
    pub address: usize,
    pub original_function: usize,
}

impl Hook {
    pub fn hook_fn(address: usize, new_function: usize) -> Self {
        let original_function = unsafe { *(address as *const usize) };
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let start_page = address & !(page_size - 1);
        let end_page = (address + std::mem::size_of::<usize>() - 1) & !(page_size - 1);
        let len = end_page - start_page + page_size;
        let addr = start_page as *mut libc::c_void;
        let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

        unsafe {
            libc::mprotect(addr, len, prot);
            *(address as *mut usize) = new_function;
            // NOTE: We intentionally do NOT restore memory protections (e.g., to PROT_READ)
            // If the hooked function resides in a writable section (like .data, partial RELRO GOT,
            // or dynamically allocated memory), forcing it to read-only will cause the game to
            // segfault when it or the dynamic linker attempts to write to it later.
            // Leaving it as PROT_READ | PROT_WRITE | PROT_EXEC is safer.
        }

        Self {
            address,
            original_function,
        }
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let start_page = self.address & !(page_size - 1);
        let end_page = (self.address + std::mem::size_of::<usize>() - 1) & !(page_size - 1);
        let len = end_page - start_page + page_size;
        let addr = start_page as *mut libc::c_void;
        let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

        unsafe {
            libc::mprotect(addr, len, prot);
            *(self.address as *mut usize) = self.original_function;
            // Again, do not restore to PROT_READ | PROT_EXEC to avoid stripping PROT_WRITE
            // from sections that originally had it.
        }
    }
}

pub struct Hooks {
    pub frame_stage_notify: Hook,
    pub sdl_gl_swap_window: SdlHook,
    pub sdl_poll_event: SdlHook,
}

static ORIGINAL_FRAME_STAGE_NOTIFY: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_SDL_SWAP_WINDOW: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_SDL_POLL_EVENT: AtomicUsize = AtomicUsize::new(0);

impl Hooks {
    pub fn hook(libraries: &Libraries) -> Option<Self> {
        let frame_stage_notify = Self::hook_vtable(
            &libraries.client().interface_client()?,
            37,
            frame_stage_notify_hook as *const () as usize,
            &ORIGINAL_FRAME_STAGE_NOTIFY,
        )?;
        log::info!(
            "hooked frame_stage_notify at 0x{:X}",
            frame_stage_notify.address
        );
        let sdl_gl_swap_window = Self::hook_sdl(
            libraries.sdl().gl_swap_window_ptr(),
            sdl_swap_window_hook as *const () as usize,
            &ORIGINAL_SDL_SWAP_WINDOW,
        );
        log::info!(
            "hooked sdl_gl_swapwindow at 0x{:X}",
            sdl_gl_swap_window.proxy
        );
        let sdl_poll_event = Self::hook_sdl(
            libraries.sdl().poll_event_ptr(),
            sdl_poll_event_hook as *const () as usize,
            &ORIGINAL_SDL_POLL_EVENT,
        );

        Some(Self {
            frame_stage_notify,
            sdl_gl_swap_window,
            sdl_poll_event,
        })
    }

    fn hook_vtable(
        interface: &Interface,
        vtable_entry: usize,
        new_function: usize,
        atomic: &AtomicUsize,
    ) -> Option<Hook> {
        let vtable_entry = unsafe { interface.vtable().add(vtable_entry) as usize };
        let original_ptr = unsafe { *(vtable_entry as *const usize) };
        atomic.store(original_ptr, Ordering::Relaxed);

        let hook = Hook::hook_fn(vtable_entry, new_function);
        Some(hook)
    }

    fn hook_sdl(function: usize, new_function: usize, atomic: &AtomicUsize) -> SdlHook {
        let hook = SdlHook::new(function, new_function);
        atomic.store(hook.proxy, Ordering::Relaxed);
        hook
    }
}
type FrameStageNotifyFn = extern "C" fn(*const c_void, i32);
extern "C" fn frame_stage_notify_hook(this: *const c_void, client_frame_stage: i32) {
    if let Some(cheat) = CHEAT.lock().as_mut()
        && let Some(stage) = ClientFrameStage::from_i32(client_frame_stage)
    {
        cheat.frame_stage_notify(stage);
    }

    let original_ptr = ORIGINAL_FRAME_STAGE_NOTIFY.load(Ordering::Relaxed);
    if original_ptr != 0 {
        let original_fn: FrameStageNotifyFn = unsafe { std::mem::transmute(original_ptr) };
        original_fn(this, client_frame_stage);
    }
}

extern "C" fn sdl_swap_window_hook(window: *mut c_void) {
    if let Some(cheat) = CHEAT.lock().as_mut() {
        cheat.gl_swap_buffers();
    }

    let original_ptr = ORIGINAL_SDL_SWAP_WINDOW.load(Ordering::Relaxed);
    if original_ptr != 0 {
        let original_fn: GlSwapFn = unsafe { std::mem::transmute(original_ptr) };
        original_fn(window);
    }
}

extern "C" fn sdl_poll_event_hook(event: *mut SdlEvent) -> c_int {
    let original_ptr = ORIGINAL_SDL_SWAP_WINDOW.load(Ordering::Relaxed);
    let value = if original_ptr != 0 {
        let original_fn: PollEventFn = unsafe { std::mem::transmute(original_ptr) };
        original_fn(event)
    } else {
        0
    };

    if let Some(cheat) = CHEAT.lock().as_mut() {
        let event = unsafe { &*event };
        cheat.poll_event(event);
    }

    value
}

pub enum ClientFrameStage {
    Undefined = -1,
    Start,
    NetUpdateStart,
    NetUpdatePostDataUpdateStart,
    NetUpdatePostDataUpdateEnd,
    NetUpdateEnd,
    RenderStart,
    RenderEnd,
}

impl ClientFrameStage {
    pub fn from_i32(value: i32) -> Option<Self> {
        Some(match value {
            -1 => Self::Undefined,
            0 => Self::Start,
            1 => Self::NetUpdateStart,
            2 => Self::NetUpdatePostDataUpdateStart,
            3 => Self::NetUpdatePostDataUpdateEnd,
            4 => Self::NetUpdateEnd,
            5 => Self::RenderStart,
            6 => Self::RenderEnd,
            _ => return None,
        })
    }
}
