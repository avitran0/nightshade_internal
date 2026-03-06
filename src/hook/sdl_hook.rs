use libc::c_void;

pub struct SdlHook {
    pub jump_target: *mut usize,
    pub proxy: usize,
}

impl SdlHook {
    pub fn new(original: usize, hook: usize) -> Option<Self> {
        let first_byte = unsafe { *(original as *const u8) };
        let second_byte = unsafe { *((original + 1) as *const u8) };

        // Check if the instruction is `jmp QWORD PTR [rip + offset]` (FF 25)
        // Steam overlay uses this instruction to hook SDL_GL_SwapWindow.
        if first_byte == 0xFF && second_byte == 0x25 {
            let offset = unsafe { *((original + 2) as *const i32) };
            let next_instruction = original + 6;

            let target_addr = (next_instruction as isize + offset as isize) as usize;
            let jump_target = target_addr as *mut usize;

            let proxy = unsafe { *jump_target };

            // We mimic the C++ hook and avoid using `mprotect`. The jump target is typically
            // a GOT entry or writable memory region. Using `mprotect` to restore to READ|EXEC
            // accidentally removes WRITE permissions from the GOT page, causing lazy-binding
            // segfaults and crashing the game.
            unsafe {
                *jump_target = hook;
            }

            Some(Self { jump_target, proxy })
        } else {
            // Steam overlay is either disabled or uses a different hook instruction.
            println!("nightshade: Error: SDL_GL_SwapWindow is not hooked by Steam Overlay (starts with {:02X} {:02X}). Ensure Steam Overlay is enabled.", first_byte, second_byte);
            None
        }
    }
}

impl Drop for SdlHook {
    fn drop(&mut self) {
        unsafe {
            // Restore proxy without modifying page permissions.
            *self.jump_target = self.proxy;
        }
    }
}
