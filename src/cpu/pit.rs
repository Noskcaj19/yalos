use x86_64::structures::idt::ExceptionStackFrame;

pub extern "x86-interrupt" fn handler(_: &mut ExceptionStackFrame) {
    // Send EOI
    ::cpu::pic::eoi(32);
}
