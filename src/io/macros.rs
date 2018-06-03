/// Macro for printing to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::{io, drivers};
        drivers::vga_buffer::print(format_args!($($arg)*));
        io::serial::print(format_args!($($arg)*));
    }}
}

/// Macro for printing to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*))
}
