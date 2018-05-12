#[macro_use]
pub mod macros;

pub mod device;
pub mod idt;
pub mod interrupt;
pub mod memory;

use os_bootinfo::BootInfo;

pub fn init(boot_info_address: usize) {
    use x86_64::structures::paging::{PageTable, RecursivePageTable};

    let boot_info: &BootInfo = unsafe { &*(boot_info_address as *mut BootInfo) };

    for i in boot_info.memory_map.iter() {
        println!(
            "{:#08x}.{:#08x} - {:?}",
            i.range.start_frame_number, i.range.end_frame_number, i.region_type
        );
    }

    let mut page_table: &mut PageTable =
        unsafe { &mut *(boot_info.p4_table_addr as *mut PageTable) };

    let rec_page_table =
        RecursivePageTable::new(&mut page_table).expect("recursive page table creation failed");

    let _memory_controller = memory::init(boot_info, rec_page_table);

    unsafe {
        use arch::memory::heap::{HEAP_SIZE, HEAP_START};
        ::HEAP_ALLOCATOR.init(HEAP_START as usize, HEAP_SIZE as usize);
    }

    device::pic::init();

    idt::initalize();
}
