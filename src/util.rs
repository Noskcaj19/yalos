/// Called while spinning (name borrowed from Linux). Can be implemented to call
/// a platform-specific method of lightening CPU load in spinlocks.
/// Borrowed from https://github.com/mvdnes/spin-rs/blob/master/src/util.rs
#[inline(always)]
pub fn cpu_relax() {
    // This instruction is meant for usage in spinlock loops
    // (see Intel x86 manual, III, 4.2)
    unsafe {
        asm!("pause" :::: "volatile");
    }
}
