/// Macro for printing to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::drivers::vga_buffer::print(format_args!($($arg)*));
        $crate::devices::serial::print(format_args!($($arg)*));
    }}
}

/// Macro for printing to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*))
}
