/// Macro for printing to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga_buffer::print(format_args!($($arg)*)))
}

/// Macro for printing to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*))
}

/// Generate a software interrupt.
/// This is a macro because the argument needs to be an immediate.
#[macro_export]
macro_rules! int {
    ($x: expr) => {{
        unsafe {
            asm!("int $0" :: "N" ($x));
        }
    }};
}
