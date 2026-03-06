#[allow(dead_code)]
pub struct Pattern {
    bytes: Vec<Option<u8>>,
}

#[allow(dead_code)]
impl Pattern {
    pub fn new(pattern: &str) -> Self {
        let mut bytes = Vec::new();
        for byte_str in pattern.split_whitespace() {
            if byte_str == "?" || byte_str == "??" {
                bytes.push(None);
            } else if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                bytes.push(Some(byte));
            }
        }
        Self { bytes }
    }

    pub fn scan(&self, data: &[u8]) -> Option<usize> {
        if self.bytes.is_empty() || data.len() < self.bytes.len() {
            return None;
        }

        for i in 0..=(data.len() - self.bytes.len()) {
            let mut found = true;
            for (j, byte_opt) in self.bytes.iter().enumerate() {
                if let Some(byte) = byte_opt
                    && data[i + j] != *byte
                {
                    found = false;
                    break;
                }
            }
            if found {
                return Some(i);
            }
        }
        None
    }
}
