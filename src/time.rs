use spin::Mutex;

lazy_static! {
    /// Kernel up time, in (second, nanosecond)
    pub static ref OFFSET: Mutex<(u64, u64)> = Mutex::new((0, 0));
}

pub fn monotomic() -> (u64, u64) {
    *OFFSET.lock()
}
