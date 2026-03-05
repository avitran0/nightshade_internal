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

    pub fn local_player_index(&self) -> i32 {
        type LocalPlayerIndexFn = extern "C" fn(*const c_void) -> i32;
        let func: LocalPlayerIndexFn = self.interface.vfunc(12);
        func(self.interface.handle())
    }

    /// player is in game, not loading
    pub fn in_game(&self) -> bool {
        type InGameFn = extern "C" fn(*const c_void) -> bool;
        let func: InGameFn = self.interface.vfunc(26);
        func(self.interface.handle())
    }

    pub fn world_to_screen_matrix(&self) -> glam::Mat4 {
        type WorldToScreenMatrixFn = extern "C" fn(*const c_void) -> *const glam::Mat4;
        let func: WorldToScreenMatrixFn = self.interface.vfunc(37);
        unsafe { *func(self.interface.handle()) }
    }
}
