#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]

#[macro_use]
pub mod devices;
#[macro_use]
pub mod macros;
#[macro_use]
pub mod arch;
pub mod drivers;
mod time;
pub mod util;

//use arch::memory::heap::HeapAllocator;

//#[cfg_attr(not(test), global_allocator)]
//static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
