use spin::Mutex;
use x86_64::structures::idt::{ExceptionStackFrame, Idt};

extern "x86-interrupt" fn empty_lower(_: &mut ExceptionStackFrame) {
    // Send EOI
    ::drivers::ports::outb(0x20, 0x20);
}

extern "x86-interrupt" fn empty_upper(_: &mut ExceptionStackFrame) {
    // Send EOI
    ::drivers::ports::outb(0xA0, 0x20);
    ::drivers::ports::outb(0x20, 0x20);
}

lazy_static! {
    pub static ref IDT: Mutex<Idt> = Mutex::new(Idt::new());
}

pub fn initalize() {
    let idt = Idt::new();
    // Add empty interrupt handlers to entire table
    let mut lock = IDT.lock();
    for i in 0..256 {
        // Skip special idt indices
        match i {
            8 | 10...14 | 15 | 17 | 21...29 | 30 | 31 => continue,
            0...40 => {
                lock[i].set_handler_fn(empty_lower);
            }
            41...256 => {
                lock[i].set_handler_fn(empty_lower);
            }
            _ => {}
        }
    }

    // Load idt using `lidt`
    load_idt(&lock);
}

pub fn load_idt(idt: &Idt) {
    use x86_64::instructions::tables::{lidt, DescriptorTablePointer};
    use core::mem::size_of;

    let ptr = DescriptorTablePointer {
        base: idt as *const _ as u64,
        limit: (size_of::<Idt>() - 1) as u16,
    };

    unsafe { lidt(&ptr) };
}
