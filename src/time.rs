use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// Kernel up time, in (second, nanosecond)
    pub static ref OFFSET: Mutex<(u64, u64)> = Mutex::new((0, 0));
}

#[allow(dead_code)]
pub fn monotonic() -> (u64, u64) {
    *OFFSET.lock()
}
