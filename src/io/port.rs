#![allow(dead_code)]
use core::marker::PhantomData;

use super::io::Io;

#[derive(Debug, Copy, Clone)]
pub struct Port<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> Port<T> {
    /// Create a PIO from a given port
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            value: PhantomData,
        }
    }
}

/// IO for bytes
impl Io for Port<u8> {
    type Value = u8;

    /// Read
    #[inline(always)]
    fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    /// Write
    #[inline(always)]
    fn write(&self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

/// IO for words
impl Io for Port<u16> {
    type Value = u16;

    /// Read
    #[inline(always)]
    fn read(&self) -> u16 {
        let value: u16;
        unsafe {
            asm!("in $0, $1" : "={ax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    /// Write
    #[inline(always)]
    fn write(&self, value: u16) {
        unsafe {
            asm!("out $1, $0" : : "{ax}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}

/// IO for doublewords
impl Io for Port<u32> {
    type Value = u32;

    /// Read
    #[inline(always)]
    fn read(&self) -> u32 {
        let value: u32;
        unsafe {
            asm!("in $0, $1" : "={eax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    /// Write
    #[inline(always)]
    fn write(&self, value: u32) {
        unsafe {
            asm!("out $1, $0" : : "{eax}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}
