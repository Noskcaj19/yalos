use super::PICS;
use crate::time;
use x86_64::instructions::port::Port;

interrupt!(pit, {
    const PIT_RATE: u64 = 2_250_286;

    let mut offset = time::OFFSET.lock();
    let sum = offset.1 + PIT_RATE;
    offset.1 = sum % 1_000_000_000;
    offset.0 += sum / 1_000_000_000;

    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
});

pub static KEYBOARD: Port<u8> = Port::new(0x60);

interrupt!(keyboard, {
    crate::drivers::keyboard::key_handler();

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
});
