#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    msg: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    column: u32,
) -> ! {
    println!("Panic at {}:{}, {}", file, line, msg);
    loop {}
}

#[no_mangle]
pub fn _start() -> ! {
    print!("Hello");
    println!(", some numbers: {} \"{}\"", 42, 1.337);

    loop {}
}
