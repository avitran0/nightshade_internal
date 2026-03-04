pub mod pattern;

pub struct Hook {}

impl Hook {
    pub fn new(_address: usize) -> Self {
        Self {}
    }

    pub fn unhook(&self) {}
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.unhook();
    }
}
