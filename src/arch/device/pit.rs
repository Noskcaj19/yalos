use io::{Io, Port};

lazy_static! {
    pub static ref CHAN0: Port<u8> = Port::new(0x40);
    pub static ref COMMAND: Port<u8> = Port::new(0x43);
}

static SELECT_CHAN0: u8 = 0;
// Access both low and high bits
static LOHI: u8 = 0x30;

static CHAN0_DIVISOR: u16 = 2685;

pub fn init() {
    // TODO: Crashing on stdio serial
    // COMMAND.write(SELECT_CHAN0 | LOHI | 5);
    // CHAN0.write((CHAN0_DIVISOR & 0xFF) as u8);
    // CHAN0.write((CHAN0_DIVISOR >> 8) as u8);
}
