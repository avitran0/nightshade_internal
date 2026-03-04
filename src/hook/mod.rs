pub mod pattern;

pub struct Hook {}

impl Hook {
    pub fn new(address: usize) -> Self {

    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.unhook();
    }
}
