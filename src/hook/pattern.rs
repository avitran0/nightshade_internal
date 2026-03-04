pub struct Pattern {
    pattern: String,
}

impl Pattern {
    pub const fn new(pattern: &str) -> Self {
        Self {
            pattern: String::new(),
        }
    }

    pub fn eval() {}
}
