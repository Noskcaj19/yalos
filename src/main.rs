#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(global_allocator, allocator_api, alloc)]
#![feature(panic_implementation, panic_info_message)]
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
#[macro_use]
extern crate bitflags;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
mod io;
#[macro_use]
mod arch;
mod drivers;
mod time;
mod util;

use arch::memory::heap::HeapAllocator;

#[cfg_attr(not(test), global_allocator)]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();

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
    arch::init(boot_info_address);

    println!("{:^80}", "YALOS 0.0.4");

    alloc::boxed::Box::new(5);

    loop {
        util::halt();
    }
}
