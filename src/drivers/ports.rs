/// Read data from `port`
pub fn byte_in(port: usize) -> u8 {
    let result: u8;
    unsafe { asm!("in %dx, %al" : "={al}"(result) : "{dx}"(port)) }
    result
}

/// Write `data` to `port`
pub fn byte_out(port: usize, data: u8) {
    unsafe {
        asm!("out %al, %dx" :: "{al}"(data), "{dx}"(port));
    }
}
