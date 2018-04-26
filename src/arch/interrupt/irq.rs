use arch::device::pic;
use io::{Io, Port};

interrupt!(pit, {
    pic::MASTER.ack();
});

pub static mut KEYBOARD: Port<u8> = Port::new(0x60);

interrupt!(keyboard, {
    let data = KEYBOARD.read();

    println!("{}", data);

    pic::MASTER.ack();
});
