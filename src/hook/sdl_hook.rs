use libc::c_void;

pub struct SdlHook {
    pub jump_target: *mut usize,
    pub proxy: usize,
}

impl SdlHook {
    pub fn new(original: usize, hook: usize) -> Self {
        let offset = unsafe { *((original + 2) as *const i32) };
        let next_instruction = original + 6;

        let target_addr = (next_instruction as isize + offset as isize) as usize;
        let jump_target = target_addr as *mut usize;

        let proxy = unsafe { *jump_target };

        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let addr = (target_addr & !(page_size - 1)) as *mut c_void;
        let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

        unsafe {
            libc::mprotect(addr, page_size, prot);
            *jump_target = hook;
            libc::mprotect(addr, page_size, libc::PROT_READ | libc::PROT_EXEC);
        }

        Self { jump_target, proxy }
    }
}

impl Drop for SdlHook {
    fn drop(&mut self) {
        unsafe {
            let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
            let addr = (self.jump_target as usize & !(page_size - 1)) as *mut c_void;
            let prot = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

            libc::mprotect(addr, page_size, prot);
            *self.jump_target = self.proxy;
            libc::mprotect(addr, page_size, libc::PROT_READ | libc::PROT_EXEC);
        }
    }
}
