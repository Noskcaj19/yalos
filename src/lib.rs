#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(global_allocator, allocator_api, alloc)]
#![feature(panic_implementation, panic_info_message)]
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
#[macro_use]
extern crate bitflags;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
pub mod io;
#[macro_use]
pub mod arch;
pub mod drivers;
mod time;
pub mod util;

use arch::memory::heap::HeapAllocator;

#[cfg_attr(not(test), global_allocator)]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
