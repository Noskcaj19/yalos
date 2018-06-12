#![feature(lang_items)]
#![feature(asm)]
#![feature(alloc)]
#![feature(panic_implementation, panic_info_message)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_macros))]
#![no_std]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate yalos;

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic_impl(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("Panic at {}:{}, {}", location.file(), location.line(), msg);
    } else {
        if let Some(fmt_args) = info.message() {
            println!(
                "Panic at {}:{}, {}",
                location.file(),
                location.line(),
                fmt_args
            );
        } else {
            println!("Panic at {}:{}", location.file(), location.line());
        }
    };

    loop {
        yalos::util::halt();
    }
}

#[cfg(not(test))]
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    panic!("Out of memory");
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    yalos::arch::init(boot_info_address);

    println!("{:^80}", "YALOS 0.0.4");

    alloc::boxed::Box::new(5);

    loop {
        yalos::util::halt();
    }
}
