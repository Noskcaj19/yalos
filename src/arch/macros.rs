/// Generate a software interrupt.
/// This is a macro because the argument needs to be an immediate.
#[macro_export]
macro_rules! int {
    ($x:expr) => {{
        unsafe {
            asm!("int $0" :: "N" ($x));
        }
    }};
}

/// Creates a x86-interrupt functio
macro_rules! interrupt {
    ($name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            _: &mut ::x86_64::structures::idt::ExceptionStackFrame,
        ) {
            pub fn inner() {
                $func
            }

            inner();
        }
    };
}

/// Creates a x86-interrupt function with access to the stack frame
macro_rules! interrupt_stack {
    ($name:ident, $stack:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            __stack: &mut ::x86_64::structures::idt::ExceptionStackFrame,
        ) {
            pub fn inner($stack: &mut ::x86_64::structures::idt::ExceptionStackFrame) {
                $func
            }

            inner(__stack);
        }
    };
}

/// Creates a x86-interrupt function with access to the stack frame and error code
macro_rules! interrupt_stack_error {
    ($name:ident, $stack:ident, $error_code:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            __stack: &mut ::x86_64::structures::idt::ExceptionStackFrame,
            __error_code: u64,
        ) {
            pub fn inner(
                $stack: &mut ::x86_64::structures::idt::ExceptionStackFrame,
                $error_code: u64,
            ) {
                $func
            }

            inner(__stack, __error_code);
        }
    };
}

/// Creates a x86-interrupt function with access to the stack frame and error code
macro_rules! page_fault {
    ($name:ident, $stack:ident, $error_code:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            __stack: &mut ::x86_64::structures::idt::ExceptionStackFrame,
            __error_code: ::x86_64::structures::idt::PageFaultErrorCode,
        ) {
            pub fn inner(
                $stack: &mut ::x86_64::structures::idt::ExceptionStackFrame,
                $error_code: ::x86_64::structures::idt::PageFaultErrorCode,
            ) {
                $func
            }

            inner(__stack, __error_code);
        }
    };
}
