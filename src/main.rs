#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
mod arch;
mod drivers;
mod util;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    msg: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    _column: u32,
) -> ! {
    println!("Panic at {}:{}, {}", file, line, msg);
    loop {
        util::halt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{:^80}", "YALOS 0.0.2");

    arch::device::pic::remap();

    arch::idt::initalize();

    loop {
        util::halt();
    }
}
