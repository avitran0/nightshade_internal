use libc::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{hook::sdl_hook::SdlHook, interface::Interface, library::Libraries};

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
        let addr = (address & !(page_size - 1)) as *mut libc::c_void;
        let len = page_size;
        let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

        unsafe {
            libc::mprotect(addr, len, prot);
            *(address as *mut usize) = new_function;
            libc::mprotect(addr, len, libc::PROT_READ | libc::PROT_EXEC);
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
        let addr = (self.address & !(page_size - 1)) as *mut libc::c_void;
        let len = page_size;
        let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

        unsafe {
            libc::mprotect(addr, len, prot);
            *(self.address as *mut usize) = self.original_function;
            libc::mprotect(addr, len, libc::PROT_READ | libc::PROT_EXEC);
        }
    }
}

pub struct Hooks {
    pub sdl_gl_swap_window: SdlHook,
    pub frame_stage_notify: Hook,
}

static ORIGINAL_FRAME_STAGE_NOTIFY: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_SDL_SWAP_WINDOW: AtomicUsize = AtomicUsize::new(0);

impl Hooks {
    pub fn hook(libraries: &Libraries) -> Option<Self> {
        let frame_stage_notify = Self::hook_vtable(
            &libraries.client().interface_client()?,
            37,
            frame_stage_notify_hook as *const () as usize,
            &ORIGINAL_FRAME_STAGE_NOTIFY,
        )?;
        let sdl_gl_swap_window = Self::hook_sdl(
            libraries.sdl().gl_swap_window_ptr(),
            sdl_swap_window_hook as *const () as usize,
            &ORIGINAL_SDL_SWAP_WINDOW,
        );

        Some(Self {
            sdl_gl_swap_window,
            frame_stage_notify,
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
    if let Some(mut cheat_guard) = crate::CHEAT.try_lock() {
        if let Some(cheat) = cheat_guard.as_mut() {
            if let Some(stage) = ClientFrameStage::from_i32(client_frame_stage) {
                cheat.frame_stage_notify(stage);
            }
        }
    }

    let original_ptr = ORIGINAL_FRAME_STAGE_NOTIFY.load(Ordering::Relaxed);
    if original_ptr != 0 {
        let original_fn: FrameStageNotifyFn = unsafe { std::mem::transmute(original_ptr) };
        original_fn(this, client_frame_stage);
    }
}

type SdlSwapWindowFn = extern "C" fn(*mut c_void);
extern "C" fn sdl_swap_window_hook(window: *mut c_void) {
    if let Some(mut cheat_guard) = crate::CHEAT.try_lock() {
        if let Some(cheat) = cheat_guard.as_mut() {
            cheat.gl_swap_buffers();
        }
    }

    let original_ptr = ORIGINAL_SDL_SWAP_WINDOW.load(Ordering::Relaxed);
    if original_ptr != 0 {
        let original_fn: SdlSwapWindowFn = unsafe { std::mem::transmute(original_ptr) };
        original_fn(window);
    }
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
