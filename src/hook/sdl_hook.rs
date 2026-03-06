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

        unsafe {
            *jump_target = hook;
        }

        Self { jump_target, proxy }
    }
}

impl Drop for SdlHook {
    fn drop(&mut self) {
        unsafe {
            *self.jump_target = self.proxy;
        }
    }
}
