use lazy_static::lazy_static;

use x86_64::instructions::port::Port;

lazy_static! {
    pub static ref CHAN0: Port<u8> = Port::new(0x40);
    pub static ref COMMAND: Port<u8> = Port::new(0x43);
}

#[allow(dead_code)]
static SELECT_CHAN0: u8 = 0;
// Access both low and high bits
#[allow(dead_code)]
static LOHI: u8 = 0x30;

#[allow(dead_code)]
static CHAN0_DIVISOR: u16 = 2685;

pub fn init() {
    // TODO: Crashing on stdio serial
    // COMMAND.write(SELECT_CHAN0 | LOHI | 5);
    // CHAN0.write((CHAN0_DIVISOR & 0xFF) as u8);
    // CHAN0.write((CHAN0_DIVISOR >> 8) as u8);
}
