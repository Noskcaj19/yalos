interrupt_stack!(divide_by_zero, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Divide by zero error");
});

interrupt_stack!(debug, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Debug trap");
});

interrupt_stack!(non_maskable_interrupt, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Non-maskable interrupt");
});

interrupt_stack!(breakpoint, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Breakpoint trap");
});

interrupt_stack!(overflow, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Overflow trap");
});

interrupt_stack!(bound_range_exceeded, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Bound range exceeded fault");
});

interrupt_stack!(invalid_opcode, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Invalid opcode fault");
});

interrupt_stack!(device_not_available, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Device not available fault");
});

interrupt_stack_error!(double_fault, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Double fault- code: {}", error);
});

interrupt_stack_error!(invalid_tss, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Invalid TSS fault - code: {}", error);
});

interrupt_stack_error!(segment_not_present, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Segment not present fault - code: {}", error);
});

interrupt_stack_error!(stack_segment_fault, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Stack segment fault - code: {}", error);
});

interrupt_stack_error!(general_protection_fault, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Protection fault - code: {}", error);
});

page_fault!(page_fault, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Page fault - code: {:>016X}", error.bits());
});

interrupt_stack!(x87_floating_point, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("x87 Floating point fault");
});

interrupt_stack_error!(alignment_check, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Alignment check fault - code: {}", error);
});

interrupt_stack!(machine_check, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Machine check fault");
});

interrupt_stack!(simd_floating_point, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("SIMD floating point fault");
});

interrupt_stack!(virtualization, stack, {
    println!("Stack frame: {:#?}", stack);
    println!("Virtualization fault");
});

interrupt_stack_error!(security_exception, stack, error, {
    println!("Stack frame: {:#?}", stack);
    println!("Security exception - code: {}", error);
});
