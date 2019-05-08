#![feature(lang_items)]
#![feature(asm)]
#![feature(panic_info_message)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_macros))]
#![feature(alloc_error_handler)]
#![no_std]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate yalos;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    yalos::arch::init(boot_info);

    println!("{:^80}", "YALOS 0.0.4");

    //    alloc::boxed::Box::new(5);

    loop {
        yalos::util::halt();
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
pub fn rust_oom(o: core::alloc::Layout) -> ! {
    panic!("Out of memory: {} bytes requested", o.size());
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("Panic at {}:{}, {}", location.file(), location.line(), msg);
    } else if let Some(fmt_args) = info.message() {
        println!(
            "Panic at {}:{}, {}",
            location.file(),
            location.line(),
            fmt_args
        );
    } else {
        println!("Panic at {}:{}", location.file(), location.line());
    };

    loop {
        yalos::util::halt();
    }
}
