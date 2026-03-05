use libc::{c_int, c_void};

use crate::interface::Interface;

pub struct EngineInterface {
    interface: Interface,
}

impl EngineInterface {
    pub fn new(interface: Interface) -> Self {
        Self { interface }
    }

    pub fn screen_size(&self) -> (i32, i32) {
        type ScreenSizeFn = extern "C" fn(*const c_void, *mut c_int, *mut c_int);
        let mut width = 0;
        let mut height = 0;
        let func: ScreenSizeFn = self.interface.vfunc(5);
        func(self.interface.handle(), &mut width, &mut height);
        (width, height)
    }
}
