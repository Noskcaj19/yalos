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
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*))
}
