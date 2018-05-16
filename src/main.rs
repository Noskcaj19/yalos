#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(global_allocator, allocator_api, alloc)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_macros))]
#![no_std]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate once;

#[macro_use]
extern crate lazy_static;
extern crate linked_list_allocator;
extern crate os_bootinfo;
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
mod time;
mod util;

use arch::memory::heap::HeapAllocator;

#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();

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
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    panic!("Out of memory");
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    println!("{:^80}", "YALOS 0.0.3");

    arch::init(boot_info_address);

    alloc::boxed::Box::new(5);

    loop {
        util::halt();
    }
}
