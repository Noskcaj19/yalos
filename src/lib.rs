#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(global_allocator, allocator_api, alloc)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_macros))]
#![no_std]

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
pub mod drivers;
mod io;
mod time;
pub mod util;

use arch::memory::heap::HeapAllocator;

#[cfg_attr(not(test), global_allocator)]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();

/// The main kernel entry point
pub fn kstart(boot_info_address: usize) -> ! {
    println!("{:^80}", "YALOS 0.0.3");

    arch::init(boot_info_address);

    alloc::boxed::Box::new(5);

    loop {
        util::halt();
    }
}
