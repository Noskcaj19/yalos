use arch::device::{pic, port};

interrupt!(pit, {
    pic::eoi(32);
});

interrupt!(keyboard, {
    let data = port::inb(0x60);

    println!("{}", data);

    ::arch::device::pic::eoi(33);
});
