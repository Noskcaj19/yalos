use log::error;

interrupt_stack!(divide_by_zero, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Divide by zero error");
});

interrupt_stack!(debug, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Debug trap");
});

interrupt_stack!(non_maskable_interrupt, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Non-maskable interrupts");
});

interrupt_stack!(breakpoint, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Breakpoint trap");
});

interrupt_stack!(overflow, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Overflow trap");
});

interrupt_stack!(bound_range_exceeded, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Bound range exceeded fault");
});

interrupt_stack!(invalid_opcode, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Invalid opcode fault");
});

interrupt_stack!(device_not_available, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Device not available fault");
});

interrupt_stack_error!(double_fault, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Double fault- code: {}", error);
});

interrupt_stack_error!(invalid_tss, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Invalid TSS fault - code: {}", error);
});

interrupt_stack_error!(segment_not_present, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Segment not present fault - code: {}", error);
});

interrupt_stack_error!(stack_segment_fault, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Stack segment fault - code: {}", error);
});

interrupt_stack_error!(general_protection_fault, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Protection fault - code: {}", error);
});

page_fault!(page_fault, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Page fault - code: {:>016X}", error.bits());
});

interrupt_stack!(x87_floating_point, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("x87 Floating point fault");
});

interrupt_stack_error!(alignment_check, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Alignment check fault - code: {}", error);
});

interrupt_stack!(machine_check, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Machine check fault");
});

interrupt_stack!(simd_floating_point, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("SIMD floating point fault");
});

interrupt_stack!(virtualization, stack, {
    error!("Stack frame: {:#?}", stack);
    error!("Virtualization fault");
});

interrupt_stack_error!(security_exception, stack, error, {
    error!("Stack frame: {:#?}", stack);
    error!("Security exception - code: {}", error);
});
