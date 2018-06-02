use os_bootinfo::BootInfo;
use x86_64::structures::paging::{
    MapToError, Mapper, Page, PageTableFlags, PhysFrame, RecursivePageTable, Size4KB,
};
use x86_64::VirtAddr;

pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::stack_allocator::Stack;

mod area_frame_allocator;
pub mod heap;
mod stack_allocator;

pub fn init<'a>(
    boot_info: &BootInfo,
    mut rec_page_table: RecursivePageTable<'a>,
) -> MemoryController<'a> {
    assert_has_not_been_called!("memory init must be called only once");

    let mut frame_allocator = AreaFrameAllocator::new(&boot_info.memory_map);

    use self::heap::{HEAP_SIZE, HEAP_START};

    let heap_start_page = Page::containing_address(VirtAddr::new(HEAP_START));
    let heap_end_page = Page::containing_address(VirtAddr::new(HEAP_START + HEAP_SIZE - 1));

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        map_page(page, flags, &mut rec_page_table, &mut frame_allocator)
            .expect("Heap page mapping failed");
    }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        page_table: rec_page_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}

pub fn map_page<'a>(
    page: Page<Size4KB>,
    flags: PageTableFlags,
    page_table: &mut RecursivePageTable<'a>,
    frame_allocator: &mut FrameAllocator,
) -> Result<(), MapToError> {
    let frame = frame_allocator
        .allocate_frame()
        .expect("OOM - Cannot allocate frame");

    page_table
        .map_to(page, frame, flags, &mut || frame_allocator.allocate_frame())?
        .flush();

    Ok(())
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame>;
    fn deallocate_frame(&mut self, frame: PhysFrame);
}

#[allow(dead_code)]
pub struct MemoryController<'a> {
    page_table: RecursivePageTable<'a>,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl<'a> MemoryController<'a> {
    #[allow(dead_code)]
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut page_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(page_table, frame_allocator, size_in_pages)
    }
}

#[cfg(test)]
mod tests {

    // TODO: Renable and catch stack overflows
    // #[test]
    // #[should_panic]
    // // Stack overflow test that could corrupt memory below stack
    // // Issue: Use stack probes to check required stack pages before function
    // // Tracking: https://github.com/rust-lang/rust/issues/16012
    // fn stack_overflow() {
    //     let x = [0; 99999];
    // }
}
