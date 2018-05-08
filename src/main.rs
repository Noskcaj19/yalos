#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_macros))]
#![no_std]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate lazy_static;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
mod arch;
mod drivers;
mod io;
mod util;

#[cfg(not(test))]
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

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{:^80}", "YALOS 0.0.2");

    arch::device::pic::init();

    arch::idt::initalize();

    loop {
        util::halt();
    }
}
