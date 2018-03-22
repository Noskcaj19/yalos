#![allow(dead_code)]

/// Read a `u8` from `port`
pub fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe { asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile") }
    result
}

/// Write a `u8` to `port`
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("outb %al, %dx" :: "{al}"(data), "{dx}"(port) :: "volatile");
    }
}

/// Read a `u16` from `port`
pub fn inw(port: u16) -> u16 {
    let result: u16;
    unsafe { asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile") }
    result
}

/// Write a `u16` to `port`
pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("outw %ax, %dx" :: "{ax}"(data), "{dx}"(port) :: "volatile");
    }
}

/// Read a `u32` from `port`
pub fn inl(port: u16) -> u32 {
    let result: u32;
    unsafe { asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile") }
    result
}

/// Write a `u32` to `port`
pub fn outl(port: u16, data: u32) {
    unsafe {
        asm!("outl %eax, %dx" :: "{eax}"(data), "{dx}"(port) :: "volatile");
    }
}
