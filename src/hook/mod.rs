pub mod pattern;

pub struct Hook {
    address: usize,
}

impl Hook {

}

impl Drop for Hook {
    fn drop(&mut self) {
    }
}
