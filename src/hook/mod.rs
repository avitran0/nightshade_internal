use libc::c_void;
use utils::log;

use crate::library::{Libraries, Symbol};

pub mod pattern;

pub struct Hook {
    pub address: usize,
    pub original_function: Symbol,
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
            original_function: Symbol {
                ptr: original_function as *mut libc::c_void,
            },
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
            *(self.address as *mut usize) = self.original_function.ptr as usize;
            libc::mprotect(addr, len, libc::PROT_READ | libc::PROT_EXEC);
        }
    }
}

pub struct Hooks {
    pub frame_stage_notify: Hook,
}

impl Hooks {
    pub fn hook(libraries: &Libraries) -> Option<Self> {
        let frame_stage_notify = Hook::hook_fn(
            libraries.client().interface_client()?.vfunc(37),
            frame_stage_notify_hook as *const () as usize,
        );

        Some(Self { frame_stage_notify })
    }
}

type FrameStageNotifyFn = extern "C" fn(*const c_void, i32);
extern "C" fn frame_stage_notify_hook(this: *const c_void, client_frame_stage: i32) {
    log::info!("frame stage: {client_frame_stage}");

    if let Some(cheat) = crate::CHEAT.lock().as_ref() {
        let original_fn: FrameStageNotifyFn =
            cheat.hooks.frame_stage_notify.original_function.cast();
        original_fn(this, client_frame_stage);
    }
}

enum ClientFrameStage {
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
