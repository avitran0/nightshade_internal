use utils::sync::Mutex;
fn main() {
    let m = Mutex::new(0);
    if let Some(guard) = m.try_lock() {}
}
