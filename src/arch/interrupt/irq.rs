use arch::device::pic;
use io::{Io, Port};
use time;

interrupt!(pit, {
    const PIT_RATE: u64 = 2_250_286;

    let mut offset = time::OFFSET.lock();
    let sum = offset.1 + PIT_RATE;
    offset.1 = sum % 1_000_000_000;
    offset.0 += sum / 1_000_000_000;

    pic::MASTER.ack();
});

pub static KEYBOARD: Port<u8> = Port::new(0x60);

interrupt!(keyboard, {
    let data = KEYBOARD.read();

    println!("{}", data);

    pic::MASTER.ack();
});
