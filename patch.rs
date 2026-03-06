use std::sync::atomic::{AtomicUsize, Ordering};

pub static ORIGINAL_FRAME_STAGE_NOTIFY: AtomicUsize = AtomicUsize::new(0);
